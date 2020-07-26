use nannou::prelude::*;
use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
struct StyledRect {
    rect: Rect,
    fill_color: Rgb8,
    stroke_weight: f32,
    stroke_color: Rgb8,
}

impl StyledRect {
    fn new(rect: Rect) -> Self {
        Self {
            rect,
            fill_color: WHITE,
            stroke_weight: 1.0,
            stroke_color: BLACK,
        }
    }
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run()
}

struct Model {
    grid: Vec<StyledRect>,
}

fn model(app: &App) -> Model {
    let win = app.main_window().rect();
    let square_size = 16.0;
    let grid = build_grid(&win, square_size);

    Model { grid }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple: _ } => {
            let position = app.mouse.buttons.left().if_down();
            match position {
                Some(position) => {
                    let rects = &mut model.grid;
                    for rect in rects {
                        if rect.rect.contains(position) {
                            let rgb = generate_random_color();
                            rect.stroke_color = Rgb8::new(rgb.0, rgb.1, rgb.2);
                            rect.fill_color = Rgb8::new(rgb.0, rgb.1, rgb.2);
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.main_window();
    let _win = window.rect();

    draw.background().rgb(0.11, 0.12, 0.13);

    draw_squares(&draw, &model.grid);

    draw.to_frame(app, &frame).unwrap();
}

fn build_grid(win: &Rect, square_size: f32) -> Vec<StyledRect> {
    let mut grid: Vec<StyledRect> = vec![];
    let step_by = || (0..).map(|i| i as f32 * square_size);
    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    let x_iter = r_iter.chain(l_iter);
    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    let y_iter = t_iter.chain(b_iter);

    for x in x_iter {
        for y in y_iter.clone() {
            let rect = Rect::from_x_y_w_h(x, y, square_size, square_size);
            let styled_rect = StyledRect::new(rect);
            grid.push(styled_rect);
        }
    }

    grid
}

fn draw_squares(draw: &Draw, squares: &Vec<StyledRect>) {
    for &square in squares {
        draw.quad()
            .points(
                square.rect.top_left(),
                square.rect.top_right(),
                square.rect.bottom_right(),
                square.rect.bottom_left(),
            )
            .stroke_weight(square.stroke_weight)
            .stroke_color(square.stroke_color)
            .color(square.fill_color);
    }
}

fn generate_random_color() -> (u8, u8, u8) {
    let mut rng = thread_rng();
    let r = rng.gen_range(0, 255);
    let g = rng.gen_range(0, 255);
    let b = rng.gen_range(0, 255);

    (r, g, b)
}
