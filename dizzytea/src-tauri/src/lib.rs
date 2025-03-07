use flecs_ecs::prelude::*;
//use flecs_ecs::core::Builder;
use std::sync::Mutex;
use tauri::{Manager, State};

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
    world.to_json_world(Some(&json::WorldToJsonDesc {
        serialize_builtin: false,
        serialize_modules: false,
    }))
}

// Request query
#[tauri::command]
async fn ecs_query(state: State<'_, AppState>, query: String) -> Result<String, String> {
    let world = state.world.lock().unwrap();
    world
        .query::<()>()
        .expr(&query.to_string())
        .build()
        .to_json(None)
        .ok_or("Query failed!".to_string())
}

// Request named query
#[tauri::command]
async fn ecs_query_named(
    state: State<'_, AppState>,
    name: String,
    query: String,
) -> Result<String, String> {
    let world = state.world.lock().unwrap();
    world
        .query_named::<()>(&name.to_string())
        .expr(&query.to_string())
        .build()
        .to_json(None)
        .ok_or("Query failed".to_string())
}

// Add tag
#[tauri::command]
async fn ecs_add(
    state: State<'_, AppState>,
    path: String,
    component: String,
) -> Result<(), String> {
    let world = state.world.lock().unwrap();
    let comp = world.entity_named(&component.to_string());
    let entity = world.entity_named(&path.to_string());
    entity.add_id(comp);
    //world.from_json_id(comp, entity.get_untyped_mut(comp), &value.to_string(), None);
    Ok(())
}

// Set component
#[tauri::command]
async fn ecs_set(
    state: State<'_, AppState>,
    path: String,
    component: String,
    value: String,
) -> Result<(), String> {
    let world = state.world.lock().unwrap();
    let comp = world.entity_named(&component.to_string());
    let entity = world.entity_named(&path.to_string());
    world.from_json_id(comp, entity.get_untyped_mut(comp), &value.to_string(), None);
    Ok(())
}

// Get component
#[tauri::command]
async fn ecs_get(
    state: State<'_, AppState>,
    path: String,
    component: String,
) -> Result<String, String> {
    let world = state.world.lock().unwrap();
    let comp = world.entity_named(&component.to_string());
    let entity = world.entity_named(&path.to_string());
    if !entity.has_id(comp) {
        return Err("Entity does not have component!".to_string());
    }
    Ok(world.to_json_id(comp, entity.get_untyped(comp)))
}

// Remove component
#[tauri::command]
async fn ecs_remove(
    state: State<'_, AppState>,
    path: String,
    component: String,
) -> Result<(), String> {
    let world = state.world.lock().unwrap();
    let comp = world.entity_named(&component.to_string());
    let entity = world.entity_named(&path.to_string());
    entity.remove_id(comp);
    Ok(())
}

// Enable entity/component
// Disable entity/component

// Lookup entity
#[tauri::command]
async fn ecs_lookup(state: State<'_, AppState>, path: String) -> Result<String, String> {
    let world = state.world.lock().unwrap();
    world
        .try_lookup(&path.to_string())
        .map_or(Err("Entity does not exist!".to_string()), |e| {
            e.path().ok_or("Entity does not exist!".to_string())
        })
}

// Check entity
#[tauri::command]
async fn ecs_exists(state: State<'_, AppState>, path: String) -> Result<bool, String> {
    let world = state.world.lock().unwrap();
    Ok(world.try_lookup(&path.to_string()).is_some())
}

// Create entity
#[tauri::command]
async fn ecs_create(state: State<'_, AppState>, path: String) -> Result<String, String> {
    let world = state.world.lock().unwrap();
    world
        .entity_named(&path.to_string())
        .path()
        .ok_or("Failed to create entity!".to_string())
}


// Get entity
#[tauri::command]
async fn ecs_entity(state: State<'_, AppState>, path: String) -> Result<String, ()> {
    let world = state.world.lock().unwrap();
    Ok(
    world
        .entity_named(&path.to_string())
        .to_json(Some(&flecs_ecs::addons::json::EntityToJsonDesc {
            serialize_builtin: true,
            serialize_doc: true,
            serialize_refs: 0,
            serialize_values: true,
            serialize_alerts: true,
            serialize_entity_id: true,
            serialize_matches: true,
            serialize_type_info: true,
            serialize_inherited: true,
            serialize_full_paths: true,
        }))
    )
        //.ok_or("Failed to create entity!".to_string())
}


//

// Delete entity
// Get all entities in world

// Parse flecs
#[tauri::command]
async fn ecs_run_script(
    state: State<'_, AppState>,
    path: String,
    code: String,
) -> Result<(), String> {
    let world = state.world.lock().unwrap();
    let e = world.entity_named(&path.to_string());
    if !e.has::<flecs::Script>() {
        world.script_from(e).build_from_code(&code.to_string());
    }
    world
        .script_entity_from_id(e)
        .update(e.world(), None::<Entity>, &code.to_string())
        .then_some(())
        .ok_or("Failed to run script!".to_string())
    //Ok(())
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
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AppState::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_entire_world,
            reload_asset_metadata,
            ecs_lookup,
            ecs_exists,
            ecs_create,
            ecs_entity,
            ecs_query,
            ecs_query_named,
            ecs_add,
            ecs_set,
            ecs_get,
            ecs_remove,
            ecs_run_script,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
