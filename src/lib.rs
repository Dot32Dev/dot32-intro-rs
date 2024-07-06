// TODO: delete_when_finished should run once, not every frame

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::*;
use std::env;

const RESTARTABLE: bool = true;
const SUBTEXT: &str = "Games";
pub const LENGTH: f32 = 1.0;
pub const FADE: f32 = 0.2;

pub struct Intro;

impl Plugin for Intro {
    fn build(&self, app: &mut App) {
        app.insert_resource(Progress { time: -0.2 })
            .insert_resource(Completed { value: false })
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    update_time,
                    update_background,
                    delete_when_finished,
                    keys,
                    update_dot32_text,
                    update_subtext_text,
                ),
            );
    }
}

#[derive(Resource)]
struct Progress {
    time: f32,
}

#[derive(Resource)]
pub struct Completed {
    pub value: bool,
}

#[derive(Component)]
struct Dot32;

#[derive(Component)]
struct Subtext;

#[derive(Component)]
struct Background;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn(Camera2dBundle::default());
    let camera = commands
        .spawn(Camera2dBundle {
            camera: Camera {
                // renders after / on top of the main camera
                order: -1,
                ..default()
            },
            ..default()
        })
        .id();
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..default()
                },
                background_color: Color::srgba(0.17, 0.17, 0.17, 1.0).into(),
                ..default()
            },
            TargetCamera(camera),
        ))
        .insert(Background)
        .insert(Name::new("Intro"))
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        SUBTEXT,
                        TextStyle {
                            font: asset_server
                                .load("fonts/PT_Sans/PTSans-Regular.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE,
                        },
                    )
                    // .with_text_alignment(TextAlignment {
                    // 	horizontal: HorizontalAlign::Center,
                    // 	vertical: VerticalAlign::Center,
                    // })
                    .with_style(Style {
                        align_self: AlignSelf::Center,
                        // position: UiRect {
                        // 	bottom: Val::Px(20.0),
                        // 	..default()
                        // },
                        bottom: Val::Px(20.0),
                        ..default()
                    }),
                )
                .insert(Name::new("Subtext"))
                .insert(Subtext);

            parent
                .spawn(
                    TextBundle::from_section(
                        "Dot32",
                        TextStyle {
                            font: asset_server
                                .load("fonts/PT_Sans/PTSans-Bold.ttf"),
                            font_size: 140.0,
                            color: Color::WHITE,
                        },
                    )
                    // .with_text_alignment(TextAlignment {
                    // 	horizontal: HorizontalAlign::Center,
                    // 	vertical: VerticalAlign::Center,
                    // })
                    .with_style(Style {
                        align_self: AlignSelf::Center,
                        // position: UiRect {
                        // 	bottom: Val::Px(0.0),
                        // 	..default()
                        // },
                        bottom: Val::Px(0.0),
                        ..default()
                    }),
                )
                .insert(Name::new("Dot32"))
                .insert(Dot32);
        });
}

fn ease_out_elastic(x: f32) -> f32 {
    let c4 = (2.0 * std::f64::consts::PI as f32) / 2.3; // edit "2.3" for effect
    2.0_f32.powf(-18.0 * x.max(0.0)) * ((x.max(0.0) * 10.0 - 0.75) * c4).sin()
        + 1.0 // edit "-18" for efefct
}

fn update_time(time: Res<Time>, mut progress: ResMut<Progress>) {
    progress.time += time.delta_seconds();
}

fn update_background(
    progress: Res<Progress>,
    mut background: Query<&mut BackgroundColor, With<Background>>,
) {
    for mut color in background.iter_mut() {
        *color = Color::srgba(
            0.17,
            0.17,
            0.17,
            ((-progress.time + LENGTH) / FADE).max(0.0).min(1.0),
        )
        .into()
    }
}

fn update_dot32_text(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    progress: ResMut<Progress>,
    mut dot32: Query<&mut Style, With<Dot32>>,
    mut color: Query<&mut Text, With<Dot32>>,
) {
    let Ok(window) = primary_window.get_single() else {
        return;
    };

    for mut style in dot32.iter_mut() {
        style.top = Val::Px(
            ease_out_elastic(progress.time) * window.height() / 2.0
                - window.height() / 2.0,
        )
    }

    for mut value in color.iter_mut() {
        value.sections[0].style.color = Color::srgba(
            1.0,
            1.0,
            1.0,
            ((-progress.time + LENGTH) / FADE).max(0.0).min(1.0),
        )
    }
}

fn update_subtext_text(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    progress: Res<Progress>,
    mut subtext: Query<&mut Style, With<Subtext>>,
    mut color: Query<&mut Text, With<Subtext>>,
) {
    let Ok(window) = primary_window.get_single() else {
        return;
    };

    for mut style in subtext.iter_mut() {
        style.left = Val::Px(
            ease_out_elastic(progress.time) * window.width() / 2.0
                - window.width() / 2.0,
        )
    }

    for mut value in color.iter_mut() {
        value.sections[0].style.color = Color::srgba(
            1.0,
            1.0,
            1.0,
            ((-progress.time + LENGTH) / FADE).max(0.0).min(1.0),
        )
    }
}

fn keys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut progress: ResMut<Progress>,
    mut exit: EventWriter<AppExit>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut window) = primary_window.get_single_mut() else {
        return;
    };

    if env::consts::OS == "macos" {
        if keyboard_input.pressed(KeyCode::SuperLeft)
            && keyboard_input.just_pressed(KeyCode::KeyW)
        {
            exit.send(AppExit::Success);
            // window.set_mode(WindowMode::Windowed);
            window.mode = WindowMode::Windowed;
        }
        if keyboard_input.pressed(KeyCode::SuperLeft)
            && keyboard_input.pressed(KeyCode::ControlLeft)
            && keyboard_input.just_pressed(KeyCode::KeyF)
        {
            println!("{:?}", window.mode);
            if window.mode == WindowMode::Windowed {
                // window.set_mode(WindowMode::BorderlessFullscreen);
                window.mode = WindowMode::BorderlessFullscreen;
            } else if window.mode == WindowMode::BorderlessFullscreen {
                // window.set_mode(WindowMode::Windowed);
                window.mode = WindowMode::Windowed;
            }
        }

        if RESTARTABLE {
            if keyboard_input.pressed(KeyCode::SuperLeft)
                && keyboard_input.just_pressed(KeyCode::KeyR)
            {
                progress.time = 0.0
            }
        }
    }
    if env::consts::OS == "windows" {
        if keyboard_input.just_pressed(KeyCode::F11) {
            if window.mode == WindowMode::Windowed {
                // window.set_mode(WindowMode::BorderlessFullscreen);
                window.mode = WindowMode::BorderlessFullscreen;
            } else if window.mode == WindowMode::BorderlessFullscreen {
                // window.set_mode(WindowMode::Windowed);
                window.mode = WindowMode::Windowed;
            }
        }

        if RESTARTABLE {
            if keyboard_input.pressed(KeyCode::ControlLeft)
                && keyboard_input.just_pressed(KeyCode::KeyR)
            {
                progress.time = 0.0
            }
        }
    }
}

fn delete_when_finished(
    progress: Res<Progress>,
    mut completed: ResMut<Completed>,
    intro: Query<Entity, With<Background>>,
    mut commands: Commands,
) {
    for intro_entity in intro.iter() {
        if progress.time > LENGTH + FADE {
            commands.entity(intro_entity).despawn_recursive();
            completed.value = true;
        }
    }
}
