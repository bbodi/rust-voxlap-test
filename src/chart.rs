extern crate voxlap;

use std::collections::RingBuf;
use std::collections::Deque;
use std::cmp::min;
use std::cmp::max;

use voxlap::RenderContext;

pub struct Chart {
    data: RingBuf<u32>,
    x: u32,
    y: u32,
    max_elem_count: u32,
    max_height: u32,
    column_width: u32,
    right_x: u32,
    bottom_y: u32
}

impl Chart {
    pub fn new() -> Chart {
        Chart {
            data: RingBuf::new(),
            x: 0,
            y: 0,
            max_elem_count: 100,
            max_height: 100,
            column_width: 2,
            right_x: 200,
            bottom_y: 100,
        }
    }

    pub fn x(mut self, x: u32) -> Chart {
        self.x = x;
        self.right_x = (x + self.max_elem_count * self.column_width) as u32;
        self
    }

    pub fn y(mut self, y: u32) -> Chart {
        self.y = y;
        self.bottom_y = min( (self.y + self.max_height) as u32, 599);
        self
    }

    pub fn max_elem_count(mut self, max_elem_count: u32) -> Chart {
        self.max_elem_count = max_elem_count;
        self.right_x = (self.x + self.max_elem_count * self.column_width) as u32;
        self
    }

    pub fn max_height(mut self, max_height: u32) -> Chart {
        self.max_height = max_height;
        self.bottom_y = min( (self.y + self.max_height) as u32, 599);
        self
    }

    pub fn column_width(mut self, column_width: u32) -> Chart {
        self.column_width = column_width;
        self
    }

    fn draw_bars(&self, dst: &RenderContext) {
        for iter in self.data.iter().enumerate() {
            let (index, value) = iter;
            for i in range(0, self.column_width) {
                let x1 = max(0, self.right_x as i32 - (index as i32*self.column_width as i32 + i as i32));
                let y1 = self.bottom_y - min(*value, self.max_height);
                let x2 = x1;
                let y2 = self.bottom_y;
                dst.draw_line_2d(x1 as u32, y1, x2 as u32, y2, voxlap::Color::rgb(0, 0, 255));
            }
        }
    }

    fn draw_center_line(&self, dst: &RenderContext) {
        let x1 = self.x;
        let y1 = self.y + self.max_height/2;
        let x2 = self.right_x;
        let y2 = self.y + self.max_height/2;
        dst.draw_line_2d(x1, y1, x2, y2, voxlap::Color::rgb(0xFF, 0x66, 0));
        dst.print6x8(self.right_x-16, y1, voxlap::Color::rgb(0xFF, 0x66, 0), None, "50");
    }

    fn draw_top_line(&self, dst: &RenderContext) {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = self.right_x;
        let y2 = self.y;
        dst.draw_line_2d(x1, y1, x2, y2, voxlap::Color::rgb(0, 255, 0));
        dst.print6x8(self.right_x-24, y1, voxlap::Color::rgb(00, 255, 0), None, "100");
    }

    pub fn add_data(&mut self, data: u32) {
        self.data.push_front(data);
            if self.data.len() >= 100 {
             self.data.pop();
         }
    }

    pub fn draw(&self, dst: &RenderContext) {
        self.draw_bars(dst);
        self.draw_center_line(dst);
        self.draw_top_line(dst);

        let current_fps = *self.data.front().unwrap_or(&0);
        let y_pos = self.bottom_y - min(current_fps, self.max_height);
        if y_pos + 7 < 600 {
            dst.print6x8(self.right_x, self.bottom_y - current_fps, voxlap::Color::rgb(255, 255, 255), None, format!("{}", current_fps).as_slice());
        }
    }
}
