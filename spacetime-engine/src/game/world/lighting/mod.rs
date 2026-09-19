//! Shared outdoor lighting for Loo Cast world compositions.

use bevy::prelude::*;

pub(super) fn configure(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(0.34, 0.50, 0.76)))
        .add_systems(Startup, setup_environment_lighting);
}

fn setup_environment_lighting(mut commands: Commands, mut ambient: ResMut<GlobalAmbientLight>) {
    ambient.color = Color::srgb(0.72, 0.80, 1.0);
    ambient.brightness = 140.0;

    commands.spawn((
        Name::new("Sun"),
        DirectionalLight {
            color: Color::srgb(1.0, 0.94, 0.84),
            illuminance: 30_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.90, -0.65, 0.0)),
    ));
}
