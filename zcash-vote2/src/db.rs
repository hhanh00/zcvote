use crate::{data::Election, seed::generate_seed, VoteResult};
use sqlx::{sqlite::SqliteRow, SqliteConnection, Row};

pub async fn create_db(connection: &mut SqliteConnection) -> VoteResult<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS election_defs(
        id_election INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        seed TEXT,
        definition TEXT NOT NULL,
        UNIQUE (name))",
    )
    .execute(&mut *connection)
    .await?;
    Ok(())
}

pub async fn list_election_defs(connection: &mut SqliteConnection) -> VoteResult<Vec<Election>> {
    let elections = sqlx::query("SELECT seed, definition FROM election_defs ORDER BY name")
    .map(|r: SqliteRow| {
        let seed: String = r.get(0);
        let def: String = r.get(1);
        let mut def: Election = serde_json::from_str(&def).expect("Invalid election definition");
        def.seed = Some(seed);
        def
    })
    .fetch_all(&mut *connection)
    .await?;
    Ok(elections)
}

pub async fn new_election(connection: &mut SqliteConnection, name: String) -> VoteResult<Election> {
    let seed = generate_seed()?;
    let mut election = Election {
        name: name.clone(),
        ..Default::default()
    };
    // Do not store the seed with the definition in the db
    sqlx::query("INSERT INTO election_defs(name, seed, definition)
        VALUES (?1, ?2, ?3)")
    .bind(&name)
    .bind(&seed)
    .bind(serde_json::to_string(&election).unwrap())
    .execute(connection)
    .await?;
    election.seed = Some(seed);
    Ok(election)
}

pub async fn save_election(connection: &mut SqliteConnection, mut election: Election) -> VoteResult<()> {
    // Saving erases the cmx/nf/frontier since we are not sure that they are still valid
    election.cmx = None;
    election.nf = None;
    election.cmx_frontier = None;

    sqlx::query(
        "UPDATE election_defs SET definition = ?2 WHERE name = ?1")
    .bind(&election.name)
    .bind(serde_json::to_string(&election).unwrap())
    .execute(connection)
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
