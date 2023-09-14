use chrono::Utc;
use serde::Serialize;
use surrealdb::engine::local::{Db, SpeeDb};
use surrealdb::opt::auth::{Jwt, Scope};
use surrealdb::sql::Value;
use surrealdb::Surreal;

use super::models::{run::*, user::*};

pub async fn get_db() -> surrealdb::Result<Surreal<Db>> {
    let db = Surreal::new::<SpeeDb>("/Users/noahpritchard/Documents/rust/runner/runs.db").await?;
    db.use_ns("my_ns").use_db("my_db").await?;
    Ok(db)
}

pub async fn insert_run(r: Run, db: &Surreal<Db>) -> surrealdb::Result<DBRun> {
    let id = vec![Value::from(Utc::now())];
    let run: Option<DBRun> = db.create(("run", id)).content(r).await?;
    Ok(run.unwrap())
}

pub async fn get_all_runs(db: &Surreal<Db>) -> surrealdb::Result<Vec<DBRun>> {
    let runs = db.select("run").await?;
    Ok(runs)
}

pub async fn get_weekly_stats(db: &Surreal<Db>) -> surrealdb::Result<Vec<DBRun>> {
    let mut res = db
        .query("select * from run:[time::now() - 1w]..[time::now()]")
        .await?;
    let runs = res.take(0)?;
    // let runs = db.select("run").range("time::now() - 1w".."time::now()").await?;
    Ok(runs)
}

#[derive(Serialize)]
pub struct Credentials<'a> {
    email: &'a str,
    pass: &'a str,
}

pub async fn sign_up_user(
    db: &Surreal<Db>,
    credentials: Credentials<'_>,
) -> surrealdb::Result<Jwt> {
    let token = db
        .signup(Scope {
            namespace: "my_ns",
            database: "my_db",
            scope: "user",
            params: credentials,
        })
        .await?;
    Ok(token)
}

pub async fn sign_in_user(
    db: &Surreal<Db>,
    credentials: Credentials<'_>,
) -> surrealdb::Result<Jwt> {
    let token = db
        .signin(Scope {
            namespace: "my_ns",
            database: "my_db",
            scope: "user",
            params: credentials,
        })
        .await?;
    Ok(token)
}

pub async fn auth_user(db: &Surreal<Db>, token: Jwt) -> anyhow::Result<()> {
    db.authenticate(token).await?;
    Ok(())
}
