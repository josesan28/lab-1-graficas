use crate::framebuffer::Framebuffer;
use crate::line::line;
use raylib::prelude::Vector2;

pub fn draw_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    let len = points.len();
    if len < 3 {
        return;
    }

    for i in 0..len {
        let j = (i + 1) % len;
        let start = points[i];
        let end = points[j];
        line(framebuffer, start, end);
    }
}