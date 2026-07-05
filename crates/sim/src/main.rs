use crate::map::{declare, draw};

mod map;

#[macroquad::main("Roomba Sim")]
async fn main() {
    let scene = declare();

    draw(scene).await;
}
