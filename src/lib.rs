const GAME_TRY: usize = 10;
const CODE_SIZE: u8 = 4;

#[derive(Clone, Copy)]
pub enum Color {
    Black = 0,
    White = 1,
    Yellow = 2,
    Blue = 3,
    Red = 4,
    Green = 5,
}

#[derive(Clone, Copy)]
pub struct Code([Color; CODE_SIZE as usize]);

impl Code {
    pub fn evaluate(&self, _: &Code) -> (u8, u8) {
        (0, 4)
    }
}

#[derive(Clone, Copy)]
pub struct Try(Code, (u8, u8));

pub struct Playable {
    pub tries: Vec<Try>,
}
enum Result {
    Win,
    Loose,
}
pub struct Finish(Code, Result, Vec<Try>);

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
            secret_code: Code([Color::Black, Color::Black, Color::Black, Color::Black]),
        }
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn play(&mut self, code: Code) -> &State {
        match &mut self.state {
            State::Playable(playable) => {
                let result = self.secret_code.evaluate(&code);
                playable.tries.push(Try(code, result));
                match (result, playable.tries.len()) {
                    ((CODE_SIZE, _), _) => {
                        self.state = State::Finish(Finish(
                            self.secret_code,
                            Result::Win,
                            playable.tries.clone(),
                        ))
                    }
                    ((_, _), GAME_TRY) => {
                        self.state = State::Finish(Finish(
                            self.secret_code,
                            Result::Loose,
                            playable.tries.clone(),
                        ))
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
mod tests {
    use crate::*;
    #[test]
    fn combianaison() {
        let mut game = Game::new();
        let state = game.play(Code([
            Color::White,
            Color::White,
            Color::White,
            Color::White,
        ]));
        match state {
            State::Playable(playable) => assert_eq!(playable.tries[0].1 .1, 4),
            _ => panic!(),
        }
    }
}
