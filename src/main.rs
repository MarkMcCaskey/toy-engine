extern crate piston;
extern crate piston_window;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate dyon;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate clap;
extern crate find_folder;
extern crate dev_menu;
extern crate futures;
extern crate tokio_core;

pub mod scripting;
pub mod asset_manager;
pub mod mesh;
pub mod texture;
pub mod io;
pub mod globalstate;
pub mod job_manager;

use piston_window::*;

fn main() {
    let mut globalstate = globalstate::GlobalState::new();

    loop {
        globalstate.run();
    }

}
