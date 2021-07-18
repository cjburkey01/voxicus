use crate::input::{AxisBinding, MovementBindingTypes};
use amethyst::core::ecs::{
    Component, Join, NullStorage, Read, ReadStorage, System, SystemData, VecStorage, WriteStorage,
};
use amethyst::core::math::{Vector1, Vector3};
use amethyst::core::{SystemDesc, Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::input::InputHandler;

/// The component to mark a free flying camera.
#[derive(Debug, Default)]
pub struct FreeFlyCamera {
    pub speed: f32,
}

impl Component for FreeFlyCamera {
    type Storage = VecStorage<Self>;
}

/// The system responsible for moving the free fly camera.
#[derive(Default, SystemDesc)]
pub struct FreeFlyCameraSystem {}

impl<'a> System<'a> for FreeFlyCameraSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputHandler<MovementBindingTypes>>,
        ReadStorage<'a, FreeFlyCamera>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (time, input, ffc, mut transform): Self::SystemData) {
        for (ffc, transform) in (&ffc, &mut transform).join() {
            // Make velocity relative to delta time
            let delta_time = time.delta_seconds();
            let relative_translation = Vector3::new(
                input.axis_value(&AxisBinding::Horizontal).unwrap(),
                0.0,
                -input.axis_value(&AxisBinding::Vertical).unwrap(),
            ) * ffc.speed;

            // Move forward relative to the camera's current bearing.
            transform.append_translation(relative_translation * delta_time);
        }
    }
}
