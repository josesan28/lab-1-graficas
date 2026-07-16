mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::line;
use polygon::draw_polygon;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, Color::new(50, 50, 100, 255));
    framebuffer.clear();

    // Líneas
    framebuffer.set_current_color(Color::GREEN);
    line(
        &mut framebuffer,
        Vector2::new(50.0, 50.0),
        Vector2::new(350.0, 350.0),
    );

    framebuffer.set_current_color(Color::RED);
    line(
        &mut framebuffer,
        Vector2::new(350.0, 50.0),
        Vector2::new(50.0, 350.0),
    );

    // Triángulo
    framebuffer.set_current_color(Color::YELLOW);
    let triangulo = vec![
        Vector2::new(400.0, 100.0),
        Vector2::new(550.0, 400.0),
        Vector2::new(250.0, 400.0),
    ];
    draw_polygon(&mut framebuffer, &triangulo);

    // Cuadrado
    framebuffer.set_current_color(Color::CYAN);
    let cuadrado = vec![
        Vector2::new(100.0, 450.0),
        Vector2::new(300.0, 450.0),
        Vector2::new(300.0, 550.0),
        Vector2::new(100.0, 550.0),
    ];
    draw_polygon(&mut framebuffer, &cuadrado);

    // Polígono que se sale de la pantalla
    framebuffer.set_current_color(Color::MAGENTA);
    let fuera = vec![
        Vector2::new(-50.0, -50.0),
        Vector2::new(850.0, -50.0),
        Vector2::new(850.0, 650.0),
        Vector2::new(-50.0, 650.0),
    ];
    draw_polygon(&mut framebuffer, &fuera);

    let output_file = "polygons.png";
    framebuffer.render_to_file(output_file);
    println!("Imagen guardada en {}", output_file);
}