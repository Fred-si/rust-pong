use crate::ball::Ball;
use crate::player::Player;
use std::sync::{Arc, Mutex};

pub type Places = [Place; 2];
pub type SharedContext = Arc<Mutex<Context>>;

// Temporary value for waiting to choose how to set context size
// TODO: delete this
const CONTEXT_HEIGHT: f32 = 10.0;
const CONTEXT_WIDTH: f32 = 10.0;

#[derive(Clone, Copy)]
pub struct Context {
    pub places: Places,
    pub ball: Ball,
}

#[derive(Clone, Copy)]
pub enum Place {
    Empty,
    Player(Player),
}

impl Context {
    pub fn new() -> Self {
        Self {
            places: [Place::Empty; 2],
            ball: Ball::new(CONTEXT_HEIGHT, CONTEXT_WIDTH),
        }
    }
    pub fn update(&mut self) {
        if let Place::Player(mut player) = self.places[0] {
            player.update();
        }
        if let Place::Player(mut player) = self.places[1] {
            player.update();
        }
        if let [Place::Player(left), Place::Player(right)] = self.places {
            self.ball.update([left, right]);
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(32);
        let player0 = match self.places[0] {
            Place::Empty => [0; 8],
            Place::Player(player) => player.serialize(),
        };
        let player1 = match self.places[1] {
            Place::Empty => [0; 8],
            Place::Player(player) => player.serialize(),
        };
        let ball = self.ball.serialize();

        output.extend(&player0);
        output.extend(&player1);
        output.extend(&ball);

        output
    }
}
