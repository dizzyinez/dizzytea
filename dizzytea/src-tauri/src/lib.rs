use flecs_ecs::prelude::*;
use std::sync::Mutex;
use tauri::{Builder, Manager, State};

mod data_types;

struct AppState {
    world: Mutex<World>,
    //cache: Option<AssetCache>,
}

impl AppState {
    fn new() -> AppState {
        //if std::fs::exists("/home/inez/.dizzytea/")
        //    .inspect_err(|e| eprintln!("Failed to check if directory exists: {e}"))
        //    .is_ok_and(|exists| !exists)
        //{
        //    std::fs::create_dir("/home/inez/.dizzytea/")
        //        .inspect_err(|e| eprintln!("Failed to create directory: {e}"));
        //}
        AppState {
            world: Mutex::new(World::new()),
            //    cache: AssetCache::new("/home/inez/.dizzytea/")
            //        .inspect_err(|e| eprintln!("Failed to open file cache: {e}"))
            //        .ok(),
        }
    }
}

#[tauri::command]
fn get_entire_world(state: State<'_, AppState>) -> String {
    let world = state.world.lock().unwrap();
    world.to_json_world(None)
}

fn load_file_metadata(d: &std::fs::DirEntry, world: &Mutex<World>) -> std::io::Result<()> {
    match d.path().extension() {
        None => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "no file extension",
        )),
        Some(os_str) => match os_str.to_str() {

            Some("mp3") | Some("wav") | Some("flac") => Ok(()),

            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid file extension",
            )),
            //TODO: implement unsupported filetypes in ECS so they at least show up and are
            //grabbable :3
            Some(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "unsupported filetype",
            )),
        },
    }
    //let file = std::fs::File::open(d.path);
}

fn load_dir<P: AsRef<std::path::Path>>(path: P, world: &Mutex<World>) -> std::io::Result<()> {
    //let world = world.lock().unwrap();
    for entry in std::fs::read_dir(path)? {
        let Ok(entry) = entry else {
            eprintln!("skipped entry");
            continue;
        };
        if entry.path().is_dir() {
            let _ = load_dir(entry.path(), world).inspect_err(|e| println!("{e}"));
        }
        if entry.path().is_file() {
            load_file_metadata(&entry, world);
        }
        //println!("{}", entry.file_name().to_string_lossy());
    }
    Ok(())
}

#[tauri::command]
async fn reload_asset_metadata(state: State<'_, AppState>) -> Result<(), String> {
    println!("FUCK");
    match load_dir("/home/inez/.dizzytea".to_string(), &state.world) {
        Ok(_r) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
    //let state = state.lock().unwrap();
    //let cache = state.cache.as_ref().ok_or("Asset cache does not exist!")?;
    //for item in cache.load_rec_dir("") {}
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AppState::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_entire_world,
            reload_asset_metadata
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
