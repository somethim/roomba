mod draw;
mod sim;

use crate::sim::SimHost;

#[macroquad::main("sim")]
async fn main() {
    SimHost::new().run().await;

    println!("Program complete!");
}
