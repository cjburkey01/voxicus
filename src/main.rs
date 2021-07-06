use amethyst::input::{InputBundle, StringBindings};
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::{RenderToWindow, RenderingBundle};
use amethyst::utils::application_root_dir;
use amethyst::window::DisplayConfig;
use amethyst::{Application, GameDataBuilder, SimpleState};
use amethyst_imgui::RenderImgui;

// The system that will execute with the IMGUI handle
#[derive(Default, Clone, Copy)]
struct ImguiSystem;
impl<'a> amethyst::ecs::System<'a> for ImguiSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        amethyst_imgui::with(|ui| {
            ui.show_demo_window(&mut true);
        });
    }
}

struct MainState;
impl SimpleState for MainState {}

fn main() -> amethyst::Result<()> {
    // Start logging
    println!("starting game engine");
    amethyst::start_logger(Default::default());
    let _app_root = application_root_dir()?;

    let game_data = GameDataBuilder::default()
        // Include the IMGUI system
        .with(ImguiSystem::default(), "imgui_use", &[])
        .with_bundle(InputBundle::<StringBindings>::default())?
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
                .with_plugin(RenderImgui::<StringBindings>::default()),
        )?;

    // Start the game
    Application::build("/", MainState)?.build(game_data)?.run();

    // Return after shutdown
    println!("exiting");
    Ok(())
}
