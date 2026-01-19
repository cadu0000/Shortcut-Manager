use crate::enums::input_device::InputDevice;
use crate::ui::render_cli::run;

pub mod models;
pub mod ui;
pub mod enums;
pub mod services;
pub mod utils;

fn main() {
    run();
}
