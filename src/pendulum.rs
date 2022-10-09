use nannou::{prelude::{Vec2, rgb8}, Draw};

pub struct Pendulum {
  pub radius: f32,
  pub mass: f32,
  pub angle: f32,
  pub angular_velocity: f32,
  pub angular_acceleration: f32,
  pub position: Vec2,
}

pub struct DoublePendulum {
  pub first_pendulum: Pendulum,
  pub second_pendulum: Pendulum,
  pub gravity: f32,
  pub origin: Vec2,
  pub path: Vec<Vec2>,
}

impl DoublePendulum {
  pub fn update(&mut self) {
      let mut equation_first_part = -1.0
          * self.gravity
          * (2.0 * self.first_pendulum.mass + self.second_pendulum.mass)
          * self.first_pendulum.angle.sin();

      let mut equation_second_part = -1.0
          * self.second_pendulum.mass
          * self.gravity
          * (self.first_pendulum.angle - 2.0 * self.second_pendulum.angle).sin();

      let mut equation_third_part = -1.0
          * 2.0
          * (self.first_pendulum.angle - self.second_pendulum.angle).sin()
          * self.second_pendulum.mass;

      let mut equation_fourth_part = self.second_pendulum.angular_velocity.powf(2.0)
          * self.second_pendulum.radius
          + self.first_pendulum.angular_velocity.powf(2.0)
              * self.first_pendulum.radius
              * (self.first_pendulum.angle - self.second_pendulum.angle).cos();

      let mut equation_fifth_part = self.first_pendulum.radius
          * (2.0 * self.first_pendulum.mass + self.second_pendulum.mass
              - self.second_pendulum.mass
                  * (2.0 * self.first_pendulum.angle - 2.0 * self.second_pendulum.angle).cos());

      self.first_pendulum.angular_acceleration = (equation_first_part
          + equation_second_part
          + equation_third_part * equation_fourth_part)
          / equation_fifth_part;

      equation_first_part = 2.0 * (self.first_pendulum.angle - self.second_pendulum.angle).sin();
      equation_second_part = self.first_pendulum.angular_velocity.powf(2.0)
          * self.first_pendulum.radius
          * (self.first_pendulum.mass + self.second_pendulum.mass);

      equation_third_part = self.gravity
          * (self.first_pendulum.mass + self.second_pendulum.mass)
          * self.first_pendulum.angle.cos();

      equation_fourth_part = self.second_pendulum.angular_velocity.powf(2.0)
          * self.second_pendulum.radius
          * self.second_pendulum.mass
          * (self.first_pendulum.angle - self.second_pendulum.angle).cos();

      equation_fifth_part = self.second_pendulum.radius
          * (2.0 * self.first_pendulum.mass + self.second_pendulum.mass
              - self.second_pendulum.mass
                  * (2.0 * self.first_pendulum.angle - 2.0 * self.second_pendulum.angle).cos());

      self.second_pendulum.angular_acceleration = (equation_first_part
          * (equation_second_part + equation_third_part + equation_fourth_part))
          / equation_fifth_part;

      self.first_pendulum.angular_velocity += self.first_pendulum.angular_acceleration;
      self.first_pendulum.angle += self.first_pendulum.angular_velocity;

      self.second_pendulum.angular_velocity += self.second_pendulum.angular_acceleration;
      self.second_pendulum.angle += self.second_pendulum.angular_velocity;

      self.first_pendulum.position.x =
          self.first_pendulum.radius * self.first_pendulum.angle.sin();
      self.first_pendulum.position.y =
          self.first_pendulum.radius * self.first_pendulum.angle.cos();

      self.second_pendulum.position.x =
          self.second_pendulum.radius * self.second_pendulum.angle.sin();
      self.second_pendulum.position.y =
          self.second_pendulum.radius * self.second_pendulum.angle.cos();

      self.first_pendulum.position += self.origin;
      self.second_pendulum.position += self.first_pendulum.position;

      self.path.push(self.second_pendulum.position);
  }

  pub fn draw(&self, draw: &Draw) {
      // First Pendulum
      draw.line()
          .stroke_weight(2.0)
          .color(rgb8(214, 213, 168)) //rgb(214, 213, 168)
          .points(self.origin, self.first_pendulum.position);

      draw.ellipse()
          .color(rgb8(129, 103, 151)) // rgb(129, 103, 151)
          .x_y(
              self.first_pendulum.position.x,
              self.first_pendulum.position.y,
          )
          .w_h(
              self.first_pendulum.mass * 5.0,
              self.first_pendulum.mass * 5.0,
          );

      // Second Pendulum
      draw.line()
          .stroke_weight(2.0)
          .color(rgb8(114, 213, 168)) //rgb(114, 213, 168)
          .points(self.first_pendulum.position, self.second_pendulum.position);

      draw.ellipse()
          .color(rgb8(129, 103, 0)) // rgb(129, 103, 0)
          .x(self.second_pendulum.position.x)
          .y(self.second_pendulum.position.y)
          .w_h(
              self.second_pendulum.mass * 5.0,
              self.second_pendulum.mass * 5.0,
          );
  }
}
