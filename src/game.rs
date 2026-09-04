//! High-level game loop, state transitions, and toolkit integration.

use crate::data::GameData;
use crate::state::{migrate_save_value, GameSession, SaveData};
use crate::ui::{self, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::camera::{Camera2D, Camera2DConfig, CameraBounds};
use macroquad_toolkit::events::EventBus;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};
use macroquad_toolkit::persistence::{
    delete_slot, get_save_slots, load_from_slot_with_migration, save_to_slot_with_version,
    slot_exists,
};
use macroquad_toolkit::prelude::{begin_virtual_ui_frame, dark, end_virtual_ui_frame, InputState};

pub struct Game {
    in_menu: bool,
    session_started: bool,
    data: GameData,
    session: GameSession,
    assets: AssetManager,
    notifications: NotificationManager,
    camera: Camera2D,
    events: EventBus<UiAction>,
    save_exists: bool,
    save_slots: Vec<String>,
}

impl Game {
    pub async fn new() -> Self {
        let data = GameData::load().unwrap_or_else(|err| {
            panic!("Occupational Hazard embedded data failed to load: {}", err);
        });

        let mut assets = AssetManager::new();
        let placeholder = Image::gen_image_color(16, 16, Color::new(0.75, 0.2, 0.8, 1.0));
        assets.set_placeholder_texture_direct(Texture2D::from_image(&placeholder));
        assets.load_texture_configs(&data.texture_manifest).await;
        let notifications = NotificationManager::new();

        let session = GameSession::new(&data.config);
        let camera = Camera2D::with_config(
            vec2(0.0, 0.0),
            1.0,
            Camera2DConfig {
                drag_button: Some(MouseButton::Right),
                min_zoom: 0.75,
                max_zoom: 1.75,
                bounds: Some(CameraBounds::new(vec2(-240.0, -160.0), vec2(240.0, 160.0))),
                ..Default::default()
            },
        );

        let mut game = Self {
            in_menu: true,
            session_started: false,
            data,
            session,
            assets,
            notifications,
            camera,
            events: EventBus::new(),
            save_exists: false,
            save_slots: Vec::new(),
        };
        game.refresh_save_state();
        game
    }

    pub fn update(&mut self, dt: f32) {
        self.notifications.update(dt);
        let actions: Vec<UiAction> = self.events.drain().collect();
        for action in actions {
            self.apply_action(action);
        }
        if self.in_menu {
            return;
        }
        self.session.update_energy(&self.data.config, dt);

        let input = InputState::capture();
        if input.escape_pressed {
            self.events.push(UiAction::OpenMenu);
        }
        if input.space_pressed {
            if let Some((id, _)) = self.data.actions.iter().next() {
                self.events.push(UiAction::RunAction(id.clone()));
            }
        }
        if is_key_pressed(KeyCode::S) {
            self.events.push(UiAction::Save);
        }
        if is_key_pressed(KeyCode::L) {
            self.events.push(UiAction::Load);
        }
        if let Some((dx, dy)) = ui::tile_move_from_keys() {
            self.session.move_selection(dx, dy);
        }

        self.camera.update(dt, false);

        let actions: Vec<UiAction> = self.events.drain().collect();
        for action in actions {
            self.apply_action(action);
        }
    }

    pub fn draw(&mut self) {
        clear_background(dark::BACKGROUND);

        let virtual_ui = begin_virtual_ui_frame(ui::LOGICAL_WIDTH, ui::LOGICAL_HEIGHT);
        let actions = if self.in_menu {
            ui::draw_title(&virtual_ui, self.save_exists, self.session_started)
        } else {
            let ctx = UiContext {
                data: &self.data,
                session: &self.session,
                save_exists: self.save_exists,
                save_slots: &self.save_slots,
                loaded_assets: self.assets.len(),
                camera_target: self.camera.target,
                camera_zoom: self.camera.zoom,
                ui: &virtual_ui,
            };

            ui::draw_game_ui(ctx)
        };
        end_virtual_ui_frame();

        for action in actions {
            self.events.push(action);
        }

        self.notifications
            .draw_with_config(&NotificationRenderConfig {
                anchor: NotificationAnchor::BottomRight,
                ..Default::default()
            });
    }

    fn apply_action(&mut self, action: UiAction) {
        match action {
            UiAction::OpenMenu => self.in_menu = true,
            UiAction::Resume => self.in_menu = false,
            UiAction::ZoomIn => self.camera.zoom = (self.camera.zoom + 0.1).min(1.75),
            UiAction::ZoomOut => self.camera.zoom = (self.camera.zoom - 0.1).max(0.75),
            UiAction::ResetCamera => {
                self.camera.target = Vec2::ZERO;
                self.camera.zoom = 1.0;
            }
            UiAction::NewGame => {
                self.in_menu = false;
                self.session_started = true;
                self.camera.target = Vec2::ZERO;
                self.camera.zoom = 1.0;
                self.session = GameSession::new(&self.data.config);
                self.notifications
                    .info("Started a fresh development session");
            }
            UiAction::Save => self.save_game(),
            UiAction::Load => self.load_game(),
            UiAction::DeleteSave => self.delete_save(),
            UiAction::RunAction(id) => self.run_data_action(&id),
            UiAction::SelectTile(tile) => {
                self.session.select_tile(tile);
                self.notifications
                    .info(format!("Selected tile {}, {}", tile.x, tile.y));
            }
        }
    }

    fn run_data_action(&mut self, action_id: &str) {
        let Some(action) = self.data.actions.get(action_id) else {
            self.notifications
                .warning(format!("Unknown action: {}", action_id));
            return;
        };

        if self.session.apply_action(action) {
            self.notifications
                .success(format!("{} complete", action.name));
        } else {
            self.notifications.warning("Not enough energy");
        }
    }

    fn save_game(&mut self) {
        let save = self.session.to_save(&self.data.config.version);
        match save_to_slot_with_version(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &save,
            &self.data.config.version,
        ) {
            Ok(()) => {
                self.notifications.success("Saved with toolkit save slots");
                self.refresh_save_state();
            }
            Err(err) => self.notifications.danger(format!("Save failed: {}", err)),
        }
    }

    fn load_game(&mut self) {
        let loaded: Result<SaveData, String> = load_from_slot_with_migration(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &self.data.config.version,
            |version, value| migrate_save_value(version, value, &self.data.config),
        );

        match loaded {
            Ok(save) => {
                self.in_menu = false;
                self.session_started = true;
                self.session = GameSession::from_save(save);
                self.notifications
                    .success("Loaded save with migration support");
                self.refresh_save_state();
            }
            Err(err) => self.notifications.warning(format!("Load failed: {}", err)),
        }
    }

    fn delete_save(&mut self) {
        match delete_slot(&self.data.config.game_name, &self.data.config.save_slot) {
            Ok(()) => {
                self.notifications
                    .info("Deleted Occupational Hazard save slot");
                self.refresh_save_state();
            }
            Err(err) => self.notifications.danger(format!("Delete failed: {}", err)),
        }
    }

    fn refresh_save_state(&mut self) {
        self.save_exists = slot_exists(&self.data.config.game_name, &self.data.config.save_slot);
        self.save_slots = get_save_slots(&self.data.config.game_name);
    }

    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.session = GameSession::new(&self.data.config);
        self.camera.target = Vec2::ZERO;
        self.camera.zoom = 1.0;
        self.notifications = NotificationManager::new();
        self.save_exists = false;
        self.save_slots.clear();
        self.in_menu = match scene {
            "title" => true,
            "gameplay" => false,
            other => panic!("Unknown capture scene: {other}"),
        };
        self.session_started = !self.in_menu;
    }
}
