use crate::VoteResult;
use sqlx::SqliteConnection;

pub async fn create_db(connection: &mut SqliteConnection) -> VoteResult<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS test(
        v INTEGER NOT NULL)",
    )
    .execute(&mut *connection)
    .await?;
    Ok(())
}

pub async fn get(connection: &mut SqliteConnection) -> VoteResult<u32> {
    let r = sqlx::query("INSERT INTO test(v) VALUES (?1)")
        .bind(1)
        .execute(&mut *connection)
        .await?;
    let res = r.rows_affected() as u32;
    Ok(res)
}
