use macroquad::prelude::{draw_rectangle, draw_line, is_mouse_button_released, MouseButton, mouse_position, is_key_released, KeyCode, WHITE, RED, GREEN};
use std::cmp::min;

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
    boardBuf: [[bool; 25]; 25],
    size: Vector,
    paused: bool,
    next: bool,
    coolTime: i32
}

impl Board
{
    pub fn new(width: f32, height: f32) -> Self
    {
        let board: [[bool; 25]; 25] = [[false; 25]; 25];

        let blockWidth:f32 = width / 25f32;
        let blockHeight:f32 = height / 25f32;

        Self {board, boardBuf: board, size: Vector::new(blockWidth, blockHeight), paused: true, next: false, coolTime: 0}
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
                    let colour = if self.paused
                    {
                        RED
                    }
                    else
                    {
                        GREEN
                    };

                    draw_rectangle(self.size.width * col as f32, self.size.width * row as f32, self.size.width, self.size.height, colour);
                }
            }
        }
    }

    pub fn input(&mut self)
    {
        if is_mouse_button_released(MouseButton::Left)
        {
            let (mouseX, mouseY) = mouse_position();

            let x = (mouseX / self.size.width) as usize;
            let y = (mouseY / self.size.height) as usize;

            self.board[y][x] = !self.board[y][x];

            if !self.paused
            {
                self.paused = true;
            }
        }

        if is_key_released(KeyCode::P)
        {
            self.paused = !self.paused;
        }

        if is_key_released(KeyCode::N)
        {
            self.next = true
        }
    }

    pub fn update(&mut self)
    {
        if self.next
        {
            self.next = false;
        }
        else if self.paused
        {
            return;
        }
        else if self.coolTime > 0
        {
            self.coolTime -= 1;
            return;
        }

        for row in 0..=self.board.len() -1
        {
            for col in 0..=self.board[row].len() - 1
            {
                let mut aliveNum:i32 = 0;

                for rowAdd in row.saturating_sub(1)..=min(row + 1, self.board.len() - 1)
                {
                    for colAdd in col.saturating_sub(1)..=min(col + 1, self.board[rowAdd].len() - 1)
                    {
                        if row == rowAdd && col == colAdd
                        {
                            continue;
                        }
                        else if self.board[rowAdd][colAdd]
                        {
                            aliveNum += 1;
                        }
                    }
                }

                if self.board[row][col]
                {
                    self.boardBuf[row][col] = match aliveNum
                    {
                        0 | 1 => false,
                        4.. => false,
                        2 | 3 => true,
                        _ => unreachable!()
                    };
                }
                else
                {
                    self.boardBuf[row][col] = matches!(aliveNum, 3);
                }
            }
        }

        self.coolTime = 10;
        self.board = self.boardBuf;
    }
}
