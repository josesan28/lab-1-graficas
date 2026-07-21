mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::fill_polygon;

fn main() {
    let width = 800;
    let height = 600;
    let background = Color::new(50, 50, 100, 255);
    let mut framebuffer = Framebuffer::new(width, height, background);
    framebuffer.clear();

    // Polígono 1
    let poly1 = vec![
        Vector2::new(165.0, 380.0),
        Vector2::new(185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];

    // Rellenar polígonos con sus colores
    fill_polygon(&mut framebuffer, &poly1, Color::YELLOW, Color::WHITE);

    // Guardar imagen en formato PNG
    let output_file = "out.png";
    framebuffer.render_to_file(output_file);
    println!("Imagen guardada en {}", output_file);
}