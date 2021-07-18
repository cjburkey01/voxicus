mod cam;
mod input;
mod voxel;

use amethyst::assets::{DefaultLoader, Handle, Loader, LoaderBundle, ProcessingQueue};
use amethyst::core::ecs::World;
use amethyst::core::transform::TransformBundle;
use amethyst::core::Transform;
use amethyst::ecs::DispatcherBuilder;
use amethyst::input::{Axis, Bindings, InputBundle};
use amethyst::input::{Button, VirtualKeyCode};
use amethyst::renderer::light::{Light, PointLight};
use amethyst::renderer::palette::rgb::Rgb;
use amethyst::renderer::palette::LinSrgba;
use amethyst::renderer::plugins::RenderPbr3D;
use amethyst::renderer::rendy::core::hal::command::ClearColor;
use amethyst::renderer::rendy::mesh::{Normal, Position, Tangent, TexCoord};
use amethyst::renderer::rendy::texture::palette;
use amethyst::renderer::shape::Shape;
use amethyst::renderer::types::{DefaultBackend, MeshData, TextureData};
use amethyst::renderer::{
    Camera, Material, MaterialDefaults, Mesh, RenderToWindow, RenderingBundle,
};
use amethyst::utils::application_root_dir;
use amethyst::utils::auto_fov::{AutoFov, AutoFovSystem};
use amethyst::window::DisplayConfig;
use amethyst::{Application, GameData, LoggerConfig, SimpleState, SimpleTrans, StateData, Trans};
use cam::FreeFlyCamera;

/// The main state for the game to run in.
struct MainState;
impl SimpleState for MainState {
    fn on_start(&mut self, mut state_data: StateData<'_, GameData>) {
        initialize_camera(state_data.world);
        initialize_sphere(&mut state_data);
        initialize_light(state_data.world);
    }

    fn fixed_update(&mut self, _data: StateData<'_, GameData>) -> SimpleTrans {
        Trans::None
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

    // Other required components
    let camera = Camera::perspective(1.0, std::f32::consts::FRAC_PI_2, 0.05);
    let free_fly_cam = FreeFlyCamera { speed: 5.0 };

    // Create the entity from the given components
    let entity = (transform, auto_fov, camera, free_fly_cam);

    // Add the entity to the world
    world.push(entity);
}

/// Example sphere.
fn initialize_sphere(data: &mut StateData<'_, GameData>) {
    // Get resources necessary to generate the mesh
    // Hopefully this gets a little less ugly with time
    let loader = data.resources.get::<DefaultLoader>().unwrap();
    let mesh_storage = data.resources.get::<ProcessingQueue<MeshData>>().unwrap();
    let tex_storage = data
        .resources
        .get::<ProcessingQueue<TextureData>>()
        .unwrap();
    let mtl_storage = data.resources.get::<ProcessingQueue<Material>>().unwrap();

    // Create mesh
    let mesh: Handle<Mesh> = loader.load_from_data(
        Shape::Sphere(64, 64)
            .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
            .into(),
        (),
        &mesh_storage,
    );

    // Create flat texture
    let albedo = loader.load_from_data(
        palette::load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 0.5)).into(),
        (),
        &tex_storage,
    );

    // Create material
    let mtl: Handle<Material> = {
        let mat_defaults = data.resources.get::<MaterialDefaults>().unwrap().0.clone();

        loader.load_from_data(
            Material {
                albedo,
                ..mat_defaults
            },
            (),
            &mtl_storage,
        )
    };

    // Add the entity to the world
    data.world.push((Transform::default(), mesh, mtl));
}

/// Create an example light.
fn initialize_light(world: &mut World) {
    // Create the transform component
    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);

    // Create the light component
    let light: Light = PointLight {
        intensity: 10.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }
    .into();

    // Create an example light
    world.push((transform, light));
}

fn init_input_bindings() -> amethyst::Result<Bindings> {
    let mut bindings = Bindings::new();

    // Insert main control axes
    bindings.insert_axis(
        input::HORIZONTAL,
        Axis::Emulated {
            pos: Button::Key(VirtualKeyCode::D),
            neg: Button::Key(VirtualKeyCode::A),
        },
    )?;
    bindings.insert_axis(
        input::VERTICAL,
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
    amethyst::start_logger(LoggerConfig::default());
    let app_root = application_root_dir()?;

    let mut dispatcher_builder = DispatcherBuilder::default();
    dispatcher_builder
        // Bundles
        .add_bundle(LoaderBundle)
        .add_bundle(TransformBundle)
        .add_bundle(InputBundle::new().with_bindings(init_input_bindings()?))
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(DisplayConfig {
                        title: concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"))
                            .to_owned(),
                        //maximized: true,
                        ..Default::default()
                    })
                    .with_clear(ClearColor {
                        float32: [0.2, 0.4, 0.6, 1.0],
                    }),
                )
                .with_plugin(RenderPbr3D::default()),
        )
        // Systems
        .add_system(AutoFovSystem)
        .add_system(cam::free_fly_camera_update_system);

    // Start the game
    Application::build(app_root, MainState)?
        .build(dispatcher_builder)?
        .run();

    // Return after shutdown
    println!("exiting");
    Ok(())
}
