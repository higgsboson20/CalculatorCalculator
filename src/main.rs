/*
Idea: given eq E: f = f(x0, x1, ... , xn), solve for variable xi, given the rest of the variables are known
*/

mod gui;
mod process;
mod math;
use tcrate::ErrorState;

use gui::start_app;

fn main() {

    match start_app() {
        Ok(_) => {},
        Err(err) => print!("Error: {}", err)
    }
}
