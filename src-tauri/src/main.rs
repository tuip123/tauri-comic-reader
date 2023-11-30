#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::collections::HashSet;
use std::fs;
use std::fs::create_dir_all;
use std::fs::metadata;
use std::path::Path;
use std::process::Command;

use serde_with::serde_as;
use sqlite::{Connection, Statement};
use sqlite::State;
use tauri::api::path::app_local_data_dir;
use trash;

#[serde_as]
#[derive(serde::Serialize)]
#[derive(Eq, Hash, PartialEq)]
#[derive(Debug)]
pub struct Config {
    pub key: String,
    pub value: String,
}


#[serde_as]
#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct Pagination {
    pub current: i32,
    pub size: i32,
    pub total: i32,
}

#[serde_as]
#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct Library {
    pub id: i64,
    pub root: String,
    pub count: i64,
    pub random_mode: i64,
}

#[serde_as]
#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct LibraryList {
    pub list: Vec<Library>,
    pub pagination: Pagination,
}

#[serde_as]
#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct Comic {
    pub id: i64,
    pub path: String,
    pub title: String,
    pub cover: String,
    pub count: i64,
    pub library_id: i64,
}

#[serde_as]
#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct ComicList {
    pub list: Vec<Comic>,
    pub pagination: Pagination,
}

#[serde_as]
#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct ComicRead {
    pub page: Vec<String>,
}

static NOW_VERSION_CODE: i32 = 9;
static NOW_VERSION: &str = "0.2.1";

// 初始化相关
fn get_conn() -> Result<Connection, String> {
    let dir = app_local_data_dir(&Default::default()).unwrap().join("tuip123-comic\\");
    let full_dir = dir.to_str().unwrap().to_owned();
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
        query = "CREATE TABLE comic (Id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,path TEXT,title TEXT,cover TEXT,count INTEGER,libraryId INTEGER,isDelete INTEGER DEFAULT 0,lastPage INTEGER DEFAULT 0)";
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
        query = "\
        CREATE TABLE config (key TEXT PRIMARY KEY,value TEXT);\
        insert into config (key,value) values ('third_party_image_viewer','null');\
        insert into config (key,value) values ('third_party_open','false');\
        insert into config (key,value) values ('delete_source_file','false');\
        insert into config (key,value) values ('minimize_window','false');\
        insert into config (key,value) values ('comic_width',40);\
        insert into config (key,value) values ('read_type',0);
        ";
        conn.execute(query).unwrap();
        let mut temp = format!("insert into config (key,value) values ('version','{}');", NOW_VERSION);
        query = &*temp;
        conn.execute(query).unwrap();
        temp = format!("insert into config (key,value) values ('version_code',{});", NOW_VERSION_CODE);
        query = &*temp;
        conn.execute(query).unwrap();
    }
}

fn update_app() {
    let conn = get_conn().unwrap();
    let mut select = conn.prepare("select value from config where key = 'version_code'").unwrap();
    let mut update: Statement;
    let mut version_code = -1;
    while let Ok(State::Row) = select.next() {
        let str = select.read::<String, _>(0).unwrap();
        version_code = str.parse::<i32>().unwrap();
    }
    if version_code == -1 {
        let insert = "insert into config (key,value) values ('version_code',1) ";
        conn.execute(insert).unwrap();
    }
    if version_code < NOW_VERSION_CODE {
        if version_code < 1 {
            let insert = "insert into config (key,value) values ('minimize_window','false') ";
            match conn.execute(insert) {
                Ok(_) => {}
                Err(_) => {}
            };
        }
        if version_code < 3 {
            let insert = "insert into config (key,value) values ('comic_width',40);\
                                insert into config (key,value) values ('read_type',0);";
            match conn.execute(insert) {
                Ok(_) => {}
                Err(_) => {}
            };
        }
        if version_code < 5 {
            let alert = "ALTER TABLE comic ADD COLUMN isDelete INTEGER DEFAULT 0";
            match conn.execute(alert) {
                Ok(_) => {}
                Err(_) => {}
            };
        }
        if version_code < 7 {
            let alert = "ALTER TABLE comic ADD COLUMN lastPage INTEGER DEFAULT 0";
            match conn.execute(alert) {
                Ok(_) => {}
                Err(_) => {}
            };
        }
        if version_code < 9 {
            let alert = "ALTER TABLE library ADD COLUMN randomMode INTEGER DEFAULT 0";
            match conn.execute(alert) {
                Ok(_) => {}
                Err(_) => {}
            };
        }
        let temp = format!("update config set value = '{}' where key = 'version' ", NOW_VERSION);
        let update_version = &*temp;
        conn.execute(update_version).unwrap();
        update = conn.prepare("update config set value = ? where key = 'version_code' ").unwrap();
        update.bind((1, NOW_VERSION_CODE.to_string().as_str())).unwrap();
        update.next().unwrap();
    }
}

fn has_extension(exten: &str, extensions: &[String]) -> bool {
    for e in extensions {
        if exten == e
        {
            return true;
        }
    }
    false
}

// 库相关
#[tauri::command]
fn add_library(path: &str) -> Result<(), String> {
    let conn = get_conn().unwrap();
    // 查找有无记录
    let mut insert = conn.prepare("select Id from library where root = ? limit 1").unwrap();
    insert.bind((1, path)).unwrap();
    while let Ok(State::Row) = insert.next() {
        return Err(String::from("库路径重复"));
    }
    // 添加库记录
    insert = conn.prepare("insert into library (root) values (?)").unwrap();
    insert.bind((1, path)).unwrap();
    insert.next().unwrap();
    insert = conn.prepare("select Id from library where root = ? limit 1").unwrap();
    insert.bind((1, path)).unwrap();
    let mut library_id = 0;
    while let Ok(State::Row) = insert.next() {
        library_id = insert.read::<i64, _>(0).unwrap();
    }
    // 添加库下漫画记录
    let result = reload_library(library_id);
    return match result {
        Ok(_statement) => {
            Ok(())
        }
        Err(_err) => {
            Err(String::from("添加失败"))
        }
    };
}

#[tauri::command]
fn reload_library(library_id: i64) -> Result<(), String> {
    let conn = get_conn().unwrap();

    let mut select = conn.prepare("select root from library where Id = ?").unwrap();
    select.bind((1, library_id)).unwrap();
    let mut root = String::from("");
    while let Ok(State::Row) = select.next() {
        root = select.read::<String, _>(0).unwrap();
    }
    if root == "" {
        return Err(String::from("库id不存在"));
    }
    let mut path_set: HashSet<String> = HashSet::new();
    let paths = fs::read_dir(root).unwrap();
    for path in paths {
        let comic_path = path.unwrap().path();
        let title = comic_path.to_str().unwrap().to_string();
        path_set.insert(title);
    }


    select = conn.prepare("select Id,path from comic where libraryId = ? ").unwrap();
    select.bind((1, library_id)).unwrap();
    while let State::Row = select.next().unwrap() {
        let id: i64 = select.read::<i64, _>(0).unwrap();
        let path: String = select.read::<String, _>(1).unwrap();
        if path_set.contains(&path) {
            path_set.remove(&path);
        } else {
            let mut delete = conn.prepare("delete from comic where Id = ?").unwrap();
            delete.bind((1, id)).unwrap();
            delete.next().unwrap();
        }
    }


    for comic_path in path_set.iter() {
        let mut insert = conn.prepare("insert into comic (libraryId,title,path,cover,count) values (?,?,?,?,?)").unwrap();
        let comic_path_clone = comic_path.clone();
        let comic_path_clone2 = comic_path.clone();
        let comic_path_clone3 = comic_path.clone();
        let md = metadata(comic_path_clone3).unwrap();
        if md.is_file() {
            continue;
        }

        let temp: Vec<_> = comic_path.split("\\").collect();
        let title = temp[temp.len() - 1];
        // let title = comic_path.file_name().unwrap().to_str().unwrap();

        let comic_path_str = comic_path_clone.as_str();
        let mut comic_cover_str = comic_path_str.clone().to_string();

        let files = fs::read_dir(comic_path_clone2).unwrap();
        let mut comic_count = 0;
        let mut cover_temp = true;

        for file in files {
            let file_name = file.unwrap().file_name();
            let file_name_clone = file_name.clone();
            let file_split: Vec<&str> = file_name.to_str().unwrap().split(".").collect();
            let lc_extend_name = file_split[file_split.len() - 1].to_string().to_lowercase();
            let extend_name = lc_extend_name.trim();
            if has_extension(extend_name, &[String::from("png"), String::from("jpg"), String::from("jpeg")])
            {
                if cover_temp == true {
                    comic_cover_str = format!("{}\\{}", comic_cover_str, file_name_clone.to_str().unwrap());
                    cover_temp = false;
                }
                comic_count += 1;
            }
        }
        if comic_count == 0 {
            continue;
        }
        insert.bind((1, library_id)).unwrap();
        insert.bind((2, title)).unwrap();
        insert.bind((3, comic_path_str)).unwrap();
        insert.bind((4, &*comic_cover_str)).unwrap();
        insert.bind((5, comic_count)).unwrap();
        insert.next().unwrap();
    }

    select = conn.prepare("select count(1) from comic where libraryId = ? and isDelete = 0 ").unwrap();
    select.bind((1, library_id)).unwrap();
    let mut count = 0;
    while let Ok(State::Row) = select.next() {
        count = select.read::<i64, _>(0).unwrap();
    }

    let mut update = conn.prepare("update library set count = ? where id = ?").unwrap();
    update.bind((1, count)).unwrap();
    update.bind((2, library_id)).unwrap();
    update.next().unwrap();
    Ok(())
}

// 查询相关
#[tauri::command]
fn query_library(search: &str, page: i64, page_size: i64) -> Result<LibraryList, String> {
    let conn = get_conn().unwrap();
    let offset = (page - 1) * page_size;
    let mut select: Statement;
    let mut return_list: LibraryList = LibraryList {
        list: vec![],
        pagination: Pagination { current: page as i32, size: page_size as i32, total: 0 },
    };
    select = conn.prepare("select count(1) from library").unwrap();
    while let State::Row = select.next().unwrap() {
        return_list.pagination.total = select.read::<i64, _>(0).unwrap() as i32;
    }
    if search.trim().len() == 0 {
        select = conn.prepare("select Id,root,count,randomMode from library LIMIT ? OFFSET ?").unwrap();
        select.bind((1, page_size)).unwrap();
        select.bind((2, offset)).unwrap();
    } else {
        select = conn.prepare("select Id,root,count,randomMode from library where root LIKE ? LIMIT ? OFFSET ?").unwrap();
        select.bind((1, &*String::from(format!("%{}%", search.trim())))).unwrap();
        select.bind((2, page_size)).unwrap();
        select.bind((3, offset)).unwrap();
    }
    while let State::Row = select.next().unwrap() {
        let library: Library = Library {
            id: select.read::<i64, _>(0).unwrap(),
            root: select.read::<String, _>(1).unwrap(),
            count: select.read::<i64, _>(2).unwrap(),
            random_mode: select.read::<i64, _>(3).unwrap(),
        };
        return_list.list.push(library);
    }
    Ok(return_list)
}

#[tauri::command]
fn query_library_by_id(id: i64) -> Result<Library, String> {
    let conn = get_conn().unwrap();
    let mut select: Statement;
    select = conn.prepare("select Id,root,count,randomMode from library where id = ?").unwrap();
    select.bind((1, id)).unwrap();
    let mut library: Library = Library {
        id: 0,
        root: "".to_string(),
        count: 0,
        random_mode: 0,
    };
    while let State::Row = select.next().unwrap() {
        library = Library {
            id: select.read::<i64, _>(0).unwrap(),
            root: select.read::<String, _>(1).unwrap(),
            count: select.read::<i64, _>(2).unwrap(),
            random_mode: select.read::<i64, _>(3).unwrap(),
        };
    }
    Ok(library)
}

#[tauri::command]
fn query_comic(search: &str, library_id: i64, page: i64, page_size: i64) -> Result<ComicList, String> {
    let conn = get_conn().unwrap();
    let offset = (page - 1) * page_size;
    let mut select: Statement;
    let mut return_list: ComicList = ComicList {
        list: vec![],
        pagination: Pagination { current: page as i32, size: page_size as i32, total: 0 },
    };
    if search.trim().len() == 0 {
        select = conn.prepare("select count(1) from comic where libraryId = ? and isDelete = 0").unwrap();
        select.bind((1, library_id)).unwrap();
    } else {
        select = conn.prepare("select count(1) from comic where libraryId = ? and title LIKE ? and isDelete = 0").unwrap();
        select.bind((1, library_id)).unwrap();
        select.bind((2, &*String::from(format!("%{}%", search.trim())))).unwrap();
    }
    while let State::Row = select.next().unwrap() {
        return_list.pagination.total = select.read::<i64, _>(0).unwrap() as i32;
    }

    if search.trim().len() == 0 {
        select = conn.prepare("select Id,path,title,cover,count from comic where libraryId = ? and isDelete = 0 LIMIT ? OFFSET ? ").unwrap();
        select.bind((1, library_id)).unwrap();
        select.bind((2, page_size)).unwrap();
        select.bind((3, offset)).unwrap();
    } else {
        select = conn.prepare("select Id,path,title,cover,count from comic where libraryId = ? and title LIKE ? and isDelete = 0 LIMIT ? OFFSET ? ").unwrap();
        select.bind((1, library_id)).unwrap();
        select.bind((2, &*String::from(format!("%{}%", search.trim())))).unwrap();
        select.bind((3, page_size)).unwrap();
        select.bind((4, offset)).unwrap();
    }
    while let State::Row = select.next().unwrap() {
        let comic: Comic = Comic {
            id: select.read::<i64, _>(0).unwrap(),
            path: select.read::<String, _>(1).unwrap(),
            title: select.read::<String, _>(2).unwrap(),
            cover: select.read::<String, _>(3).unwrap(),
            count: select.read::<i64, _>(4).unwrap(),
            library_id,
        };
        return_list.list.push(comic);
    }
    Ok(return_list)
}

#[tauri::command]
fn query_comic_name(library_id: i64) -> Result<Vec<Comic>, String> {
    let conn = get_conn().unwrap();
    let mut select = conn.prepare("select Id,title,path from comic where libraryId = ? and isDelete = 0").unwrap();
    select.bind((1, library_id)).unwrap();
    let mut title_list: Vec<Comic> = Vec::new();
    while let State::Row = select.next().unwrap() {
        let comic: Comic = Comic {
            id: select.read::<i64, _>(0).unwrap(),
            path: select.read::<String, _>(2).unwrap(),
            title: select.read::<String, _>(1).unwrap(),
            cover: "".to_string(),
            count: 0,
            library_id,
        };
        title_list.push(comic);
    };
    Ok(title_list)
}

// 配置相关
#[tauri::command]
fn add_third_party_image_viewer(path: &str) -> Result<String, String> {
    if cfg!(target_os = "windows") {
        let conn = get_conn().unwrap();
        let mut update = conn.prepare("update config set value = ? where key = 'third_party_image_viewer'").unwrap();
        update.bind((1, path)).unwrap();
        update.next().unwrap();
        Ok(String::from("设置成功"))
    } else {
        Err(String::from("不支持当前系统"))
    }
}

#[tauri::command]
fn update_config(key: &str, value: &str) -> Result<(), String> {
    let conn = get_conn().unwrap();
    let mut select = conn.prepare("select count(1) from config where key = ?").unwrap();
    select.bind((1, key)).unwrap();
    let mut count = 0;
    while let State::Row = select.next().unwrap() {
        count = select.read::<i64, _>(0).unwrap();
    }
    if count == 0 {
        let mut insert = conn.prepare("insert into config (key,value) values (?,?) ").unwrap();
        insert.bind((1, key)).unwrap();
        insert.bind((2, value)).unwrap();
        insert.next().unwrap();
        Ok(())
    } else {
        let mut update = conn.prepare("update config set value = ? where key = ?").unwrap();
        update.bind((1, value)).unwrap();
        update.bind((2, key)).unwrap();
        update.next().unwrap();
        Ok(())
    }
}


#[tauri::command]
fn get_config() -> Result<HashSet<Config>, String> {
    let conn = get_conn().unwrap();
    let mut select = conn.prepare("select key,value from config").unwrap();
    let mut set: HashSet<Config> = HashSet::new();
    while let State::Row = select.next().unwrap() {
        let mut config: Config = Config { key: "".to_string(), value: "".to_string() };
        config.key = select.read::<String, _>(0).unwrap();
        config.value = select.read::<String, _>(1).unwrap();
        set.insert(config);
    }
    Ok(set)
}

// 删除相关
#[tauri::command]
fn delete_comic(id: i64) {
    // 检查是否删除源文件
    let conn = get_conn().unwrap();
    let mut select = conn.prepare("select value from config where key = 'delete_source_file'").unwrap();
    let mut delete_source_file = false;
    while let State::Row = select.next().unwrap() {
        let temp = select.read::<String, _>(0).unwrap();
        if temp == "true" {
            delete_source_file = true;
        }
    };
    // 执行删除源文件
    if delete_source_file {
        select = conn.prepare("select path from comic where id = ?").unwrap();
        select.bind((1, id)).unwrap();

        let mut path = String::from("");
        while let State::Row = select.next().unwrap() {
            path = select.read::<String, _>(0).unwrap();
        }
        trash::delete(path).unwrap();

        let mut delete = conn.prepare("delete from comic where id = ?").unwrap();
        delete.bind((1, id)).unwrap();
        delete.next().unwrap();
    } else {
        let mut delete = conn.prepare("update comic set isDelete = 1 where id = ?").unwrap();
        delete.bind((1, id)).unwrap();
        delete.next().unwrap();
    }

    // 获取library id
    select = conn.prepare("select libraryId from comic where id = ?").unwrap();
    select.bind((1, id)).unwrap();
    let mut library_id = 0;
    while let State::Row = select.next().unwrap() {
        library_id = select.read::<i64, _>(0).unwrap();
    }

    // 更新library数据
    select = conn.prepare("select count(1) from comic where libraryId = ? and isDelete = 0").unwrap();
    select.bind((1, library_id)).unwrap();
    let mut count = 0;
    while let State::Row = select.next().unwrap() {
        count = select.read::<i64, _>(0).unwrap();
    }
    let mut update = conn.prepare("update library set count = ? where id = ?").unwrap();
    update.bind((1, count)).unwrap();
    update.bind((2, library_id)).unwrap();
    update.next().unwrap();
}

#[tauri::command]
fn delete_library(id: i64) {
    let conn = get_conn().unwrap();
    let mut delete = conn.prepare("delete from library where id = ?").unwrap();
    delete.bind((1, id)).unwrap();
    delete.next().unwrap();

    delete = conn.prepare("delete from comic where libraryId = ?").unwrap();
    delete.bind((1, id)).unwrap();
    delete.next().unwrap();
}

#[tauri::command]
fn set_library_random(id: i64, mode: i64) {
    let conn = get_conn().unwrap();
    let mut update = conn.prepare("update library set randomMode = ? where id = ?").unwrap();
    update.bind((1, mode)).unwrap();
    update.bind((2, id)).unwrap();
    update.next().unwrap();
}

// 阅读相关
#[tauri::command]
fn open_with_third_party(folder: &str) -> Result<(), String> {
    let conn = get_conn().unwrap();
    let mut select = conn.prepare("select value from config where key = 'third_party_image_viewer'").unwrap();
    let mut third_party_path = String::from("");
    while let State::Row = select.next().unwrap() {
        third_party_path = select.read::<String, _>(0).unwrap();
    }
    if third_party_path == "null" {
        return Err(String::from("未定义第三方启动器"));
    }
    if cfg!(target_os = "windows") {
        Command::new(third_party_path).arg(folder).spawn().unwrap();
        Ok(())
    } else {
        Err(String::from("不支持当前系统"))
    }
}

#[tauri::command]
fn read_comic(id: i64) -> Result<Vec<String>, String> {
    let mut comic_read: ComicRead = ComicRead {
        page: vec![]
    };
    let conn = get_conn().unwrap();
    let mut select = conn.prepare("select path from comic where id = ?").unwrap();
    select.bind((1, id)).unwrap();
    let mut path = String::from("");
    while let State::Row = select.next().unwrap() {
        path = select.read::<String, _>(0).unwrap();
    }
    if path == "" {
        return Err(String::from("路径不存在"));
    }
    let paths = fs::read_dir(path).unwrap();
    let mut count = 0;
    for p in paths {
        let str = String::from(p.unwrap().path().to_str().unwrap());
        let file_split: Vec<&str> = str.split(".").collect();
        let lc_extend_name = file_split[file_split.len() - 1].to_string().to_lowercase();
        let extend_name = lc_extend_name.trim();
        if has_extension(extend_name, &[String::from("png"), String::from("jpg"), String::from("jpeg")])
        {
            comic_read.page.push(str);
            count += 1;
        }
    }
    let mut update = conn.prepare("update comic set count = ? where id = ?").unwrap();
    update.bind((1, count)).unwrap();
    update.bind((2, id)).unwrap();
    update.next().unwrap();
    Ok(comic_read.page)
}

#[tauri::command]
fn get_page(id: i64) -> Result<i64, String> {
    let conn = get_conn().unwrap();
    let mut select = conn.prepare("select lastPage from comic where id = ?").unwrap();
    select.bind((1, id)).unwrap();
    let mut last_page: String = String::from("");
    while let State::Row = select.next().unwrap() {
        last_page = select.read::<String, _>(0).unwrap();
    }
    Ok(last_page.parse().unwrap())
}

#[tauri::command]
fn save_page(id: i64, page: i64) -> Result<(), String> {
    let conn = get_conn().unwrap();
    let mut update = conn.prepare("update comic set lastPage = ? where id = ?").unwrap();
    update.bind((1, page)).unwrap();
    update.bind((2, id)).unwrap();
    update.next().unwrap();
    Ok(())
}

#[tauri::command]
fn open_source_folder(folder: &str) -> Result<(), String> {
    if cfg!(target_os = "windows") {
        // Command::new("explorer").arg(folder).spawn().unwrap();
        open::that(folder).unwrap();
        Ok(())
    } else {
        Err(String::from("不支持当前系统"))
    }
}

#[tauri::command]
fn minimize_window() -> bool {
    let conn = get_conn().unwrap();
    let mut select = conn.prepare("select value from config where key = 'minimize_window'").unwrap();
    while let State::Row = select.next().unwrap() {
        let temp = select.read::<String, _>(0).unwrap();
        if temp == "true" {
            return true;
        }
    };
    return false;
}

fn main() {
    init_db();
    update_app();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_library,
            reload_library,
            update_config,
            get_config,
            add_third_party_image_viewer,
            open_with_third_party,
            open_source_folder,
            query_library,
            query_library_by_id,
            query_comic,
            query_comic_name,
            delete_comic,
            delete_library,
            read_comic,
            get_page,
            save_page,
            minimize_window,
            set_library_random,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
