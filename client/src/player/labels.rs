use bevy::prelude::*;

use super::ViewMode;

#[derive(Component, Debug)]
pub struct PlayerLabel {
    pub entity: Entity,
}

pub fn player_labels_system(
    camera: Single<(&mut Camera, &GlobalTransform), With<Camera>>,
    mut labels: Query<(&mut Node, &mut Visibility, &PlayerLabel)>,
    labeled: Query<&GlobalTransform>,
    view: Res<ViewMode>,
) {
    let (camera, camera_global_transform) = camera.into_inner();

    let view_mode = *view;

    if view_mode == ViewMode::FirstPerson {
        for (_, mut vis, _label) in &mut labels.iter_mut() {
            *vis = Visibility::Hidden;
        }
    } else {
        for (_, mut vis, _label) in &mut labels.iter_mut() {
            *vis = Visibility::Visible;
        }

        for (mut node, _vis, label) in &mut labels {
            let entity = labeled.get(label.entity);
            if let Ok(entity) = entity {
                let offset = Vec3::new(0.0, 1.0, 0.0);
                let world_position = entity.translation() + offset;

                let viewport_position =
                    camera.world_to_viewport(camera_global_transform, world_position);

                if let Ok(viewport_position) = viewport_position {
                    node.top = Val::Px(viewport_position.y);
                    node.left = Val::Px(viewport_position.x);
                } else {
                    warn!("Viewport position not found: {:?}", viewport_position);
                }
            } else {
                warn!("Entity not found: {:?}", label);
            }
        }
    }
}
