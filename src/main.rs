use bevy::{gltf::Gltf, prelude::*};
use bevy_editor_pls::EditorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_startup_system(init)
        .add_system(loading_done)
        .add_system(assign_components)
        .add_system(boat)
        .add_system(rotate)
        .add_system(ladder)
        .run();
}

fn init(mut commands: Commands, ass: Res<AssetServer>) {
    println!("Loading scene");
    let load = ass.load("test.glb");
    commands.spawn().insert(MyScene(load.clone()));
    commands.insert_resource(MyScene(load));
    println!("Scene loaded");
}

#[derive(Component)]
struct MyScene(Handle<Gltf>);

fn loading_done(
    mut commands: Commands,
    scene: Query<(Entity, &MyScene)>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    let (entity, scene) = match scene.get_single() {
        Ok(scene) => scene,
        Err(_) => return,
    };
    if let Some(gltf) = gltf_assets.get(&scene.0) {
        println!("Spawning scene");
        commands.spawn_scene(gltf.scenes[0].clone());
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
struct Boat;

#[derive(Component)]
struct Rotating;

#[derive(Component)]
struct Ladder;

fn assign_components(mut commands: Commands, entities: Query<(Entity, &Name), Added<Name>>) {
    for (entity, name) in entities.iter() {
        println!("Spawned {}", name.as_str());
        match name.as_str() {
            "boat" => {
                commands.entity(entity).insert(Boat);
            }
            "waterwheel_waterwheel" | "windmill_windmill" => {
                commands.entity(entity).insert(Rotating);
            }
            "ladder" => {
                commands.entity(entity).insert(Ladder);
            }
            _ => println!("Unassigned mesh name"),
        };
    }
}

fn boat(time: Res<Time>, mut boats: Query<&mut Transform, With<Boat>>) {
    for mut transform in boats.iter_mut() {
        transform.translation.z += time.delta_seconds() * 10.0;
        if transform.translation.z >= 20.0 {
            transform.translation.z = 0.0;
        }
    }
}

fn rotate(time: Res<Time>, mut rotating: Query<&mut Transform, With<Rotating>>) {
    for mut transform in rotating.iter_mut() {
        transform.rotate(Quat::from_euler(
            EulerRot::XYZ,
            1.0 * time.delta_seconds(),
            0.0,
            0.0,
        ));
    }
}

fn ladder(time: Res<Time>, mut ladders: Query<&mut Transform, With<Ladder>>) {
    for mut transform in ladders.iter_mut() {
        transform.scale.z += 10.0 * time.delta_seconds();
        if transform.scale.z >= 20.0 {
            transform.scale.z = 0.0;
        }
    }
}
