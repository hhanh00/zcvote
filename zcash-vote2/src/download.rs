use prost::Message;
use sqlx::{Row, SqliteConnection, sqlite::SqliteRow};
use tonic::{
    Request,
    transport::{Channel, ClientTlsConfig},
};

use crate::{
    Client, IntoAnyhow, ProgressReporter, VoteResult,
    legacy::LegacyElection,
    lwd_prc::{BlockId, BlockRange, CompactBlock},
};

pub async fn connect(lwd: String) -> VoteResult<Client> {
    let tls_config = ClientTlsConfig::new().with_enabled_roots();
    let channel = Channel::from_shared(lwd)
        .anyhow()?
        .tls_config(tls_config)
        .unwrap();
    let client = Client::connect(channel).await.unwrap();
    Ok(client)
}

pub async fn clear_blocks(connection: &mut SqliteConnection) -> VoteResult<()> {
    sqlx::query("DELETE FROM blocks")
        .execute(connection)
        .await?;
    Ok(())
}

pub async fn download_blocks<PR: ProgressReporter>(
    client: &mut Client,
    connection: &mut SqliteConnection,
    start: u32,
    end: u32,
    progress_reporter: &PR,
) -> VoteResult<()> {
    let (c,): (u32,) =
        sqlx::query_as("SELECT COUNT(*) FROM blocks WHERE height >= ?1 AND height <= ?2")
            .bind(start)
            .bind(end)
            .fetch_one(&mut *connection)
            .await?;
    if c == end - start + 1 {
        return Ok(());
    }

    let mut blocks = client
        .get_block_range(Request::new(BlockRange {
            start: Some(BlockId {
                height: start as u64,
                hash: vec![],
            }),
            end: Some(BlockId {
                height: end as u64,
                hash: vec![],
            }),
            spam_filter_threshold: 0,
        }))
        .await
        .anyhow()?
        .into_inner();
    while let Some(block) = blocks.message().await.anyhow()? {
        let data = block.encode_to_vec();
        sqlx::query(
            "INSERT INTO blocks(height, data)
        VALUES (?1, ?2) ON CONFLICT DO UPDATE
        SET data = excluded.data",
        )
        .bind(block.height as u32)
        .bind(&data)
        .execute(&mut *connection)
        .await?;
        if block.height.is_multiple_of(10_000) {
            progress_reporter
                .submit(format!("Downloaded Block #{}", block.height))
                .await;
        }
    }
    progress_reporter
        .submit(format!("Download Completed at Block #{end}"))
        .await;
    Ok(())
}

pub async fn extract_commitments<PR: ProgressReporter>(
    connection: &mut SqliteConnection,
    start: u32,
    end: u32,
    progress_reporter: &PR,
) -> VoteResult<()> {
    let report_interval = (end - start + 1) / 20;
    sqlx::query("DELETE FROM actions WHERE height >= ?1 AND height <= ?2")
        .bind(start)
        .bind(end)
        .execute(&mut *connection)
        .await?;
    for h in start..=end {
        if (h - start).is_multiple_of(report_interval) {
            progress_reporter
                .submit(format!("Extracted block #{h}"))
                .await;
        }
        let block = sqlx::query("SELECT data FROM blocks WHERE height = ?1")
            .bind(h)
            .map(|r: SqliteRow| {
                let data: Vec<u8> = r.get(0);
                CompactBlock::decode(&*data).unwrap()
            })
            .fetch_one(&mut *connection)
            .await?;
        let mut idx = 0;
        for tx in block.vtx.iter() {
            for a in tx.actions.iter() {
                let mut nf = a.nullifier.clone();
                nf.reverse(); // nf are stored in BE so that we can use ORDER BY
                let cmx = &a.cmx;
                sqlx::query(
                    "INSERT INTO actions(height, idx, nf, cmx)
                VALUES (?1, ?2, ?3, ?4)",
                )
                .bind(block.height as u32)
                .bind(idx)
                .bind(&nf)
                .bind(cmx)
                .execute(&mut *connection)
                .await?;
                idx += 1;
            }
        }
    }
    Ok(())
}

pub async fn fetch_election(
    url: &str,
    id: &str,
) -> VoteResult<LegacyElection> {
    let election = reqwest::get(&format!("{url}/election/{id}"))
        .await
        .anyhow()?
        .json::<LegacyElection>()
        .await
        .anyhow()?;
    Ok(election)
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
    use tonic::transport::{Channel, ClientTlsConfig};

    use crate::{Client, db::create_db};

    #[tokio::test]
    async fn test_download() {
        let (tx, mut rx) = mpsc::channel::<String>(1);
        tokio::spawn(async move {
            let tls_config = ClientTlsConfig::new().with_enabled_roots();
            let channel = Channel::from_static("https://zec.rocks")
                .tls_config(tls_config)
                .unwrap();
            let mut client = Client::connect(channel).await.unwrap();
            let options = SqliteConnectOptions::new().filename("zcvote.db");
            let pool = SqlitePool::connect_with(options).await.unwrap();
            let mut connection = pool.acquire().await.unwrap();
            create_db(&mut connection).await.unwrap();
            super::download_blocks(&mut client, &mut connection, 2_200_000, 2_220_000, &tx)
                .await
                .unwrap();
        });
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    }

    #[tokio::test]
    pub async fn test_extract() {
        let options = SqliteConnectOptions::new().filename("zcvote.db");
        let pool = SqlitePool::connect_with(options).await.unwrap();
        let mut connection = pool.acquire().await.unwrap();
        create_db(&mut connection).await.unwrap();
        let (tx, mut rx) = mpsc::channel::<String>(1);
        tokio::spawn(async move {
            super::extract_commitments(&mut connection, 2_200_000, 2_210_000, &tx)
                .await
                .unwrap();
        });
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    }
}
