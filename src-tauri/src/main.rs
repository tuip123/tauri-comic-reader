#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use sqlite::{Connection, State};
use std::fs::create_dir_all;
use std::path::Path;
use tauri::api::path::app_data_dir;
use std::fs;

fn get_conn() -> Result<Connection,String> {
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
    Ok(conn)
}

fn init_db() {
    let conn = get_conn().unwrap();

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

    query = "SELECT name FROM sqlite_master WHERE type='table' AND name='config' COLLATE NOCASE";
    statement = conn
        .prepare(query)
        .unwrap();
    b = false;
    while let State::Row = statement.next().unwrap() {
        b = true;
    }
    if !b {
        query = "CREATE TABLE config (key TEXT PRIMARY KEY,value TEXT)";
        conn.execute(query).unwrap();
    }
}

#[tauri::command]
fn add_library(path: &str) -> Result<(),String> {
    let conn = get_conn().unwrap();
    // 查找有无记录
    let mut insert = conn.prepare("select Id from library where root = ? limit 1").unwrap();
    insert.bind(1, path).unwrap();
    while let State::Row = insert.next().unwrap() {
        return Err(String::from("库路径重复"));
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
    let result = reload_library(library_id);
    return match result {
        Ok(_statement) => {
            Ok(())
        },
        Err(_err) => {
            Err(String::from("添加失败"))
        }
    }
}

#[tauri::command]
fn reload_library(library_id: i64) -> Result<(),String>{
    let conn = get_conn().unwrap();

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
        return Err(String::from("库id不存在"));
    }
    let paths = fs::read_dir(root).unwrap();
    let mut count = 0;
    for path in paths {
        count += 1;
        let mut insert = conn.prepare("insert into comic (libraryId,title,path,cover,count) values (?,?,?,?,?)").unwrap();
        let comic_path = path.unwrap().path();
        let comic_path_clone = comic_path.clone();
        let comic_path_clone2 = comic_path.clone();

        let title = comic_path.file_name().unwrap().to_str().unwrap();

        let comic_path_str = comic_path_clone.to_str().unwrap();
        let mut comic_cover_str = comic_path_str.clone().to_string();

        let comic_dir = fs::read_dir(comic_path_clone2).unwrap();
        let mut comic_count = 0;
        let mut cover_temp = true;
        for file in comic_dir {
            let file_name = file.unwrap().file_name();
            let file_name_clone = file_name.clone();
            let file_split: Vec<&str> = file_name.to_str().unwrap().split(".").collect();
            if file_split[file_split.len() - 1] == "jpg"
            {
                if cover_temp == true {
                    comic_cover_str = format!("{}\\{}", comic_cover_str, file_name_clone.to_str().unwrap());
                    cover_temp = false;
                }
                comic_count += 1;
            }
        }

        insert.bind(1, library_id).unwrap();
        insert.bind(2, title).unwrap();
        insert.bind(3, comic_path_str).unwrap();
        insert.bind(4, &*comic_cover_str).unwrap();
        insert.bind(5, comic_count).unwrap();
        insert.next().unwrap();
    }

    let mut update = conn.prepare("update library set count = ? where id = ?").unwrap();
    update.bind(1, count).unwrap();
    update.bind(2, library_id).unwrap();
    update.next().unwrap();
    Ok(())
}

#[tauri::command]
fn add_third_party_image_viewer(path:&str)->Result<(),String>{
    let conn = get_conn().unwrap();
    let mut insert = conn.prepare("select count(1) from config where key = 'third_party_image_viewer'").unwrap();
    let mut update_type = false;
    while let State::Row = insert.next().unwrap() {
        if insert.read::<i64>(0).unwrap() == 1 {
            update_type = true;
        }
    }
    if update_type == true {
        let mut update = conn.prepare("update config set value = ? where key = 'third_party_image_viewer'").unwrap();
        update.bind(1,path).unwrap();
        update.next().unwrap();
    }
    else {
        insert = conn.prepare("insert into config (key,value) values ('third_party_image_viewer',?)").unwrap();
        insert.bind(1,path).unwrap();
        insert.next().unwrap();
    }
    Ok(())
}
// todo 获取所有library
// todo 根据libraryid获取所有comic
// todo 分页查询，搜索查询
// todo 添加执行cmd指令，通过cmd指令启动honeyview

fn main() {
    init_db();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_library,reload_library,add_third_party_image_viewer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
