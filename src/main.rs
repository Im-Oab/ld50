#![windows_subsystem = "windows"]
use macroquad::prelude::*;
use macroquad::window;
use std::sync::Mutex;
use std::time::Duration;

pub mod core;
pub mod game;
pub mod scenes;

use crate::core::animation::AnimationManager;
use crate::core::scene::{GameScene, SceneTransition};
use crate::core::textures::TextureManager;

pub const ONE_FRAME: Duration = Duration::from_millis(1000 / 60);
pub const GAME_WIDTH: f32 = 300.0;
pub const GAME_HEIGHT: f32 = 800.0;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref TEXTURE_MANAGER: Mutex<TextureManager> = Mutex::new(TextureManager::new());
    static ref ANIMATION_MANAGER: Mutex<AnimationManager> = Mutex::new(AnimationManager::new());
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Grow".to_owned(),
        high_dpi: true,
        window_width: crate::GAME_WIDTH as i32,
        window_height: crate::GAME_HEIGHT as i32,
        window_resizable: false,
        fullscreen: false,
        sample_count: 1,
        icon: None,
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut scenes: Vec<Box<dyn GameScene>> = vec![];

    init(&mut scenes).await;
    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }
        update(&mut scenes);
        draw(&mut scenes);

        next_frame().await
    }
}

fn draw(scenes: &mut Vec<Box<dyn GameScene>>) {
    if let Some(active_scene) = scenes.last_mut() {
        active_scene.draw();
    }
}

fn update(scenes: &mut Vec<Box<dyn GameScene>>) {
    if let Some(active_scene) = scenes.last_mut() {
        match active_scene.update() {
            Ok(v) => match v {
                SceneTransition::None => {}
                SceneTransition::Push(s) => {
                    scenes.push(s);
                }
                SceneTransition::Pop => {
                    scenes.pop();
                }
                SceneTransition::Replace(s) => {
                    scenes.pop();
                    scenes.push(s);
                }
            },
            Err(_) => {}
        }
    }
}

async fn init(scenes: &mut Vec<Box<dyn GameScene>>) {
    {
        let mut texture_manager = crate::TEXTURE_MANAGER.lock().unwrap();
        texture_manager
            .load_from_file("hand-1", "./resources/textures/1.png")
            .await;
        texture_manager
            .load_from_file("hand-2", "./resources/textures/2.png")
            .await;
        texture_manager
            .load_from_file("hand-3", "./resources/textures/3.png")
            .await;
        texture_manager
            .load_from_file("hand-4", "./resources/textures/4.png")
            .await;
        texture_manager
            .load_from_file("hand-5", "./resources/textures/5.png")
            .await;
        texture_manager
            .load_from_file("hand-6", "./resources/textures/6.png")
            .await;
        texture_manager
            .load_from_file("hand-7", "./resources/textures/7.png")
            .await;
        texture_manager
            .load_from_file("hand-8", "./resources/textures/8.png")
            .await;

        println!("texture ids: {:?}", texture_manager.textures.keys());
    }

    {
        let mut animation_manager = crate::ANIMATION_MANAGER.lock().unwrap();
        animation_manager.add(
            "player-idle",
            vec![
                "hand-1", "hand-2", "hand-3", "hand-4", "hand-5", "hand-6", "hand-7", "hand-8",
            ],
        );

        println!("Animation names: {:?}", animation_manager.animations.keys());
    }

    let mut title_scene = scenes::title::TitleScene::new();
    title_scene.init();
    scenes.push(Box::new(title_scene));
}
