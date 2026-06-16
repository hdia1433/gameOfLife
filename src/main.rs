#![allow(non_snake_case)]

mod board;

use macroquad::prelude::{BLACK, clear_background, next_frame, screen_width, screen_height, request_new_screen_size};
use board::Board;

#[macroquad::main("Game of Life")]
async fn main() 
{
    request_new_screen_size(800f32, 800f32);

    let board = Board::new(800f32, 800f32);

    loop
    {
        clear_background(BLACK);

        board.draw();

        next_frame().await;
    }
}
