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
        query = "CREATE TABLE comic (Id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,path TEXT,title TEXT,cover TEXT,count INTEGER,libraryId INTEGER)";
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
        query = "CREATE TABLE library (Id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,root TEXT,count INTEGER)";
        conn.execute(query).unwrap();
    }
}

use std::fs;

#[tauri::command]
fn add_library(path: &str) -> bool {
    let conn = get_conn();
    // 查找有无记录
    let mut insert = conn.prepare("select Id from library where root = ? limit 1").unwrap();
    insert.bind(1, path).unwrap();
    while let State::Row = insert.next().unwrap() {
        return false;
    }
    // 添加库记录
    insert = conn.prepare("insert into library (root) values (?)").unwrap();
    insert.bind(1, path).unwrap();
    insert.next().unwrap();
    insert = conn.prepare("select Id from library where root = ? limit 1").unwrap();
    insert.bind(1, path).unwrap();
    let mut library_id = 0;
    while let State::Row = insert.next().unwrap() {
        library_id = insert.read::<i64>(0).unwrap();
    }
    // 添加库下漫画记录
    reload_library(library_id);
    return true;
}

#[tauri::command]
fn reload_library(library_id: i64) -> bool {
    let conn = get_conn();

    let mut delete = conn.prepare("delete from comic where libraryId = ?").unwrap();
    delete.bind(1, library_id).unwrap();
    delete.next().unwrap();

    let mut select = conn.prepare("select root from library where Id = ?").unwrap();
    select.bind(1, library_id).unwrap();
    let mut root = String::from("");
    while let State::Row = select.next().unwrap() {
        root = select.read::<String>(0).unwrap();
    }
    if root == "" {
        println!("库id不存在");
    }
    let paths = fs::read_dir(root).unwrap();
    let mut count = 0;
    for path in paths {
        count += 1;
        let mut insert = conn.prepare("insert into comic (libraryId,title,path,count) values (?,?,?,?)").unwrap();
        let comic_path = path.unwrap().path();
        let comic_path_clone = comic_path.clone();
        let comic_path_clone2 = comic_path.clone();

        let title = comic_path_clone.file_name().unwrap().to_str().unwrap();

        let comic_path_str = comic_path.to_str().unwrap();

        let comic_dir = fs::read_dir(comic_path_clone2).unwrap();
        let mut comic_count = 0;
        for file in comic_dir {
            let temp = file.unwrap().file_name();
            let file_name: Vec<&str> = temp.to_str().unwrap().split(".").collect();
            if file_name[file_name.len()-1] == "jpg"
            {
                comic_count+=1;
            }
        }

        insert.bind(1, library_id).unwrap();
        insert.bind(2, title).unwrap();
        insert.bind(3, comic_path_str).unwrap();
        insert.bind(4,comic_count).unwrap();
        insert.next().unwrap();
    }

    let mut update = conn.prepare("update library set count = ? where id = ?").unwrap();
    update.bind(1, count).unwrap();
    update.bind(2, library_id).unwrap();
    update.next().unwrap();

    false
}
// todo 添加cover获取封面
// 思路 两个map，第一个为path结果 第二个为数据库结果
// map1遍历，map2中查，有结果，两边删除 此时剩余的map1为新增，map2为删除
// 对数据库里map2删除map1新增
// 例外情况：任何删除后检索两边长度，有0的时候结束

// todo 添加config表，增加记录启动器路径
// todo 添加执行cmd指令，通过cmd指令启动honeyview
fn main() {
    init_db();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,add_library,reload_library])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
