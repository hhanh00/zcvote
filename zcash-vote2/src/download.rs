use prost::Message;
use sqlx::SqliteConnection;
use tonic::Request;

use crate::{
    Client, IntoAnyhow, ProgressReporter, VoteResult,
    lwd_prc::{BlockId, BlockRange},
};

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
    progress_reporter: PR,
) -> VoteResult<()> {
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

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
    use tonic::transport::{Channel, ClientTlsConfig};

    use crate::{db::create_db, Client};

    #[tokio::test]
    async fn test_download() {
        let tls_config = ClientTlsConfig::new().with_enabled_roots();
        let channel = Channel::from_static("https://zec.rocks")
            .tls_config(tls_config)
            .unwrap();
        let (tx, mut rx) = mpsc::channel::<String>(1);
        tokio::spawn(async move {
            let mut client = Client::connect(channel).await.unwrap();
            let options = SqliteConnectOptions::new().filename("zcvote.db");
            let pool = SqlitePool::connect_with(options).await.unwrap();
            let mut connection = pool.acquire().await.unwrap();
            create_db(&mut connection).await.unwrap();
            super::download_blocks(&mut client, &mut connection, 3_000_000, 3_010_000, tx).await.unwrap();
        });
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    }
}
