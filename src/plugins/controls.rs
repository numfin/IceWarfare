use std::marker::PhantomData;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ControlPlugin<A: Actionlike>(PhantomData<A>);
impl<A: Actionlike> Plugin for ControlPlugin<A> {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<A>::default());
    }
}
impl<A: Actionlike> Default for ControlPlugin<A> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
