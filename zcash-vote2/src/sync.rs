use std::collections::HashMap;

use crate::{
    IntoAnyhow, ProgressReporter, VoteResult,
    lwd_prc::{CompactBlock, CompactOrchardAction},
    tiu,
};
use bip39::Mnemonic;
use futures_util::stream::StreamExt;
use orchard::{
    Address, Note,
    keys::{FullViewingKey, PreparedIncomingViewingKey, SpendingKey},
    note::{ExtractedNoteCommitment, Nullifier, Rho},
    note_encryption::{CompactAction, OrchardDomain},
};
use prost::Message;
use sqlx::{Row, SqliteConnection, sqlite::SqliteRow};
use zcash_note_encryption::{EphemeralKeyBytes, try_compact_note_decryption};
use zcash_protocol::consensus::{Network, NetworkConstants};

pub type Hash32 = [u8; 32];

pub struct ReceivedNote {
    height: u32,
    txid: Hash32,
    vout: u32,
    position: u32,
    note: Note,
    address: Address,
}

pub struct Spend {
    height: u32,
    txid: Hash32,
    vin: u32,
    nf: Hash32,
}

pub async fn scan<PR: ProgressReporter>(
    connection: &mut SqliteConnection,
    seed: &str,
    aindex: u32,
    start: u32,
    end: u32,
    progress_reporter: &PR,
) -> VoteResult<()> {
    let network = Network::MainNetwork;
    let sk = seed_to_sk(&network, seed, aindex)?;
    let fvk = FullViewingKey::from(&sk);
    let ivk = fvk.to_ivk(orchard::keys::Scope::External);
    let pivk = PreparedIncomingViewingKey::new(&ivk);

    let mut notes: HashMap<Hash32, ReceivedNote> = HashMap::new();
    let mut spends: Vec<Spend> = vec![];

    {
        let mut blocks = sqlx::query(
            "SELECT data FROM blocks WHERE height >= ?1 AND height <= ?2
            ORDER BY height",
        )
        .bind(start)
        .bind(end)
        .map(|r: SqliteRow| r.get::<Vec<u8>, _>(0))
        .fetch(&mut *connection);

        let mut i_action = 0;
        while let Some(Ok(b)) = blocks.next().await {
            let block = CompactBlock::decode(&*b).anyhow()?;
            let height = block.height;
            if height.is_multiple_of(10_000) {
                progress_reporter.submit(format!("Scanning Block #{height}")).await;
            }
            for tx in block.vtx.iter() {
                for (vout, a) in tx.actions.iter().enumerate() {
                    let CompactOrchardAction {
                        nullifier,
                        cmx,
                        ephemeral_key,
                        ciphertext,
                    } = a;

                    let nullifier: [u8; 32] = tiu!(nullifier.clone());
                    let rho = Nullifier::from_bytes(&nullifier).unwrap();
                    let domain = OrchardDomain::for_nullifier(Rho::from_nf_old(rho));
                    let ephemeral_key: [u8; 32] = tiu!(ephemeral_key.clone());
                    let action = CompactAction::from_parts(
                        rho,
                        ExtractedNoteCommitment::from_bytes(&tiu!(cmx.clone())).unwrap(),
                        EphemeralKeyBytes(ephemeral_key),
                        ciphertext.clone().try_into().unwrap(),
                    );

                    if notes.contains_key(&nullifier) {
                        spends.push(Spend {
                            height: block.height as u32,
                            txid: tiu!(tx.hash.clone()),
                            vin: vout as u32,
                            nf: nullifier,
                        });
                        progress_reporter.submit(format!("Spent nullifier {}", hex::encode(nullifier))).await;
                    }

                    if let Some((note, address)) =
                        try_compact_note_decryption(&domain, &pivk, &action)
                    {
                        tracing::info!("{note:?}");
                        let nf = note.nullifier(&fvk).to_bytes();

                        let rnote = ReceivedNote {
                            height: block.height as u32,
                            txid: tiu!(tx.hash.clone()),
                            vout: vout as u32,
                            position: i_action,
                            note,
                            address,
                        };
                        notes.insert(nf, rnote);
                        progress_reporter.submit(format!("Incoming Note {}", hex::encode(nf))).await;
                    }

                    i_action += 1;
                }
            }
        }
    }

    for (nf, note) in notes.iter() {
        tracing::info!("note nf {}", hex::encode(nf));
        sqlx::query(
            "INSERT INTO notes
            (height, position, txid, vout, nf, rho, diversifier, rseed, value)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT DO NOTHING",
        )
        .bind(note.height)
        .bind(note.position)
        .bind(&note.txid[..])
        .bind(note.vout)
        .bind(&nf[..])
        .bind(&note.note.rho().to_bytes()[..])
        .bind(&note.address.diversifier().as_array()[..])
        .bind(&note.note.rseed().as_bytes()[..])
        .bind(note.note.value().inner() as i64)
        .execute(&mut *connection)
        .await?;
    }
    for sp in spends.iter() {
        tracing::info!("spend nf {}", hex::encode(sp.nf));

        let (id_note, value): (u32, i64) = sqlx::query_as("SELECT id_note, value FROM notes WHERE nf = ?1")
        .bind(&sp.nf[..])
        .fetch_one(&mut *connection)
        .await?;

        sqlx::query(
            "INSERT INTO spends
            (id_note, height, txid, vin, value)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT DO NOTHING")
        .bind(id_note)
        .bind(sp.height)
        .bind(&sp.txid[..])
        .bind(sp.vin)
        .bind(-value)
        .execute(&mut *connection)
        .await?;
    }
    Ok(())
}

pub fn seed_to_sk(network: &Network, key: &str, aindex: u32) -> VoteResult<SpendingKey> {
    let m = Mnemonic::parse(key).anyhow()?;
    let seed = m.to_seed("");
    let spk = SpendingKey::from_zip32_seed(&seed, network.coin_type(), tiu!(aindex))
        .map_err(anyhow::Error::msg)?;
    Ok(spk)
}
