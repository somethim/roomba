mod draw;
mod sim;

use crate::sim::SimHost;
fn main() {
    SimHost::new().run();

    println!("Program complete!");
}
