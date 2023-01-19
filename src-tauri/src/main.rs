#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use sqlite::{State};
use std::fs::create_dir_all;
use std::path::Path;
use tauri::api::path::app_data_dir;

fn init_db(){
    let dir = app_data_dir(&Default::default()).unwrap();
    let full_dir = dir.to_str().unwrap().to_owned()+"tuip123-comic\\";
    let p = Path::new(&full_dir);
    match create_dir_all(p){
        Ok(_f)=>{},
        Err(err)=>{
            println!("{:?}",err)
        }
    }

    let conn = sqlite::open(full_dir.to_owned()+"test.db").unwrap();

    let mut query = "SELECT name FROM sqlite_master WHERE type='table' AND name='comic' COLLATE NOCASE";
    let mut statement = conn
        .prepare(query)
        .unwrap();
    let mut b = false;
    while let State::Row = statement.next().unwrap() {
        b = true;
    }
    if !b {
        query = "CREATE TABLE comic (Id INTEGER,path TEXT,cover TEXT,count INTEGER,libraryId INTEGER)";
        conn.execute(query).unwrap();
    }

    query = "SELECT name FROM sqlite_master WHERE type='table' AND name='library' COLLATE NOCASE";
    statement = conn
        .prepare(query)
        .unwrap();
    b = false;
    while let State::Row = statement.next().unwrap() {
        b = true;
    }
    if !b {
        query = "CREATE TABLE library (Id INTEGER,root TEXT)";
        conn.execute(query).unwrap();
    }
}

#[tauri::command]
fn add_library(path:&str){
    println!("{}", path)
}

fn main() {
    init_db();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,add_library])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
