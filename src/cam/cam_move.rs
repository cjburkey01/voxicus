use amethyst::core::math::Vector3;
use amethyst::core::{Time, Transform};
use amethyst::input::InputHandler;
use legion::system;

/// The component to mark a free flying camera.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreeFlyCamera {
    pub speed: f32,
}

/// The system responsible for moving the free fly camera.
#[system(for_each)]
pub fn free_fly_camera_update(
    #[resource] time: &Time,
    #[resource] input: &InputHandler,
    ffc: &FreeFlyCamera,
    transform: &mut Transform,
) {
    // Make velocity relative to delta time
    let delta_time = time.delta_time().as_secs_f32();
    let relative_translation = Vector3::new(
        input.axis_value(crate::input::HORIZONTAL).unwrap(),
        0.0,
        -input.axis_value(crate::input::VERTICAL).unwrap(),
    ) * ffc.speed;

    // Move forward relative to the camera's current bearing.
    transform.append_translation(relative_translation * delta_time);
}
