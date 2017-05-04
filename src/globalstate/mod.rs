use log4rs::Handle;
use find_folder;
use std;
use piston_window::*;

use io::logging::*;
use io::arguments::*;
use asset_manager::asset_manager::AssetManager;


pub struct GlobalState {
    logger: Handle,
    debug_mode: bool,
    assets_folder: std::path::PathBuf,
    asset_manager: AssetManager,
    default_font: Glyphs,
    window: PistonWindow,
}

impl GlobalState {
    pub fn new() -> GlobalState {
        let arguments = read_arguments();
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let mut asset_manager = AssetManager::new();
        let ref font = assets.join("FiraSans-Regular.ttf");

        let mut window: PistonWindow =
            WindowSettings::new("temp", [200, 200]).exit_on_esc(true).build().unwrap();


        window.set_lazy(true);

        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory).unwrap();


        GlobalState {
            logger: setup_logging(arguments.is_present("trace")),
            debug_mode: arguments.is_present("debug"),
            assets_folder: assets,
            asset_manager: asset_manager,
            default_font: glyphs,
            window: window,
        }
    }

    pub fn run(&mut self) -> () {
        while let Some(e) = self.window.next() {
            let ref mut font = self.default_font;
            self.window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(10.0, 100.0);

                clear([0.0, 0.0, 0.0, 1.0], g);
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw("Hello world!",
                                                                     font,
                                                                     &c.draw_state,
                                                                     transform,
                                                                     g);
            });
        }

    }
}
