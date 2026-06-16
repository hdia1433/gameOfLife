#![allow(non_snake_case)]

mod board;

use macroquad::prelude::{BLACK, Conf, clear_background, next_frame};
use board::Board;

fn windowConf() -> Conf
{
    Conf{
        window_title: "Game of life".to_string(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(windowConf)]
async fn main() 
{
    let mut board = Board::new(800f32, 800f32);

    loop
    {
        clear_background(BLACK);
        
        board.input();
        board.update();
        board.draw();

        next_frame().await;
    }
}
