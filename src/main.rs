//! Occupational Hazard runtime entry point.

use macroquad::prelude::*;
use macroquad_toolkit::capture;

mod contracts;
mod game;
mod reports;
mod review;
mod services;
mod simulation;
mod tutorial;

mod ui;

use game::Game;

fn window_conf() -> Conf {
    let mut config =
        capture::capture_window_conf("OCCUPATIONAL_HAZARD", "Occupational Hazard", 1280, 720);
    config.fullscreen = !capture::capture_requested("OCCUPATIONAL_HAZARD");
    config.window_resizable = true;
    config
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    // The capture manifest selects deterministic title and gameplay scenes.
    if let Some(configs) = capture::CaptureConfig::all_from_env("OCCUPATIONAL_HAZARD") {
        for config in configs {
            game.begin_capture_scene(&config.scene);
            capture::run_capture_once(&config, |dt| {
                game.update(dt);
                game.draw();
            })
            .await;
        }
        return;
    }

    loop {
        let dt = get_frame_time().min(0.1);
        game.update(dt);
        game.draw();
        #[cfg(target_os = "windows")]
        if game.exit_requested {
            return;
        }
        next_frame().await;
    }
}
