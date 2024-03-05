use bevy::prelude::*;

#[derive(Clone, Copy, Component)]
#[derive(Default)]
pub struct Target(pub Option<Entity>);

/// Indicates that an entity should use the target chosen by it's parent.
#[derive(Clone, Copy, Component, Default)]
pub struct InheritTargetFromParent;

pub fn copy_targets_from_parents(
     query: Query<(Entity, &Parent), With<InheritTargetFromParent>>,
    mut targetter_query: Query<&mut Target>
) {
    for (entity, parent) in query.iter() {
        let parent_target = match targetter_query.get(parent.get()) {
            Ok(opt) => opt.0,
            Err(_) => None
        };
        if let Ok(mut my_target) = targetter_query.get_mut(entity) {
            my_target.0 = parent_target;
        }
    }
}