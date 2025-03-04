use std::{env, io, path};
use ggez::conf::{Conf, WindowMode};
use ggez::{event, ContextBuilder};
use crate::game::difficulty::Difficulty;
use crate::game::difficulty::Difficulty::{Easy, Hard};
use crate::game::state::State;

pub struct Game {
}

impl Game {
    pub fn start() -> Game {
        let difficulty = Self::difficultyHandler();

        let conf = Conf::new().window_mode(WindowMode {
            width: 1300.0,
            height: 700.0,
            ..Default::default()
        });

        let (mut ctx, event_loop) = ContextBuilder::new("Solitaire", "Hristo").default_conf(conf.clone()).build().unwrap();

        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");
            ctx.fs.mount(&path, true);
        }

        let state = State::new(&mut ctx, difficulty).unwrap();
        ctx.gfx.set_window_title("Solitaire");

        event::run(ctx, event_loop, state);
    }

    fn difficultyHandler() -> Difficulty {
        let mut string = String::new();

        while string.to_lowercase() != "hard" && string.to_lowercase() != "easy" {
            string.clear();
            println!("Difficulty: ");
            io::stdin().read_line(&mut string).unwrap();
            if string.contains('\n') {
                string.pop();
            }
            if string.contains('\r') {
                string.pop();
            }

        }
        let difficulty: Difficulty =
            match string.as_str() {
                "Hard" | "hard" => Hard,
                _ => Easy
            };
        difficulty
    }
}