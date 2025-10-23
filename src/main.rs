use std::f32::consts::{FRAC_PI_4, PI};

use bevy::{light::DirectionalLightShadowMap, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_light_direction)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));

    let gltf = asset_server.load("FN_Scar_rigged_textured.glb");
    commands.insert_resource(WeaponScene(gltf));
}

fn spawn_gltf_objects(
    mut commands: Commands,
    weapon_scene: Res<WeaponScene>,
    gltf_assets: Res<Assets<Gltf>>,
    mut loaded: Local<bool>,
) {
    if *loaded {
        return;
    }

    let Some(gltf) = gltf_assets.get(&weapon_scene.0) else {
        return;
    };

    *loaded = true;

    commands.spawn(mesh);
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_secs() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}

#[derive(Resource)]
struct WeaponScene(Handle<Gltf>);

#[derive(Component)]
struct Weapon {}

#[derive(Component)]
struct Foregrip {}
#[derive(Component)]
struct Scope {}
#[derive(Component)]
struct Mag {}

#[derive(Component)]
#[relationship(relationship_target = WeaponAttachments)]
struct AttachedToWeapon(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = AttachedToWeapon, linked_spawn)]
struct WeaponAttachments(Vec<Entity>);
