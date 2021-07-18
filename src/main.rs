mod cam;
mod input;
mod voxel;

use crate::cam::{FreeFlyCamera, FreeFlyCameraSystem};
use amethyst::assets::AssetLoaderSystemData;
use amethyst::core::ecs::{Builder, World, WorldExt};
use amethyst::core::math::Vector3;
use amethyst::core::{Transform, TransformBundle};
use amethyst::input::{Axis, Bindings, InputBundle, StringBindings};
use amethyst::input::{Button, VirtualKeyCode};
use amethyst::renderer::light::{Light, PointLight};
use amethyst::renderer::palette::rgb::Rgb;
use amethyst::renderer::plugins::RenderPbr3D;
use amethyst::renderer::rendy::mesh::{Normal, Position, Tangent, TexCoord};
use amethyst::renderer::shape::Shape;
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::{
    Camera, Material, MaterialDefaults, Mesh, RenderToWindow, RenderingBundle,
};
use amethyst::utils::application_root_dir;
use amethyst::utils::auto_fov::{AutoFov, AutoFovSystem};
use amethyst::window::DisplayConfig;
use amethyst::{Application, GameData, GameDataBuilder, SimpleState, StateData};

/// The main state for the game to run in.
struct MainState;
impl SimpleState for MainState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        initialize_camera(state_data.world);
        initialize_sphere(state_data.world);
        initialize_light(state_data.world);
    }
}

/// Create the camera for the main state.
fn initialize_camera(world: &mut World) {
    // Default camera position
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    // Camera properties should be set on this to override the `Camera`
    // component when the window is resized.
    let mut auto_fov = AutoFov::new();
    auto_fov.set_fov(std::f32::consts::FRAC_PI_2); // 90º
    auto_fov.set_near(0.05);

    world
        .create_entity()
        .with(transform)
        .with(auto_fov)
        .with(Camera::perspective(1.0, std::f32::consts::FRAC_PI_2, 0.05))
        .with(FreeFlyCamera { speed: 5.0 })
        .build();
}

/// Example sphere.
fn initialize_sphere(world: &mut World) {
    // Create the mesh
    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Sphere(100, 100)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    // Create an example sphere
    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(material_defaults.clone(), ())
    });

    // Set it to the origin
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.0);

    world
        .create_entity()
        .with(transform)
        .with(mesh)
        .with(material)
        .build();
}

/// Create an example light.
fn initialize_light(world: &mut World) {
    // The light
    let light: Light = PointLight {
        intensity: 10.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }
    .into();

    // Create a transform
    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);

    // Create an example light
    world.create_entity().with(light).with(transform).build();
}

fn init_input_bindings() -> amethyst::Result<Bindings<input::MovementBindingTypes>> {
    let mut bindings = Bindings::new();

    // Insert main control axes
    bindings.insert_axis(
        input::AxisBinding::Horizontal,
        Axis::Emulated {
            pos: Button::Key(VirtualKeyCode::D),
            neg: Button::Key(VirtualKeyCode::A),
        },
    )?;
    bindings.insert_axis(
        input::AxisBinding::Vertical,
        Axis::Emulated {
            pos: Button::Key(VirtualKeyCode::W),
            neg: Button::Key(VirtualKeyCode::S),
        },
    )?;

    Ok(bindings)
}

fn main() -> amethyst::Result<()> {
    // Start logging
    println!("starting game engine");
    amethyst::start_logger(Default::default());
    let _app_root = application_root_dir()?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::new().with_bindings(init_input_bindings()?))?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(DisplayConfig {
                        title: concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"))
                            .to_owned(),
                        maximized: true,
                        ..Default::default()
                    })
                    .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()),
        )?
        .with(AutoFovSystem::new(), "auto_fov", &[])
        .with(
            FreeFlyCameraSystem::default(),
            "free_fly_camera",
            &["transform_system", "input_system"],
        );

    // Start the game
    Application::build("/", MainState)?.build(game_data)?.run();

    // Return after shutdown
    println!("exiting");
    Ok(())
}
