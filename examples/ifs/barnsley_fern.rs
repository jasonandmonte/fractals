//! Barnsley Fern
//! IFS Paradigm
//! 
//! References:
//! https://en.wikipedia.org/wiki/Barnsley_fern
//! https://mathworld.wolfram.com/BarnsleysFern.html

use nannou::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn _scale(&mut self, dx: f32, dy: f32) {
        self.x *= dx;
        self.y *= dy;
    }

    fn _translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    fn _rotate(&mut self, degrees: f32) {
        let rads = degrees * ((PI/180.0) as f32);
        let (sin_r, cos_r) = (rads.sin(), rads.cos());

        let nx = self.x * cos_r - self.y * sin_r;
        let ny = self.x * sin_r + self.y * cos_r;
        self.x = nx;
        self.y = ny;
    }

    fn affine_transformations(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        let nx = a * self.x + b * self.y + e;
        let ny = c * self.x + d * self.y + f;
        self.x = nx;
        self.y = ny;
    }
}

struct Model {
    _window: window::Id,
}

fn main() {
    nannou::app(model).size(1000, 1000).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn _event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    app.set_loop_mode(LoopMode::loop_ntimes(1));
    let window = app.main_window();
    // Position window to the top left of screen
    window.set_outer_position_pixels(0, 0);
    let win = window.rect();

    let draw = app.draw().x_y(-win.w()*0.5, -win.h()*0.5);
    draw.background().color(BLACK);

    let mut rng = rand::rng();
    let mut p = Point { x: 0.0, y: 0.0 };

    let mut counts: HashMap<String, u32> = HashMap::new();
    const ZOOM: f32 = 10.0;

    for _ in 0..30_000_000 {
        let r = rng.random_range(0.0..1.0);

        barnsley_fern(r, &mut p);
        let x = (p.x * win.w()) / ZOOM + win.w() * 0.5;
        let y = (p.y * win.h()) / ZOOM + win.h() * 0.5 - 525.0;

        let key = format!("{:.0},{:.0}", x, y);

        *counts.entry(key).or_insert(0) += 1;
    }

    let max_count = *counts.values().max().unwrap_or(&1);
    println!("max_count: {}", max_count);

    // Apply heatmap coloring
    for (key, count) in &counts {
        // Parse "x,y" string back to float values
        let coords: Vec<&str> = key.split(',').collect();
        let x: f32 = coords[0].parse().unwrap_or(0.0);
        let y: f32 = coords[1].parse().unwrap_or(0.0);

        let t = (*count as f32 / max_count as f32).powf(0.2);

        let mut r = 1.2 * t;
        let mut g = (1.5 + 190.0/255.0) * t;
        let mut b = 1.1 * t;

        let mut brightness = 1.0;
        let alpha = t.powf(0.1);

        if t > 0.4 {
            brightness = 0.6;
            r = 3.0 * t;
            g = 3.0 * t;
            b = 3.0 * t;
        // inner edge
        } else if t > 0.32 {
            brightness = 0.7;
            r = 2.0 * t;
            g = (2.0 + 190.0/255.0) * t;
            b = 2.0 * t;
        }

        let color = srgba(r * brightness, g * brightness, b * brightness, alpha);

        draw.rect()
            .x_y(x, y)
            .w_h(1.0, 1.0) // 1-pixel square
            .color(color);
    }
    
    draw.to_frame(app, &frame).unwrap();
}

fn barnsley_fern(r:f32, p: &mut Point) {
    if r < 0.01 { // Stem
        p.affine_transformations(0.0, 0.0, 0.0, 0.16, 0.0, 0.0);

    } else if r < 0.86 { // Successively smaller leaflets
        p.affine_transformations(0.85, 0.04, -0.04, 0.85, 0.0, 1.60);
        
    } else if r < 0.93 { // Largest left-hand leaflet
        p.affine_transformations(0.20, -0.26, 0.23, 0.22, 0.0, 1.60);
        
    } else { // Largest right-hand leaflet
        p.affine_transformations(-0.15, 0.28, 0.26, 0.24, 0.0, 0.44);
    }
}
