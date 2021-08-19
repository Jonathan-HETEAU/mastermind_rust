pub const GAME_TRY: usize = 10;
pub const CODE_SIZE: u8 = 4;
pub const COLORS_NBR: usize = 6;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Color {
    Black,
    White,
    Yellow,
    Blue,
    Red,
    Green,
}

impl Color {
    pub fn value(&self) -> usize {
        match self {
            Color::Black => 0,
            Color::White => 1,
            Color::Yellow => 2,
            Color::Blue => 3,
            Color::Red => 4,
            Color::Green => 5,
        }
    }
    pub fn from_value(value: usize) -> Color {
        match value {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Yellow,
            3 => Color::Blue,
            4 => Color::Red,
            _ => Color::Green,
        }
    }
}

pub type Code = [Color; CODE_SIZE as usize];

pub fn evaluate(source: &Code, code: &Code) -> (u8, u8) {
    let mut leftover_self: [u8; COLORS_NBR] = [0; COLORS_NBR];
    let mut leftover_code: [u8; COLORS_NBR] = [0; COLORS_NBR];

    let mut good: u8 = 0;
    let mut bad: u8 = 0;

    for i in 0..(CODE_SIZE as usize) {
        let color_self = source[i];
        let color_code = code[i];
        if color_self == color_code {
            good += 1;
        } else {
            leftover_self[color_self.value()] += 1;
            leftover_code[color_code.value()] += 1;
        }
    }
    for i in 0..COLORS_NBR {
        bad += u8::min(leftover_self[i], leftover_code[i]);
    }

    (good, bad)
}

#[derive(Clone, Copy)]
pub struct Try {
    pub code: Code,
    pub good: u8,
    pub bad: u8,
}

pub struct Playable {
    pub tries: Vec<Try>,
}
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Result {
    Win,
    Loose,
}
pub struct Finish {
    pub code: Code,
    pub result: Result,
    pub tries: Vec<Try>,
}

pub enum State {
    Playable(Playable),
    Finish(Finish),
}

pub struct Game {
    secret_code: Code,
    state: State,
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: State::Playable(Playable { tries: Vec::new() }),
            secret_code: [
                Color::from_value(rand::random::<usize>() % COLORS_NBR),
                Color::from_value(rand::random::<usize>() % COLORS_NBR),
                Color::from_value(rand::random::<usize>() % COLORS_NBR),
                Color::from_value(rand::random::<usize>() % COLORS_NBR),
            ],
        }
    }

    pub fn new_with_secret_code(code: Code) -> Self {
        Game {
            state: State::Playable(Playable { tries: Vec::new() }),
            secret_code: code,
        }
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn play(&mut self, code: Code) -> &State {
        match &mut self.state {
            State::Playable(playable) => {
                let result = evaluate(&self.secret_code, &code);
                playable.tries.push(Try {
                    code: code,
                    good: result.0,
                    bad: result.1,
                });
                match (result, playable.tries.len()) {
                    ((CODE_SIZE, _), _) => {
                        self.state = State::Finish(Finish {
                            code: self.secret_code,
                            result: Result::Win,
                            tries: playable.tries.clone(),
                        })
                    }
                    ((_, _), GAME_TRY) => {
                        self.state = State::Finish(Finish {
                            code: self.secret_code,
                            result: Result::Loose,
                            tries: playable.tries.clone(),
                        })
                    }
                    ((_, _), _) => (),
                }
            }
            State::Finish(_) => (),
        };
        &self.state
    }
}

#[cfg(test)]
mod tests_play {
    use crate::*;
    #[test]
    fn win() {
        let code = [
            Color::from_value(rand::random::<usize>() % COLORS_NBR),
            Color::from_value(rand::random::<usize>() % COLORS_NBR),
            Color::from_value(rand::random::<usize>() % COLORS_NBR),
            Color::from_value(rand::random::<usize>() % COLORS_NBR),
        ];

        let mut game = Game::new_with_secret_code(code);
        let state = game.play(code);
        match state {
            State::Finish(finish) => assert_eq!(finish.result, Result::Win),
            _ => panic!(),
        }
    }
    #[test]
    fn faux() {
        let mut game =
            Game::new_with_secret_code([Color::White, Color::White, Color::White, Color::White]);
        let state = game.play([Color::Black, Color::Black, Color::Black, Color::Black]);
        match state {
            State::Playable(playable) => {
                assert_eq!(playable.tries[0].good, 0);
                assert_eq!(playable.tries[0].bad, 0);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn mal_placer() {
        let mut game =
            Game::new_with_secret_code([Color::White, Color::Blue, Color::Green, Color::Black]);
        let state = game.play([Color::Black, Color::White, Color::Blue, Color::Green]);
        match state {
            State::Playable(playable) => {
                assert_eq!(playable.tries[0].good, 0);
                assert_eq!(playable.tries[0].bad, 4);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn loose() {
        let mut game =
            Game::new_with_secret_code([Color::White, Color::Blue, Color::Green, Color::Black]);
        let mut state = game.play([Color::Black, Color::White, Color::Blue, Color::Green]);

        for _ in 0..9 {
            state = game.play([Color::Black, Color::White, Color::Blue, Color::Green]);
        }

        match state {
            State::Finish(finish) => assert_eq!(finish.result, Result::Loose),
            _ => panic!(),
        }
    }
}
