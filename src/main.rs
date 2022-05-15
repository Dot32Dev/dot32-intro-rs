use bevy::prelude::*;
use bevy::window::*;

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: "Dot32 intro".to_string(),
			width: 800.,
			height: 600.,
			present_mode: PresentMode::Fifo, // Vesync enabled, replace Fifo with Mailbox for no vsync
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.insert_resource(Progress { time: 0.0 })
		.add_startup_system(setup)
		.add_system(update_text)
		.run();
}

struct Progress{ 
	time: f32
}

#[derive(Component)]
struct Dot32;

fn setup(mut commands: Commands , asset_server: Res<AssetServer>) {
	commands.spawn_bundle(UiCameraBundle::default());
	commands.spawn_bundle(NodeBundle {
		style: Style {
			size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
			position_type: PositionType::Absolute,
			justify_content: JustifyContent::Center,
			..default()
	},
	color: Color::rgb(0.17, 0.17, 0.17).into(),
	..default()
	}).with_children(|parent| {
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
					font_size: 100.0,
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
	});
}

fn ease_out_elastic(x: f32) -> f32 {
	let c4 = (2.0*std::f64::consts::PI as f32) / 2.3; // edit "2.3" for effect
	2.0_f32.powf(-18.0*x) * ((x*10.0 - 0.75)*c4).sin() + 1.0 // edit "-18" for efefct
}

fn update_text(
	time: Res<Time>, 
	windows: Res<Windows>, 
	mut progress: ResMut<Progress>, 
	mut dot32: Query<&mut Style, With<Dot32>>,
) {
	progress.time += time.delta_seconds();
	
	let window = windows.get_primary().unwrap();

	for mut style in dot32.iter_mut() {
		style.position.top = Val::Px(ease_out_elastic(progress.time)*window.height()/2.0-window.height()/2.0)
	}
}