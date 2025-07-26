//! Koch Curve
//! IFS Paradigm
//!

use nannou::prelude::*;
use rand::prelude::*;

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

fn main() {
    nannou::app(model).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}


fn view(app: &App, _model: &Model, frame: Frame) {
    // NOTE: Can use to wait of user input
    // app.set_loop_mode(LoopMode::wait());
    app.set_loop_mode(LoopMode::loop_ntimes(1));
    let window = app.main_window();
    let win = window.rect();

    let draw = app.draw();

    let mut rng = rand::rng();
    let mut p = Point { x: 1.0, y: 1.0 };

    for _ in 0..1_000_000 {
        let r = rng.random_range(0.0..1.0);

        koch(r, &mut p);

        draw.ellipse()
            .x_y(p.x*win.w()+win.left(), p.y*win.h()+win.bottom())
            .radius(0.6);
    }
    
    draw.to_frame(app, &frame).unwrap();
}

fn koch(r:f32, p: &mut Point) {
    if r < 0.25 {
        p.scale(1.0/3.0, 1.0/3.0);

    } else if r < 0.5 {
        p.scale(1.0/3.0, 1.0/3.0);
        p.rotate(60.0);
        p.translate(1.0/3.0, 0.0);

    } else if r < 0.75 {
        p.scale(1.0/3.0, 1.0/3.0);
        p.rotate(-60.0);
        p.translate(0.5, (3.0).sqrt()/6.0);
        
    } else {
        p.scale(1.0/3.0, 1.0/3.0);
        p.translate(2.0/3.0, 0.0);
        
    }
}