use bevy::prelude::*;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use crate::input::KeyBindings;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PlayerSave {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource, Default)]
pub struct LoadRequest(pub bool);

fn save_dir() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("com", "KoryavayaIgra", "KoryavayaIgra")?;
    let path = dirs.config_dir().to_path_buf();
    fs::create_dir_all(&path).ok()?;
    Some(path)
}

pub fn save_game(save: &PlayerSave, bindings: &KeyBindings) {
    let Some(dir) = save_dir() else {
        eprintln!("[save] cannot find save dir");
        return;
    };
    match ron::to_string(save) {
        Ok(s) => { let _ = fs::write(dir.join("save_0.ron"), s); }
        Err(e) => eprintln!("[save] serialize error: {e}"),
    }
    match ron::to_string(bindings) {
        Ok(s) => { let _ = fs::write(dir.join("keybinds.ron"), s); }
        Err(e) => eprintln!("[save] keybinds serialize error: {e}"),
    }
    if let Some(d) = save_dir() {
        println!("[save] saved to {}", d.display());
    }
}

pub fn load_save() -> Option<PlayerSave> {
    let text = fs::read_to_string(save_dir()?.join("save_0.ron")).ok()?;
    ron::from_str(&text).ok()
}

pub fn load_keybinds() -> Option<KeyBindings> {
    let text = fs::read_to_string(save_dir()?.join("keybinds.ron")).ok()?;
    ron::from_str(&text).ok()
}
