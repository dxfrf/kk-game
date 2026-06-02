use bevy::prelude::*;
use bevy_egui::egui;
use crate::input::{KeyBindings, keycode_to_str};
use super::UiScreen;

#[derive(Resource, Default)]
pub struct KeybindWaiting(pub Option<BindingSlot>);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BindingSlot { Up, Down, Left, Right, Interact, Pause }

pub fn keybinds_ui(
    ui: &mut egui::Ui,
    ui_screen: &mut UiScreen,
    bindings: &mut KeyBindings,
    waiting: &mut KeybindWaiting,
    keys: &ButtonInput<KeyCode>,
) {
    // Capture key if waiting
    if let Some(slot) = waiting.0 {
        if let Some(key) = keys.get_just_pressed().next().copied() {
            match slot {
                BindingSlot::Up       => bindings.up = key,
                BindingSlot::Down     => bindings.down = key,
                BindingSlot::Left     => bindings.left = key,
                BindingSlot::Right    => bindings.right = key,
                BindingSlot::Interact => bindings.interact = key,
                BindingSlot::Pause    => bindings.pause = key,
            }
            waiting.0 = None;
        }
    }

    ui.set_min_width(280.0);
    ui.heading("⌨  Управление");
    ui.add_space(8.0);

    let rows: &[(BindingSlot, &str, KeyCode)] = &[
        (BindingSlot::Up,       "Вверх",    bindings.up),
        (BindingSlot::Down,     "Вниз",     bindings.down),
        (BindingSlot::Left,     "Влево",    bindings.left),
        (BindingSlot::Right,    "Вправо",   bindings.right),
        (BindingSlot::Interact, "Действие", bindings.interact),
        (BindingSlot::Pause,    "Пауза",    bindings.pause),
    ];

    egui::Grid::new("binds")
        .num_columns(2)
        .spacing([24.0, 6.0])
        .show(ui, |ui| {
            for (slot, label, key) in rows {
                ui.label(*label);
                let is_waiting = waiting.0 == Some(*slot);
                let btn_text = if is_waiting {
                    "[ нажмите клавишу... ]".to_string()
                } else {
                    keycode_to_str(*key).to_string()
                };
                if ui.button(btn_text).clicked() && !is_waiting {
                    waiting.0 = Some(*slot);
                }
                ui.end_row();
            }
        });

    ui.add_space(12.0);
    if ui.button("← Назад").clicked() {
        waiting.0 = None;
        *ui_screen = UiScreen::Main;
    }
}
