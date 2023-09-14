use chrono::Utc;
use surrealdb::engine::local::{Db, SpeeDb};
use surrealdb::sql::Value;
use surrealdb::Surreal;

use super::models::{run::*, user::*};
// set up authentication

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
    Ok(runs)
}
