//! Helper functions for creating explosions.

use bevy::prelude::*;

pub struct ExplosionsPlugin;

impl Plugin for ExplosionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(create_explosions.system());
        app.add_system(update_explosions.system());
        app.add_startup_system(setup.system());
    }
}

/// Component that indicates an explosion should be created at the location of the entity.
pub struct CreateExplosion {
    pub translation: Vec3
}

fn update_explosions(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (entity, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            if sprite.index as usize == texture_atlas.textures.len()
            {
                // if we are at the final frame, delete the explosion
                commands.entity(entity).despawn_recursive();
            } else {
                // Otherwise advance the explosion frames.
                sprite.index += 1;
            }
        }
    }
}

fn create_explosions(
    mut commands: Commands,
    prefabs: Res<ExplosionPrefabs>,
    query: Query<(Entity, &CreateExplosion)>
) {
    for (entity, explosion) in query.iter() {

        // despawn the creation command
        commands.entity(entity).despawn_recursive();

        // Spawn an explosion
        commands.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: prefabs.small_explosion.atlas.clone(),
                    transform: Transform::from_translation(explosion.translation),
                    ..Default::default()
                })
        .insert(Timer::from_seconds(0.1, true));
    }
}

struct ExplosionData {
    atlas: Handle<TextureAtlas>
}

struct ExplosionPrefabs {
    small_explosion: ExplosionData
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("art/small_explosion.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 8, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let small_explosion = ExplosionData {
        atlas: texture_atlas_handle,
    };

    let resources = ExplosionPrefabs {
        small_explosion
    };

    commands.insert_resource(resources);
}