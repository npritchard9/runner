// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::NaiveTime;
use surrealdb::{engine::local::Db, Surreal};
use tauri::State;

use runner::{
    db::*,
    models::{run::*, user::*},
};

pub struct AppState(pub Surreal<Db>);

#[tauri::command]
async fn save_run(distance: u8, time: NaiveTime, state: State<'_, AppState>) -> Result<(), ()> {
    let run = Run { distance, time };
    let _r = insert_run(run, &state.0).await.unwrap();
    Ok(())
}

#[tauri::command]
async fn get_runs(state: State<'_, AppState>) -> Result<Vec<DBRun>, ()> {
    let runs = get_all_runs(&state.0).await.unwrap();
    Ok(runs)
}

#[tauri::command]
async fn get_weekly_runs(state: State<'_, AppState>) -> Result<Vec<DBRun>, ()> {
    let runs = get_weekly_stats(&state.0).await.unwrap();
    dbg!(&runs);
    Ok(runs)
}

fn main() {
    let db = tauri::async_runtime::block_on(get_db()).unwrap();
    tauri::Builder::default()
        .manage(AppState(db))
        .invoke_handler(tauri::generate_handler![
            save_run,
            get_runs,
            get_weekly_runs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
