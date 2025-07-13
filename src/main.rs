use raylib::prelude::*;
mod framebuffer;
use framebuffer::FrameBuffer;
use image::{RgbImage, Rgb};

fn main() {
    let (width, height) = (800, 600);
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Polígono 2 (solo se muestra este)")
        .build();

    let mut framebuffer = FrameBuffer::new(width, height);

    // Poligono 1
    let poly1 = vec![
        Vector2 { x: 165.0, y: 380.0 }, Vector2 { x: 185.0, y: 360.0 },
        Vector2 { x: 180.0, y: 330.0 }, Vector2 { x: 207.0, y: 345.0 },
        Vector2 { x: 233.0, y: 330.0 }, Vector2 { x: 230.0, y: 360.0 },
        Vector2 { x: 250.0, y: 380.0 }, Vector2 { x: 220.0, y: 385.0 },
        Vector2 { x: 205.0, y: 410.0 }, Vector2 { x: 193.0, y: 383.0 },
    ];

    // Poligono 2
    let poly2 = vec![
        Vector2 { x: 321.0, y: 335.0 }, Vector2 { x: 288.0, y: 286.0 },
        Vector2 { x: 339.0, y: 251.0 }, Vector2 { x: 374.0, y: 302.0 },
    ];

    // Solo dibuja polígono 2:
    draw_polygon_edges(&mut framebuffer, &poly2, Color::WHITE);
    fill_polygon(&mut framebuffer, &poly2, Color::BLUE);

    save_framebuffer_as_png(&framebuffer, "out.png");

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for y in 0..height {
            for x in 0..width {
                if let Some(color) = framebuffer.get_pixel(x, y) {
                    d.draw_pixel(x as i32, y as i32, color);
                }
            }
        }
    }
}

fn save_framebuffer_as_png(framebuffer: &FrameBuffer, filename: &str) {
    let width = framebuffer.width() as u32;
    let height = framebuffer.height() as u32;

    let mut imgbuf = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            if let Some(color) = framebuffer.get_pixel(x as i32, y as i32) {
                imgbuf.put_pixel(x, y, Rgb([color.r, color.g, color.b]));
            } else {
                imgbuf.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }
    }

    imgbuf.save(filename).expect("Error al guardar PNG");
}

fn line(framebuffer: &mut FrameBuffer, start: Vector2, end: Vector2, color: Color) {
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        framebuffer.set_pixel(x0, y0, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn draw_polygon_edges(framebuffer: &mut FrameBuffer, poly: &[Vector2], color: Color) {
    for i in 0..poly.len() {
        let start = poly[i];
        let end = poly[(i + 1) % poly.len()];
        line(framebuffer, start, end, color);
    }
}

fn point_in_polygon(point: Vector2, polygon: &[Vector2]) -> bool {
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let pi = polygon[i];
        let pj = polygon[j];
        if ((pi.y > point.y) != (pj.y > point.y)) &&
            (point.x < (pj.x - pi.x) * (point.y - pi.y) / ((pj.y - pi.y) + 0.001) + pi.x) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn fill_polygon(framebuffer: &mut FrameBuffer, poly: &[Vector2], color: Color) {
    let min_y = poly.iter().map(|v| v.y as i32).min().unwrap_or(0);
    let max_y = poly.iter().map(|v| v.y as i32).max().unwrap_or(0);
    let width = framebuffer.width();

    for y in min_y..=max_y {
        for x in 0..width {
            let point = Vector2 { x: x as f32, y: y as f32 };
            if point_in_polygon(point, poly) {
                framebuffer.set_pixel(x, y, color);
            }
        }
    }
}
