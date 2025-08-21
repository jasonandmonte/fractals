use nannou::prelude::*;

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
    app.set_loop_mode(LoopMode::loop_ntimes(1));
    let window = app.main_window();
    let win = window.rect();

    let draw = app.draw();
    draw.background().color(BLACK);

    let p1 = pt2(win.left()+1.0, win.bottom()+10.0);
    let p2 =  win.mid_top();
    let p3 = pt2(win.right()-1.0, win.bottom()+10.0);
    let d = 10;
    sierpinski(&draw, p1, p2, p3, d);
    
    draw.to_frame(app, &frame).unwrap();
}

fn sierpinski(draw: &Draw, p1: Point2, p2: Point2, p3: Point2, depth: u32) {
    if depth < 1 {
        return;
    }

    // draw a line p1 -> p2
    draw.line()
        .start(p1)
        .end(p2)
        .weight(1.0)
        .color(WHITE);
    // draw a line p2 -> p3
    draw.line()
        .start(p2)
        .end(p3)
        .weight(1.0)
        .color(WHITE);
    // draw a line p3 -> p1
    draw.line()
        .start(p3)
        .end(p1)
        .weight(1.0)
        .color(WHITE);

    let mp1 = pt2((p1.x+p2.x)/2.0, (p1.y+p2.y)/2.0);
    let mp2 = pt2((p2.x+p3.x)/2.0, (p2.y+p3.y)/2.0);
    let mp3 = pt2((p3.x+p1.x)/2.0, (p3.y+p1.y)/2.0);

    sierpinski(&draw, p1, mp1, mp3, depth-1);
    sierpinski(&draw, mp1, p2, mp2, depth-1);
    sierpinski(&draw, mp3, mp2, p3, depth-1);

}

