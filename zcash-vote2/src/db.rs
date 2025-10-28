use crate::{data::Election, seed::generate_seed, tiu, VoteResult};
use halo2_proofs::pasta::{group::ff::PrimeField, Fp};
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
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS blocks(
        height INTEGER PRIMARY KEY,
        data BLOB NOT NULL)",
    )
    .execute(&mut *connection)
    .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS actions(
        id_action INTEGER PRIMARY KEY,
        height INTEGER NOT NULL,
        idx INTEGER NOT NULL,
        nf BLOB NOT NULL,
        cmx BLOB NOT NULL)",
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

pub async fn store_election(connection: &mut SqliteConnection, election: Election) -> VoteResult<()> {
    sqlx::query(
        "UPDATE election_defs SET definition = ?2 WHERE name = ?1")
    .bind(&election.name)
    .bind(serde_json::to_string(&election).unwrap())
    .execute(connection)
    .await?;
    Ok(())
}

pub async fn list_nfs(connection: &mut SqliteConnection, start: u32, end: u32) -> VoteResult<Vec<Fp>> {
    let r= sqlx::query("SELECT nf FROM actions WHERE height >= ?1 AND height <= ?2 ORDER BY nf")
    .bind(start)
    .bind(end)
    .map(|r: SqliteRow| {
        let mut nf: Vec<u8> = r.get(0);
        nf.reverse();
        Fp::from_repr(tiu!(nf)).unwrap()
    })
    .fetch_all(connection)
    .await?;
    Ok(r)
}

pub async fn list_cmxs(connection: &mut SqliteConnection, start: u32, end: u32) -> VoteResult<Vec<Fp>> {
    let r= sqlx::query("SELECT cmx FROM actions WHERE height >= ?1 AND height <= ?2 ORDER BY height, idx")
    .bind(start)
    .bind(end)
    .map(|r: SqliteRow| {
        let cmx: Vec<u8> = r.get(0);
        Fp::from_repr(tiu!(cmx)).unwrap()
    })
    .fetch_all(connection)
    .await?;
    Ok(r)
}

pub async fn get(connection: &mut SqliteConnection) -> VoteResult<u32> {
    let r = sqlx::query("INSERT INTO test(v) VALUES (?1)")
        .bind(1)
        .execute(&mut *connection)
        .await?;
    let res = r.rows_affected() as u32;
    Ok(res)
}
