use macroquad::prelude::{draw_rectangle, draw_line, WHITE, RED};

struct Vector
{
    width: f32,
    height: f32
}

impl Vector
{
    pub fn new(width: f32, height: f32) -> Self
    {
        Self{width, height}
    }
}

impl Default for Vector
{
    fn default() -> Self
    {
        Self{width: 0f32, height: 0f32}
    }
}

pub struct Board
{
    board: [[bool; 25]; 25],
    size: Vector
}

impl Board
{
    pub fn new(width: f32, height: f32) -> Self
    {
        let board: [[bool; 25]; 25] = [[false; 25]; 25];

        let blockWidth:f32 = width / 25f32;
        let blockHeight:f32 = height / 25f32;

        Self {board, size: Vector::new(blockWidth, blockHeight)}
    }

    pub fn draw(&self)
    {
        let totalWidth = self.size.width * 25f32;
        let totalHeight = self.size.height * 25f32;

        for i in 1..=24
        {
            draw_line(self.size.width * i as f32, 0f32, self.size.width * i as f32, totalHeight, 2f32, WHITE);
            draw_line(0f32, self.size.height * i as f32, totalWidth, self.size.height * i as f32, 2f32, WHITE);
        }

        for row in 0..=24
        {
            for col in 0..=24
            {
                if self.board[row][col]
                {
                    draw_rectangle(self.size.width * col as f32, self.size.width * row as f32, self.size.width, self.size.height, RED);
                }
            }
        }
    }
}
