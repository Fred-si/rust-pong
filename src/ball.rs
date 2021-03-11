#[derive(Clone, Copy)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    max_y: f32,
    min_y: f32,
}

impl Ball {
    pub fn new(context_height: f32) -> Self {
        // x: 0, y: 0 is the center of context
        let max_y = context_height / 2.0;

        Self {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            max_y,
            min_y: -max_y,
        }
    }
    pub fn update(&mut self) {
        // TODO: collision with player
        self.x += self.vx;
        self.y += self.vy;

        // bounce against top and bottom border
        if self.y >= self.max_y || self.y <= self.min_y {
            self.vy = -self.vy;

            self.y = if self.y >= self.max_y {
                self.max_y - (self.y - self.max_y)
            } else {
                self.min_y - (self.y - self.min_y)
            };
        }
    }
    pub fn serialize(&self) -> [u8; 16] {
        let x = self.x.to_le_bytes();
        let y = self.y.to_le_bytes();
        let vx = self.vx.to_le_bytes();
        let vy = self.vy.to_le_bytes();
        [
            x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3], vx[0], vx[1], vx[2], vx[3], vy[0],
            vy[1], vy[2], vy[3],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vy_should_be_unchanged_if_no_collision_occurs() {
        let mut ball = Ball::new(1.0);
        ball.vy = 0.1;
        ball.update();

        assert_eq!(ball.vy, 0.1);
    }
    #[test]
    fn vy_should_be_reversed_if_collision_with_top_border_occurs() {
        let mut ball = Ball::new(1.0);
        ball.vy = 1.0;
        ball.update();

        assert_eq!(ball.vy, -1.0);
    }
    #[test]
    fn ball_should_bounce_against_the_top_border() {
        let mut ball = Ball::new(1.0);
        ball.vy = 1.0;
        ball.update();

        assert_eq!(ball.y, 0.0);
    }
    #[test]
    fn vy_should_be_reversed_if_collision_with_bottom_border_occurs() {
        let mut ball = Ball::new(1.0);
        ball.vy = -1.0;
        ball.update();

        assert_eq!(ball.vy, 1.0);
    }
    #[test]
    fn ball_should_bounce_against_the_bottom_border() {
        let mut ball = Ball::new(1.0);
        ball.vy = -1.0;
        ball.update();

        assert_eq!(ball.y, 0.0);
    }
}
