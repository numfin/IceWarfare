use bevy::prelude::*;
use bevy_xpbd_3d::components::LinearVelocity;
use bevy_xpbd_3d::prelude::{Collisions, SpatialQuery, SpatialQueryFilter};

#[derive(Event)]
pub struct TwoBodyImpact {
    pub a: (LinearVelocity, Entity),
    pub b: (LinearVelocity, Entity),
}

/// Sends collision events and updates [`CollidingEntities`].
pub fn report_contacts(
    // spatial_query: SpatialQuery,
    // moving_guys: Query<(&LinearVelocity, &Transform)>,
    collisions: Res<Collisions>,
    velocities: Query<&LinearVelocity>,
    mut ev_writer: EventWriter<TwoBodyImpact>,
) {
    // for (vel, t) in &moving_guys {
    //     let Ok(dir) = Direction3d::new(vel.0) else {
    //         continue;
    //     };
    //     if let Some(first_hit) = spatial_query.cast_ray(
    //         t.translation,
    //         dir,
    //         100.0,
    //         true,
    //         SpatialQueryFilter::default(),
    //     ) {
    //         dbg!(first_hit);
    //     }
    // }
    for ((entity1, entity2), contacts) in collisions.get_internal().iter() {
        // if contacts.during_current_frame {
        //     if !contacts.during_previous_frame {
        if let (Ok(vel_a), Ok(vel_b)) = (velocities.get(*entity1), velocities.get(*entity2)) {
            if vel_b.length() == 0.0 || vel_a.length() == 0.0 {
                // dbg!(vel_a.length());
                // dbg!(vel_b.length());
                // dbg!(vel_a.distance(vel_b.0));
            }

            ev_writer.send(TwoBodyImpact {
                a: (*vel_a, *entity1),
                b: (*vel_b, *entity2),
            });
            //     }
            // }
        }
    }
}
