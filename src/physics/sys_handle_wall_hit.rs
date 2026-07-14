use avian3d::prelude::*;
use bevy::prelude::*;

use crate::earth::spawn::Wall;

#[derive(EntityEvent)]
pub struct EventHitWall {
    pub entity: Entity,
    pub wall_id: Entity,
    pub body_velocity: LinearVelocity,
}

pub fn system(
    collision: On<CollisionStart>,
    mut commands: Commands,
    // mut already_run: ResMut<AlreadyRun<EventHitWall>>,
    // mut ev_hit_wall: EventWriter<EventHitWall>,
    bodies: Query<&LinearVelocity>,
    walls: Query<&Wall>,
) {
    // if already_run.is_triggered() {
    //     return;
    // };

    // let a = collisions.read().next().unwrap();
    let CollisionStart {
        collider1: a,
        collider2: b,
        ..
    } = *collision;
    debug!(?a, ?b);

    // let is_first_occurence = contacts.during_current_frame && !contacts.during_previous_frame;
    // if !is_first_occurence {
    //     continue;
    // }

    let (body_velocity, body_id, wall_id) =
        match (bodies.get(a), walls.get(b), walls.get(a), bodies.get(b)) {
            (Ok(a_body), Ok(_b_wall), ..) => (*a_body, a, b),
            (.., Ok(_a_wall), Ok(b_body)) => (*b_body, b, a),
            _ => return,
        };

    // already_run.trigger();

    commands.trigger(EventHitWall {
        entity: body_id,
        wall_id,
        body_velocity,
    });
}
