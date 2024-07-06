use bevy::prelude::*;
use bevy::window::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.7, 0.55, 0.41)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dot32 intro".into(),
                resolution: WindowResolution::new(800., 600.),
                // Vsync enabled, replace Fifo with Mailbox for no vsync
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(dot32_intro::Intro)
        .run();
}
