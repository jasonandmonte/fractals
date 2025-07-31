use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    
    let window = app.main_window();
    let win = window.rect();
    // println!("t:{}", win.top());
    // println!("h:{}", win.h());
    // println!("w:{}", win.w());

    let draw = app.draw();
    draw.background().color(WHITE);
    
    lines(&draw, win);
    draw.to_frame(app, &frame).unwrap();
}

fn lines(draw: &Draw, win: Rect) {
    // Need height & width of window
    let swidth = win.w() as f32; // 1024
    let sheight = win.h() as f32; // 768
    let n = 100.0;
    let horizontal_stride = swidth / n; // 10.24
    let vertical_stride = sheight / n; // 7.68


    for i in 0..(n as u32) {
        let i = i as f32;
        let h = win.top() - vertical_stride * i;
        let w = win.left() + horizontal_stride * (i+1.0);

        let start_point = pt2(win.left(), h);
        let end_point   = pt2(w, win.bottom());

        draw.line()
            .start(start_point)
            .end(end_point)
            .weight(1.0)
            .color(BLACK);
    }
}

