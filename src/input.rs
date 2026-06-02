use bevy::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Resource, Clone, Debug)]
pub struct KeyBindings {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub interact: KeyCode,
    pub pause: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            interact: KeyCode::KeyE,
            pause: KeyCode::Escape,
        }
    }
}

pub fn keycode_to_str(key: KeyCode) -> &'static str {
    match key {
        KeyCode::KeyA => "A", KeyCode::KeyB => "B", KeyCode::KeyC => "C",
        KeyCode::KeyD => "D", KeyCode::KeyE => "E", KeyCode::KeyF => "F",
        KeyCode::KeyG => "G", KeyCode::KeyH => "H", KeyCode::KeyI => "I",
        KeyCode::KeyJ => "J", KeyCode::KeyK => "K", KeyCode::KeyL => "L",
        KeyCode::KeyM => "M", KeyCode::KeyN => "N", KeyCode::KeyO => "O",
        KeyCode::KeyP => "P", KeyCode::KeyQ => "Q", KeyCode::KeyR => "R",
        KeyCode::KeyS => "S", KeyCode::KeyT => "T", KeyCode::KeyU => "U",
        KeyCode::KeyV => "V", KeyCode::KeyW => "W", KeyCode::KeyX => "X",
        KeyCode::KeyY => "Y", KeyCode::KeyZ => "Z",
        KeyCode::Digit0 => "0", KeyCode::Digit1 => "1", KeyCode::Digit2 => "2",
        KeyCode::Digit3 => "3", KeyCode::Digit4 => "4", KeyCode::Digit5 => "5",
        KeyCode::Digit6 => "6", KeyCode::Digit7 => "7", KeyCode::Digit8 => "8",
        KeyCode::Digit9 => "9",
        KeyCode::Space => "Space",
        KeyCode::Escape => "Esc",
        KeyCode::Enter => "Enter",
        KeyCode::Tab => "Tab",
        KeyCode::ArrowUp => "Up",
        KeyCode::ArrowDown => "Down",
        KeyCode::ArrowLeft => "Left",
        KeyCode::ArrowRight => "Right",
        KeyCode::ShiftLeft | KeyCode::ShiftRight => "Shift",
        KeyCode::ControlLeft | KeyCode::ControlRight => "Ctrl",
        KeyCode::AltLeft | KeyCode::AltRight => "Alt",
        KeyCode::F1 => "F1", KeyCode::F2 => "F2", KeyCode::F3 => "F3",
        KeyCode::F4 => "F4", KeyCode::F5 => "F5", KeyCode::F6 => "F6",
        KeyCode::F7 => "F7", KeyCode::F8 => "F8", KeyCode::F9 => "F9",
        KeyCode::F10 => "F10", KeyCode::F11 => "F11", KeyCode::F12 => "F12",
        _ => "?",
    }
}

pub fn str_to_keycode(s: &str) -> Option<KeyCode> {
    match s {
        "A" => Some(KeyCode::KeyA), "B" => Some(KeyCode::KeyB),
        "C" => Some(KeyCode::KeyC), "D" => Some(KeyCode::KeyD),
        "E" => Some(KeyCode::KeyE), "F" => Some(KeyCode::KeyF),
        "G" => Some(KeyCode::KeyG), "H" => Some(KeyCode::KeyH),
        "I" => Some(KeyCode::KeyI), "J" => Some(KeyCode::KeyJ),
        "K" => Some(KeyCode::KeyK), "L" => Some(KeyCode::KeyL),
        "M" => Some(KeyCode::KeyM), "N" => Some(KeyCode::KeyN),
        "O" => Some(KeyCode::KeyO), "P" => Some(KeyCode::KeyP),
        "Q" => Some(KeyCode::KeyQ), "R" => Some(KeyCode::KeyR),
        "S" => Some(KeyCode::KeyS), "T" => Some(KeyCode::KeyT),
        "U" => Some(KeyCode::KeyU), "V" => Some(KeyCode::KeyV),
        "W" => Some(KeyCode::KeyW), "X" => Some(KeyCode::KeyX),
        "Y" => Some(KeyCode::KeyY), "Z" => Some(KeyCode::KeyZ),
        "0" => Some(KeyCode::Digit0), "1" => Some(KeyCode::Digit1),
        "2" => Some(KeyCode::Digit2), "3" => Some(KeyCode::Digit3),
        "4" => Some(KeyCode::Digit4), "5" => Some(KeyCode::Digit5),
        "6" => Some(KeyCode::Digit6), "7" => Some(KeyCode::Digit7),
        "8" => Some(KeyCode::Digit8), "9" => Some(KeyCode::Digit9),
        "Space" => Some(KeyCode::Space),
        "Esc" => Some(KeyCode::Escape),
        "Enter" => Some(KeyCode::Enter),
        "Tab" => Some(KeyCode::Tab),
        "Up" => Some(KeyCode::ArrowUp),
        "Down" => Some(KeyCode::ArrowDown),
        "Left" => Some(KeyCode::ArrowLeft),
        "Right" => Some(KeyCode::ArrowRight),
        "Shift" => Some(KeyCode::ShiftLeft),
        "Ctrl" => Some(KeyCode::ControlLeft),
        "Alt" => Some(KeyCode::AltLeft),
        "F1" => Some(KeyCode::F1), "F2" => Some(KeyCode::F2),
        "F3" => Some(KeyCode::F3), "F4" => Some(KeyCode::F4),
        "F5" => Some(KeyCode::F5), "F6" => Some(KeyCode::F6),
        "F7" => Some(KeyCode::F7), "F8" => Some(KeyCode::F8),
        "F9" => Some(KeyCode::F9), "F10" => Some(KeyCode::F10),
        "F11" => Some(KeyCode::F11), "F12" => Some(KeyCode::F12),
        _ => None,
    }
}

#[derive(Serialize, Deserialize)]
struct KeyBindingsSerde {
    up: String, down: String, left: String,
    right: String, interact: String, pause: String,
}

impl Serialize for KeyBindings {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        KeyBindingsSerde {
            up:       keycode_to_str(self.up).to_string(),
            down:     keycode_to_str(self.down).to_string(),
            left:     keycode_to_str(self.left).to_string(),
            right:    keycode_to_str(self.right).to_string(),
            interact: keycode_to_str(self.interact).to_string(),
            pause:    keycode_to_str(self.pause).to_string(),
        }
        .serialize(s)
    }
}

impl<'de> Deserialize<'de> for KeyBindings {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = KeyBindingsSerde::deserialize(d)?;
        let def = KeyBindings::default();
        Ok(KeyBindings {
            up:       str_to_keycode(&s.up).unwrap_or(def.up),
            down:     str_to_keycode(&s.down).unwrap_or(def.down),
            left:     str_to_keycode(&s.left).unwrap_or(def.left),
            right:    str_to_keycode(&s.right).unwrap_or(def.right),
            interact: str_to_keycode(&s.interact).unwrap_or(def.interact),
            pause:    str_to_keycode(&s.pause).unwrap_or(def.pause),
        })
    }
}
