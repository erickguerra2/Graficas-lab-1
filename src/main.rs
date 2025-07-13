use raylib::prelude::*;
mod framebuffer;
use framebuffer::FrameBuffer;
use image::{RgbImage, Rgb};

fn main() {
    let (width, height) = (800, 600);
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .build();

    let mut framebuffer = FrameBuffer::new(width, height);

    // Polígono 1
    let poly1 = vec![
        Vector2 { x: 165.0, y: 380.0 }, Vector2 { x: 185.0, y: 360.0 },
        Vector2 { x: 180.0, y: 330.0 }, Vector2 { x: 207.0, y: 345.0 },
        Vector2 { x: 233.0, y: 330.0 }, Vector2 { x: 230.0, y: 360.0 },
        Vector2 { x: 250.0, y: 380.0 }, Vector2 { x: 220.0, y: 385.0 },
        Vector2 { x: 205.0, y: 410.0 }, Vector2 { x: 193.0, y: 383.0 },
    ];

    // Polígono 2
    let poly2 = vec![
        Vector2 { x: 321.0, y: 335.0 }, Vector2 { x: 288.0, y: 286.0 },
        Vector2 { x: 339.0, y: 251.0 }, Vector2 { x: 374.0, y: 302.0 },
    ];

    // Polígono 3
    let poly3 = vec![
        Vector2 { x: 377.0, y: 249.0 }, Vector2 { x: 411.0, y: 197.0 },
        Vector2 { x: 436.0, y: 249.0 },
    ];

    // Polígono 4
    let poly4 = vec![
        Vector2 { x: 413.0, y: 177.0 }, Vector2 { x: 448.0, y: 159.0 }, Vector2 { x: 502.0, y: 88.0 }, Vector2 { x: 553.0, y: 53.0 },
        Vector2 { x: 535.0, y: 36.0 }, Vector2 { x: 676.0, y: 37.0 }, Vector2 { x: 660.0, y: 52.0 }, Vector2 { x: 750.0, y: 145.0 },
        Vector2 { x: 761.0, y: 179.0 }, Vector2 { x: 672.0, y: 192.0 }, Vector2 { x: 659.0, y: 214.0 }, Vector2 { x: 615.0, y: 214.0 },
        Vector2 { x: 632.0, y: 230.0 }, Vector2 { x: 580.0, y: 230.0 }, Vector2 { x: 597.0, y: 215.0 }, Vector2 { x: 552.0, y: 214.0 },
        Vector2 { x: 517.0, y: 144.0 }, Vector2 { x: 466.0, y: 180.0 },
    ];

    // Polígono 5
    let poly5 = vec![
        Vector2 { x: 682.0, y: 175.0 }, Vector2 { x: 708.0, y: 120.0 },
        Vector2 { x: 735.0, y: 148.0 }, Vector2 { x: 739.0, y: 170.0 },
    ];

    draw_polygon_edges(&mut framebuffer, &poly4, Color::WHITE);
    draw_polygon_edges(&mut framebuffer, &poly5, Color::WHITE);

    fill_polygon_with_hole(&mut framebuffer, &poly4, &poly5, Color::GREEN, Color::BLACK);

    save_framebuffer_as_png(&framebuffer, "out.png");

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for y in 0..height {
            for x in 0..width {
                if let Some(color) = framebuffer.get_pixel(x as i32, y as i32) {
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

fn fill_polygon_with_hole(
    framebuffer: &mut FrameBuffer,
    outer: &[Vector2],
    hole: &[Vector2],
    fill_color: Color,
    hole_color: Color,
) {
    let min_y = outer.iter().map(|v| v.y as i32).min().unwrap_or(0);
    let max_y = outer.iter().map(|v| v.y as i32).max().unwrap_or(0);
    let width = framebuffer.width();

    for y in min_y..=max_y {
        for x in 0..width {
            let point = Vector2 { x: x as f32, y: y as f32 };
            if point_in_polygon(point, outer) && !point_in_polygon(point, hole) {
                framebuffer.set_pixel(x, y, fill_color);
            } else if point_in_polygon(point, hole) {
                framebuffer.set_pixel(x, y, hole_color);
            }
        }
    }
}
