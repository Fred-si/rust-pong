#[derive(Clone, Copy)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    max_y: f32,
    min_y: f32,
    max_x: f32,
    min_x: f32,
}

pub trait Racket {
    fn get_bounce_value_at(&self, _y: f32) -> Option<f32>;
}

impl Ball {
    pub fn new(context_height: f32, context_width: f32) -> Self {
        // x: 0, y: 0 is the center of context
        let max_y = context_height / 2.0;
        let max_x = context_width / 2.0;

        Self {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            max_y,
            min_y: -max_y,
            max_x,
            min_x: -max_x,
        }
    }
    pub fn update(&mut self, _rackets: [&dyn Racket; 2]) {
        // TODO: collision with player
        self.x += self.vx;
        self.y += self.vy;

        // bounce against players
        if self.x >= self.max_x || self.x <= self.min_x {
            self.vx = -self.vx;

            self.x = if self.x >= self.max_x {
                self.max_x - (self.x - self.max_x)
            } else {
                self.min_x - (self.x - self.min_x)
            };
        }

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

    struct PlayerStub {}

    impl Racket for PlayerStub {
        fn get_bounce_value_at(&self, _y: f32) -> Option<f32> {
            Some(0.0)
        }
    }

    fn get_player_stubs<'a>() -> [&'a dyn Racket; 2] {
        [&PlayerStub {}, &PlayerStub {}]
    }

    mod bounce_against_border {
        use super::*;

        #[test]
        fn vy_should_be_unchanged_if_no_collision_occurs() {
            let mut ball = Ball::new(1.0, 1.0);
            ball.vy = 0.1;
            ball.update(get_player_stubs());

            assert_eq!(ball.vy, 0.1);
        }
        #[test]
        fn vy_should_be_reversed_if_collision_with_top_border_occurs() {
            let mut ball = Ball::new(1.0, 1.0);
            ball.vy = 1.0;
            ball.update(get_player_stubs());

            assert_eq!(ball.vy, -1.0);
        }
        #[test]
        fn ball_should_bounce_against_the_top_border() {
            let mut ball = Ball::new(1.0, 1.0);
            ball.vy = 1.0;
            ball.update(get_player_stubs());

            assert_eq!(ball.y, 0.0);
        }
        #[test]
        fn vy_should_be_reversed_if_collision_with_bottom_border_occurs() {
            let mut ball = Ball::new(1.0, 1.0);
            ball.vy = -1.0;
            ball.update(get_player_stubs());

            assert_eq!(ball.vy, 1.0);
        }
        #[test]
        fn ball_should_bounce_against_the_bottom_border() {
            let mut ball = Ball::new(1.0, 1.0);
            ball.vy = -1.0;
            ball.update(get_player_stubs());

            assert_eq!(ball.y, 0.0);
        }
    }
    mod bounce_against_player {
        use super::*;

        #[test]
        fn vx_should_be_unchanged_if_no_collision_occurs() {
            let mut ball = Ball::new(1.0, 1.0);
            ball.vx = 0.1;
            ball.update(get_player_stubs());

            assert_eq!(ball.vx, 0.1);
            assert_eq!(ball.x, 0.1);
        }
        #[test]
        fn ball_should_bounce_against_right_player() {
            let mut ball = Ball::new(1.0, 1.0);
            ball.vx = 1.0;
            ball.update(get_player_stubs());

            assert_eq!(ball.x, 0.0);
        }
        #[test]
        fn ball_should_bounce_against_left_player() {
            let mut ball = Ball::new(1.0, 1.0);
            ball.vx = -1.0;
            ball.update(get_player_stubs());

            assert_eq!(ball.x, 0.0);
        }
        #[test]
        fn ball_should_go_out_of_context_if_there_is_no_player_on_its_trajectory() {
            let mut ball = Ball::new(1.0, 1.0);
            ball.vx = 1.0;
            ball.update(get_player_stubs());

            assert_eq!(ball.x, 1.0);
        }
    }
}
