use std::env;
use bevy::prelude::*;
use bevy::window::*;
use bevy::app::AppExit;

const RESTARTABLE: bool = false;
const SUBTEXT: &str = "Games";
pub const LENGTH: f32 = 1.0;
pub const FADE: f32 = 0.2;

pub struct Intro;

impl Plugin for Intro {
	fn build(&self, app: &mut App) {
			app.insert_resource(Progress { time: -0.2 })
				.add_startup_system(setup)
				.add_system(update_time)
				.add_system(update_background)
				.add_system(update_dot32_text)
				.add_system(update_subtext_text)
				.add_system(delete_when_finished)
				.add_system(keys)
				.run();
	}
}

struct Progress{ 
	time: f32
}

#[derive(Component)]
struct Dot32;

#[derive(Component)]
struct Subtext;

#[derive(Component)]
struct Background;

fn setup(mut commands: Commands , asset_server: Res<AssetServer>) {
	commands.spawn_bundle(UiCameraBundle::default());
	commands.spawn_bundle(NodeBundle {
		style: Style {
			size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
			position_type: PositionType::Absolute,
			justify_content: JustifyContent::Center,
			flex_direction: FlexDirection::ColumnReverse,
			..default()
		},
		color: Color::rgba(0.17, 0.17, 0.17, 1.0).into(),
		..default()
	}).insert(Background).with_children(|parent| {
		parent.spawn_bundle(TextBundle {
			style: Style {
					align_self: AlignSelf::Center,
					position: Rect {
							bottom: Val::Px(0.0),
							// bottom: Val::Percent(0.5),
							// right: Val::Percent(50.0),
							// left: Val::Percent(50.0),
							..Default::default()
					},
					..Default::default()
			},
			// Use the `Text::with_section` constructor
			text: Text::with_section(
				// Accepts a `String` or any type that converts into a `String`, such as `&str`
				"Dot32",
				TextStyle {
					font: asset_server.load("fonts/PT_Sans/PTSans-Bold.ttf"),
					font_size: 140.0,
					color: Color::WHITE,
				},
				// Note: You can use `Default::default()` in place of the `TextAlignment`
				TextAlignment {
					horizontal: HorizontalAlign::Center,
					vertical: VerticalAlign::Center,
				},
				// Default::default(),
			),
			..Default::default()
		}).insert(Dot32);
	}).with_children(|parent| {
		parent.spawn_bundle(TextBundle {
			style: Style {
					align_self: AlignSelf::Center,
					position: Rect {
							bottom: Val::Px(20.0),
							// bottom: Val::Percent(0.5),
							// right: Val::Percent(50.0),
							// left: Val::Percent(50.0),
							..Default::default()
					},
					..Default::default()
			},
			// Use the `Text::with_section` constructor
			text: Text::with_section(
				// Accepts a `String` or any type that converts into a `String`, such as `&str`
				SUBTEXT,
				TextStyle {
					font: asset_server.load("fonts/PT_Sans/PTSans-Regular.ttf"),
					font_size: 50.0,
					color: Color::WHITE,
				},
				// Note: You can use `Default::default()` in place of the `TextAlignment`
				TextAlignment {
					horizontal: HorizontalAlign::Center,
					vertical: VerticalAlign::Center,
				},
				// Default::default(),
			),
			..Default::default()
		}).insert(Subtext);
	});
}

fn ease_out_elastic(x: f32) -> f32 {
	let c4 = (2.0*std::f64::consts::PI as f32) / 2.3; // edit "2.3" for effect
	2.0_f32.powf(-18.0*x.max(0.0)) * ((x.max(0.0)*10.0 - 0.75)*c4).sin() + 1.0 // edit "-18" for efefct
}

fn update_time(time: Res<Time>, mut progress: ResMut<Progress>, ) {
	progress.time += time.delta_seconds();
}

fn update_background(progress: Res<Progress>, mut background: Query<&mut UiColor, With<Background>>) {
	for mut color in background.iter_mut() {
		*color = Color::rgba(0.17, 0.17, 0.17, ((-progress.time+LENGTH)/FADE).max(0.0).min(1.0)).into()
	}
}

fn update_dot32_text(
	windows: Res<Windows>, 
	progress: ResMut<Progress>, 
	mut dot32: Query<&mut Style, With<Dot32>>,
	mut color: Query<&mut Text, With<Dot32>>,
) {
	let window = windows.get_primary().unwrap();

	for mut style in dot32.iter_mut() {
		style.position.top = Val::Px(ease_out_elastic(progress.time)*window.height()/2.0-window.height()/2.0)
	}

	for mut value in color.iter_mut() {
		value.sections[0].style.color = Color::rgba(1.0, 1.0, 1.0, ((-progress.time+LENGTH)/FADE).max(0.0).min(1.0))
	}
}

fn update_subtext_text(
	windows: Res<Windows>, 
	progress: Res<Progress>, 
	mut subtext: Query<&mut Style, With<Subtext>>,
	mut color: Query<&mut Text, With<Subtext>>,
) {
	let window = windows.get_primary().unwrap();

	for mut style in subtext.iter_mut() {
		style.position.left = Val::Px(ease_out_elastic(progress.time)*window.width()/2.0-window.width()/2.0)
	}

	for mut value in color.iter_mut() {
		value.sections[0].style.color = Color::rgba(1.0, 1.0, 1.0, ((-progress.time+LENGTH)/FADE).max(0.0).min(1.0))
	}
}

fn keys(
	keyboard_input: Res<Input<KeyCode>>,
	mut progress: ResMut<Progress>, 
	mut exit: EventWriter<AppExit>,
	mut windows: ResMut<Windows>,
) {
	let window = windows.get_primary_mut().unwrap();

	if env::consts::OS == "macos" {
		if keyboard_input.pressed(KeyCode::LWin) && keyboard_input.just_pressed(KeyCode::W) {
				exit.send(AppExit);
				window.set_mode(WindowMode::Windowed);
		}
		if keyboard_input.pressed(KeyCode::LWin) 
		&& keyboard_input.pressed(KeyCode::LControl) 
		&& keyboard_input.just_pressed(KeyCode::F) {
			println!("{:?}", window.mode());
			if window.mode() == WindowMode::Windowed {
				window.set_mode(WindowMode::BorderlessFullscreen);
			} else if window.mode() == WindowMode::BorderlessFullscreen {
				window.set_mode(WindowMode::Windowed);
			}
		}

		if RESTARTABLE {
			if keyboard_input.pressed(KeyCode::LWin) && keyboard_input.just_pressed(KeyCode::R) {
				progress.time = 0.0
			}
		}
	}
	if env::consts::OS == "windows" {
		if keyboard_input.just_pressed(KeyCode::F11) {
			if window.mode() == WindowMode::Windowed {
				window.set_mode(WindowMode::BorderlessFullscreen);
			} else if window.mode() == WindowMode::BorderlessFullscreen {
				window.set_mode(WindowMode::Windowed);
			}
		}

		if RESTARTABLE {
			if keyboard_input.pressed(KeyCode::LControl) && keyboard_input.just_pressed(KeyCode::R) {
				progress.time = 0.0
			}
		}
	}
}

fn delete_when_finished(
	progress: Res<Progress>, 
	intro: Query<Entity, With<Background>>,
	mut commands: Commands,
) {
	for intro_entity in intro.iter() {
		if progress.time > LENGTH + FADE {
			commands.entity(intro_entity).despawn_recursive(); 
		}
	}
}