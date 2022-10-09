mod pendulum;

use crate::pendulum::*;
use nannou::prelude::*;

struct Model {
    double_pendulum: DoublePendulum,
    parallel_double_pendulum: DoublePendulum,
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn model(app: &App) -> Model {
    let boundaries = app.window_rect();

    let origin = vec2(boundaries.x(), boundaries.y());
    let position = vec2(0.0, 0.0);

    let first_pendulum = Pendulum {
        angle: PI / 2.0,
        radius: 120.0,
        position,
        angular_acceleration: 0.0,
        angular_velocity: 0.0,
        mass: 3.0,
    };

    let second_pendulum = Pendulum {
        angle: PI / 2.0,
        radius: 160.0,
        position,
        angular_acceleration: 0.0,
        angular_velocity: 0.0,
        mass: 2.2,
    };

    let parallel_first_pendulum = Pendulum { ..first_pendulum };

    let parallel_second_pendulum = Pendulum {
        radius: second_pendulum.radius - 0.004,
        ..second_pendulum
    };

    Model {
        double_pendulum: DoublePendulum {
            first_pendulum,
            second_pendulum,
            origin,
            gravity: 0.5,
            path: Vec::new(),
        },
        parallel_double_pendulum: DoublePendulum {
            first_pendulum: parallel_first_pendulum,
            second_pendulum: parallel_second_pendulum,
            origin,
            gravity: 0.5,
            path: Vec::new(),
        },
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.double_pendulum.update();
    model.parallel_double_pendulum.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().rotate(PI);

    draw.background().color(rgb8(27, 36, 48)); // rgb(27, 36, 48)

    draw.ellipse()
        .color(rgb8(214, 213, 168)) // rgb(214, 213, 168)
        .w_h(5.0, 5.0);

    model.double_pendulum.draw(&draw);
    model.parallel_double_pendulum.draw(&draw);

    let points = model.double_pendulum.path.iter().map(|p| p.to_array());
    let parallel_points = model
        .parallel_double_pendulum
        .path
        .iter()
        .map(|p| p.to_array());

    draw.polyline()
        .weight(2.0)
        .color(rgb8(211, 211, 211)) // rgb(211, 211, 211)
        .points(points);

    draw.polyline()
        .weight(2.0)
        .color(rgb8(100, 200, 255)) // rgb(100, 200, 255)
        .points(parallel_points);

    draw.to_frame(app, &frame).unwrap();
}
