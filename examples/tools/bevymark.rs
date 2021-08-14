use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use rand::Rng;

const BIRDS_PER_SECOND: u32 = 1000;
const BASE_COLOR: Color = Color::rgb(5.0, 5.0, 5.0);
const GRAVITY: TReal = -9.8 * 100.0;
const MAX_VELOCITY: TReal = 750.;
const BIRD_SCALE: TReal = 0.15;
const HALF_BIRD_SIZE: TReal = 256. * BIRD_SCALE * 0.5;

struct BevyCounter {
    pub count: u128,
}

struct Bird {
    velocity: TVec3,
}

struct BirdMaterial(Handle<ColorMaterial>);

impl FromWorld for BirdMaterial {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut color_materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        BirdMaterial(color_materials.add(asset_server.load("branding/icon.png").into()))
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "BevyMark".to_string(),
            width: 800.,
            height: 600.,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(BevyCounter { count: 0 })
        .init_resource::<BirdMaterial>()
        .add_startup_system(setup.system())
        .add_system(mouse_handler.system())
        .add_system(movement_system.system())
        .add_system(collision_system.system())
        .add_system(counter_system.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Bird Count: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 1.0, 0.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 1.0, 1.0),
                    },
                },
                TextSection {
                    value: "\nAverage FPS: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 1.0, 0.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 1.0, 1.0),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

#[allow(clippy::too_many_arguments)]
fn mouse_handler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mouse_button_input: Res<Input<MouseButton>>,
    window: Res<WindowDescriptor>,
    mut bird_material: ResMut<BirdMaterial>,
    mut counter: ResMut<BevyCounter>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut rnd = rand::thread_rng();
        let color = gen_color(&mut rnd);

        let texture_handle = asset_server.load("branding/icon.png");

        bird_material.0 = materials.add(ColorMaterial {
            color: BASE_COLOR * color,
            texture: Some(texture_handle),
        });
    }

    if mouse_button_input.pressed(MouseButton::Left) {
        let spawn_count = (BIRDS_PER_SECOND as f32 * time.delta_seconds()) as u128;
        let bird_x = (window.width / -2.).default_precision() + HALF_BIRD_SIZE;
        let bird_y = (window.height / 2.).default_precision() - HALF_BIRD_SIZE;

        for count in 0..spawn_count {
            let bird_z = (counter.count + count) as TReal * 0.00001;
            commands
                .spawn_bundle(SpriteBundle {
                    material: bird_material.0.clone(),
                    transform: Transform {
                        translation: TVec3::new(bird_x, bird_y, bird_z),
                        scale: TVec3::splat(BIRD_SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Bird {
                    velocity: TVec3::new(
                        rand::random::<TReal>() * MAX_VELOCITY - (MAX_VELOCITY * 0.5),
                        0.,
                        0.,
                    ),
                });
        }

        counter.count += spawn_count;
    }
}

fn movement_system(time: Res<Time>, mut bird_query: Query<(&mut Bird, &mut Transform)>) {
    for (mut bird, mut transform) in bird_query.iter_mut() {
        transform.translation.x += bird.velocity.x * time.delta_seconds().default_precision();
        transform.translation.y += bird.velocity.y * time.delta_seconds().default_precision();
        bird.velocity.y += GRAVITY * time.delta_seconds().default_precision();
    }
}

fn collision_system(window: Res<WindowDescriptor>, mut bird_query: Query<(&mut Bird, &Transform)>) {
    let half_width = window.width as TReal * 0.5;
    let half_height = window.height as TReal * 0.5;

    for (mut bird, transform) in bird_query.iter_mut() {
        let x_vel = bird.velocity.x;
        let y_vel = bird.velocity.y;
        let x_pos = transform.translation.x;
        let y_pos = transform.translation.y;

        if (x_vel > 0. && x_pos + HALF_BIRD_SIZE > half_width)
            || (x_vel <= 0. && x_pos - HALF_BIRD_SIZE < -(half_width))
        {
            bird.velocity.x = -x_vel;
        }
        if y_vel < 0. && y_pos - HALF_BIRD_SIZE < -half_height {
            bird.velocity.y = -y_vel;
        }
    }
}

fn counter_system(
    diagnostics: Res<Diagnostics>,
    counter: Res<BevyCounter>,
    mut query: Query<&mut Text>,
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            for mut text in query.iter_mut() {
                text.sections[1].value = format!("{}", counter.count);
                text.sections[3].value = format!("{:.2}", average);
            }
        }
    };
}

/// Generate a color modulation
///
/// Because there is no `Mul<Color> for Color` instead `[f32; 3]` is
/// used.
fn gen_color(rng: &mut impl Rng) -> [f32; 3] {
    let r = rng.gen_range(0.2..1.0);
    let g = rng.gen_range(0.2..1.0);
    let b = rng.gen_range(0.2..1.0);
    let v = Vec3::new(r, g, b);
    v.normalize().into()
}
