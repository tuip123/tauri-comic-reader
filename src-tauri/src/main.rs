#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use sqlite::{Connection, State};
use std::fs::create_dir_all;
use std::path::Path;
use tauri::api::path::app_data_dir;

fn get_conn() -> Connection {
    let dir = app_data_dir(&Default::default()).unwrap();
    let full_dir = dir.to_str().unwrap().to_owned() + "tuip123-comic\\";
    let p = Path::new(&full_dir);
    match create_dir_all(p) {
        Ok(_f) => {}
        Err(err) => {
            println!("{:?}", err)
        }
    }
    let conn = sqlite::open(full_dir.to_owned() + "test.db").unwrap();
    conn
}

fn init_db() {
    let conn = get_conn();

    let mut query = "SELECT name FROM sqlite_master WHERE type='table' AND name='comic' COLLATE NOCASE";
    let mut statement = conn
        .prepare(query)
        .unwrap();
    let mut b = false;
    while let State::Row = statement.next().unwrap() {
        b = true;
    }
    if !b {
        query = "CREATE TABLE comic (Id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,path TEXT,cover TEXT,count INTEGER,libraryId INTEGER)";
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
        query = "CREATE TABLE library (Id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,root TEXT)";
        conn.execute(query).unwrap();
    }
}
use std::fs;
#[tauri::command]
fn add_library(path: &str) -> bool {
    let conn = get_conn();
    // 查找有无记录
    let mut insert = conn.prepare("select Id from library where root = ? limit 1").unwrap();
    insert.bind(1,path).unwrap();
    let mut bool = true;
    while let State::Row = insert.next().unwrap(){
        bool = false;
        return bool;
    }
    // 添加库记录
    insert = conn.prepare("insert into library (root) values (?)").unwrap();
    insert.bind(1,path).unwrap();
    insert.next().unwrap();
    insert = conn.prepare("select Id from library where root = ? limit 1").unwrap();
    insert.bind(1,path).unwrap();
    let mut library_id = 0;
    while let State::Row = insert.next().unwrap(){
        library_id = insert.read::<i64>(0).unwrap();
    }
    // 添加库下漫画记录
    // todo 修改为使用 fn 刷新漫画列表
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        insert = conn.prepare("insert into comic (path,libraryId) values (?,?)").unwrap();
        insert.bind(1,path.unwrap().path().to_str()).unwrap();
        insert.bind(2, library_id).unwrap();
        insert.next().unwrap();
    }
    return bool;
}
// todo （根据库id）刷新漫画列表（重新检索path，和数据库中比对，仅path的为新增，仅数据库的为删除，移除删除记录，添加新增记录）
// todo 基于上述，添加count计算页数，添加cover获取封面
// 思路 两个map，第一个为path结果 第二个为数据库结果
// map1遍历，map2中查，有结果，两边删除 此时剩余的map1为新增，map2为删除
// 对数据库里map2删除map1新增
// 例外情况：任何删除后检索两边长度，有0的时候结束

// todo 添加config表，增加记录启动器路径
// todo 添加执行cmd指令，通过cmd指令启动honeyview
fn main() {
    init_db();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,add_library])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
