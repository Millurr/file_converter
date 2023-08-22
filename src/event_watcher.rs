use std::path::Path;

use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result, Config, Event, EventKind};

pub fn watch_folder_trigger<P: AsRef<Path>>(path: P) -> Result<String> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    let mut path: String = String::new();

    for res in rx {
        match res {
            Ok(event) => {
                
                path = String::from(event.paths[0].to_string_lossy());

                if event.kind.is_modify() {
                    println!("Modify event: {:?}", &path);
                }
                else if event.kind.is_create() {
                    println!("Create event: {:?}", &path);
                }
                else if event.kind.is_access() {
                    break;
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(path)
}
