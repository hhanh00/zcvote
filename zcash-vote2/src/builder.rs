use std::{cell::LazyCell, sync::LazyLock};

use orchard::{
    Address, Note,
    keys::{Diversifier, FullViewingKey, SpendingKey},
    note::{RandomSeed, Rho},
    value::NoteValue,
    vote::{Ballot, Circuit, ProvingKey, VerifyingKey},
};
use pasta_curves::{Fp, group::ff::PrimeField};
use prost::Message;
use rand_core::OsRng;
use sqlx::{Row, SqliteConnection, sqlite::SqliteRow};

use crate::{IntoAnyhow, VoteResult, db::{list_cmxs, list_nfs}, legacy::LegacyElection, pb, sync::ReceivedNote, tiu, trees::make_nfs_ranges};

pub async fn vote(
    connection: &mut SqliteConnection,
    election: LegacyElection,
    address: Address,
    value: u64,
    sk: &SpendingKey,
) -> VoteResult<Ballot> {
    let fvk = FullViewingKey::from(sk);
    let inputs = sqlx::query(
        "SELECT position, rho, diversifier, rseed, value
                FROM notes ORDER BY position",
    )
    .map(|r: SqliteRow| {
        let position: u32 = r.get(0);
        let rho: Vec<u8> = r.get(1);
        let diversifier: Vec<u8> = r.get(2);
        let rseed: Vec<u8> = r.get(3);
        let value: u64 = r.get(4);
        let d = Diversifier::from_bytes(tiu!(diversifier));
        let recipient = fvk.address(d, orchard::keys::Scope::External);
        let value = NoteValue::from_raw(value);
        let rho = Rho::from_bytes(&tiu!(rho)).unwrap();
        let rseed = RandomSeed::from_bytes(tiu!(rseed), &rho).unwrap();
        let note = Note::from_parts(recipient, value, rho, rseed).unwrap();
        (note, position)
    })
    .fetch_all(&mut *connection)
    .await?;

    let candidates = election
        .candidates
        .iter()
        .map(|c| crate::pb::Candidate {
            address: c.address.clone(),
            choice: c.choice.clone(),
        })
        .collect::<Vec<_>>();
    let election = pb::Election {
        name: election.name,
        start_height: election.start_height,
        end_height: election.end_height,
        question: election.question,
        candidates,
        signature_required: election.signature_required,
    };
    let election_bytes = election.encode_to_vec();
    let domain = orchard::vote::calculate_domain(&election_bytes);

    let mut nfs = list_nfs(connection, election.start_height, election.end_height).await?;
    make_nfs_ranges(&mut nfs);
    let cmxs = list_cmxs(connection, election.start_height, election.end_height).await?;

    let ballot = orchard::vote::vote(
        domain,
        election.signature_required,
        Some(*sk),
        &fvk,
        address,
        value,
        &inputs,
        &nfs,
        &cmxs,
        OsRng,
        &PROVING_KEY,
        &VERIFYING_KEY,
    )
    .anyhow()?;
    Ok(ballot)
}

pub static PROVING_KEY: LazyLock<ProvingKey<Circuit>> = LazyLock::new(|| ProvingKey::build());
pub static VERIFYING_KEY: LazyLock<VerifyingKey<Circuit>> = LazyLock::new(|| VerifyingKey::build());
