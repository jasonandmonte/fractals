//! Z2 Heighway Dragon
//! IFS Paradigm
//! 
//! References:
//! https://larryriddle.agnesscott.org/ifs/heighway/Z2heighway.htm

use nannou::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn scale(&mut self, dx: f32, dy: f32) {
        self.x *= dx;
        self.y *= dy;
    }

    fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    fn rotate(&mut self, degrees: f32) {
        let rads = degrees * ((PI/180.0) as f32);
        let (sin_r, cos_r) = (rads.sin(), rads.cos());

        let nx = self.x * cos_r - self.y * sin_r;
        let ny = self.x * sin_r + self.y * cos_r;
        self.x = nx;
        self.y = ny;
    }
}

struct Model {
    _window: window::Id,
    angle: f32,
}

fn main() {
    nannou::app(model).size(1000, 1000).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window, angle: 0.0 }
}
/*
// Can be used for 
fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {simple, ..} = event {
        match simple {
            Some(KeyPressed(Key::Right)) => {
                model.angle += 0.01;
                println!("angle++");
                // FIXME: need a way to draw after this event ONLY
            }
            Some(KeyPressed(Key::Left)) => {
                model.angle -= 0.01;
            }
            _ => {}
        }
    }
}
*/

fn view(app: &App, model: &Model, frame: Frame) {
    // NOTE: Can use to wait of user input
    // app.set_loop_mode(LoopMode::wait());
    app.set_loop_mode(LoopMode::loop_ntimes(1));
    let window = app.main_window();
    window.set_outer_position_pixels(0, 0);
    let win = window.rect();

    let draw = app.draw().x_y(-win.w()*0.5, -win.h()*0.5);
    draw.background().color(BLACK);

    let mut rng = rand::rng();
    let mut p = Point { x: 0.0, y: 0.0 };

    let mut counts: HashMap<String, u32> = HashMap::new();

    const ZOOM: f32 = 5.0;


    // rect overwhelming more memory efficient: 1B+ rect vs 1M ellipse
    for _ in 0..(5_000_000_000 as u64) {
        let r = rng.random_range(0.0..1.0);

        z2_dragon(r, &mut p);
        let x = (p.x * win.w()) / ZOOM + win.w() * 0.5;
        let y = (p.y * win.h()) / ZOOM + win.h() * 0.5;

        let key = format!("{:.0},{:.0}", x, y);


        *counts.entry(key).or_insert(0) += 1;
    }

    let max_count = *counts.values().max().unwrap_or(&1);
    println!("max_count: {}", max_count);


    // Apply heatmap
    for (key, count) in &counts {
        // Parse "x,y" string back to float values
        let coords: Vec<&str> = key.split(',').collect();
        let x: f32 = coords[0].parse().unwrap_or(0.0);
        let y: f32 = coords[1].parse().unwrap_or(0.0);

        let t = (*count as f32 / max_count as f32).powf(0.2);

        let r = t;
        let g = t;
        let b = 1.0;

        // Lower alpha and brightness for outer spirals
        let brightness = 1.0;
        let alpha = t.powf(0.25);

        let color = srgba(r * brightness, g * brightness, b * brightness, alpha);

        let sin_a = model.angle.sin();
        let cos_a = model.angle.cos();
        let rx = x * cos_a - y * sin_a;
        let ry = x * sin_a + y * cos_a;

        draw.rect()
            .x_y(rx, ry)
            .w_h(1.0, 1.0) // 1-pixel square
            .color(color);

    }
    
    draw.to_frame(app, &frame).unwrap();
}

fn z2_dragon(r:f32, p: &mut Point) {
    if r < 0.25 {
        p.scale(1.0/(2.0).sqrt(), 1.0/(2.0).sqrt());
        p.rotate(45.0);
        
    } else if r < 0.5 {
        p.scale(1.0/(2.0).sqrt(), 1.0/(2.0).sqrt());
        p.rotate(-135.0);
        
    } else if r < 0.75 {
        p.scale(1.0/(2.0).sqrt(), 1.0/(2.0).sqrt());
        p.rotate(135.0);
        p.translate(1.0, 0.0);
    } else {
        p.scale(1.0/(2.0).sqrt(), 1.0/(2.0).sqrt());
        p.rotate(-45.0);
        p.translate(-1.0, 0.0);
    }
}