use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{
    input::KeyBindings,
    persistence::{save_game, LoadRequest, PlayerSave},
    player::Player,
    state::GameState,
};
use super::{keybinds::{KeybindWaiting, keybinds_ui}, UiScreen};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiScreen>()
            .init_resource::<KeybindWaiting>()
            .add_systems(Update, pause_toggle)
            .add_systems(Update, pause_ui.run_if(in_state(GameState::Paused)));
    }
}

fn pause_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<KeyBindings>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ui_screen: ResMut<UiScreen>,
    waiting: Res<KeybindWaiting>,
) {
    // Don't toggle while capturing a keybind
    if waiting.0.is_some() { return; }
    if keys.just_pressed(bindings.pause) {
        match state.get() {
            GameState::Playing => {
                *ui_screen = UiScreen::Main;
                next_state.set(GameState::Paused);
            }
            GameState::Paused => next_state.set(GameState::Playing),
        }
    }
}

fn pause_ui(
    mut contexts: EguiContexts,
    mut ui_screen: ResMut<UiScreen>,
    mut next_state: ResMut<NextState<GameState>>,
    mut bindings: ResMut<KeyBindings>,
    mut waiting: ResMut<KeybindWaiting>,
    mut load_req: ResMut<LoadRequest>,
    player_q: Query<&Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let ctx = contexts.ctx_mut();

    // Dark overlay
    egui::Area::new("overlay".into())
        .fixed_pos([0.0, 0.0])
        .order(egui::Order::Background)
        .show(ctx, |ui| {
            ui.painter().rect_filled(
                ui.ctx().screen_rect(),
                0.0,
                egui::Color32::from_black_alpha(160),
            );
        });

    egui::Window::new("Корявая Игра ©")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| match *ui_screen {
            UiScreen::Main => main_menu(
                ui,
                &mut ui_screen,
                &mut next_state,
                &bindings,
                &mut load_req,
                &player_q,
            ),
            UiScreen::Keybinds => keybinds_ui(
                ui,
                &mut ui_screen,
                &mut bindings,
                &mut waiting,
                &keys,
            ),
        });
}

fn main_menu(
    ui: &mut egui::Ui,
    ui_screen: &mut UiScreen,
    next_state: &mut NextState<GameState>,
    bindings: &KeyBindings,
    load_req: &mut LoadRequest,
    player_q: &Query<&Transform, With<Player>>,
) {
    ui.set_min_width(220.0);
    ui.add_space(6.0);

    if ui.add_sized([220.0, 32.0], egui::Button::new("▶  Продолжить")).clicked() {
        next_state.set(GameState::Playing);
    }

    ui.separator();

    if ui.add_sized([220.0, 32.0], egui::Button::new("💾  Сохранить")).clicked() {
        let save = player_q
            .get_single()
            .map(|tf| PlayerSave { x: tf.translation.x, y: tf.translation.y })
            .unwrap_or_default();
        save_game(&save, bindings);
    }

    if ui.add_sized([220.0, 32.0], egui::Button::new("📂  Загрузить")).clicked() {
        load_req.0 = true;
        next_state.set(GameState::Playing);
    }

    ui.separator();

    if ui.add_sized([220.0, 32.0], egui::Button::new("⌨  Управление")).clicked() {
        *ui_screen = UiScreen::Keybinds;
    }

    ui.separator();

    if ui.add_sized([220.0, 32.0], egui::Button::new("✖  Выйти")).clicked() {
        std::process::exit(0);
    }

    ui.add_space(6.0);
}
