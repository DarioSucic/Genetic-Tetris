use crate::*;

pub(crate) use ggez::{
    event::{EventHandler, EventsLoop},
    Context, GameResult,
};

fn simulate_key_press(key: KeyCode, ctx: &mut Context, state: &mut TetrisState) -> GameResult<()> {
    let press_event = gen_key_event(key, true);
    ctx.process_event(&press_event);

    state.update(ctx)?;

    let release_event = gen_key_event(key, false);
    ctx.process_event(&release_event);

    Ok(())
}

pub trait Agent {
    fn run(
        &mut self,
        draw: DrawConfig,
        ctx: &mut Context,
        state: &mut TetrisState,
    ) -> GameResult<()> {
        while !state.is_over {
            if let Some(key) = self.get_action(state) {
                simulate_key_press(key, ctx, state)?;
            } else {
                simulate_key_press(KeyCode::Space, ctx, state)?;
            }

            state.update(ctx)?;

            match draw {
                DrawConfig::AllFrame => state.draw(ctx)?,
                DrawConfig::NoFrames => (),
                DrawConfig::EveryNFrames(n) => {
                    if state.drop_count % n == 0 {
                        state.draw(ctx)?
                    }
                }
            }
        }

        Ok(())
    }

    fn get_action(&mut self, _state: &mut TetrisState) -> Option<KeyCode> {
        unimplemented!()
    }

    fn evaluate(
        &mut self,
        state: &mut TetrisState,
        ctx: &mut Context,
        n: usize,
    ) -> GameResult<f64> {
        let mut scores = vec![0; n];

        state.reset();
        for i in 0..n {
            self.run(DrawConfig::NoFrames, ctx, state)?;
            scores[i] = state.score;
            state.reset();
        }

        // Ok(scores[scores.len() / 2] as f64)           // median
        // Ok(*scores.iter().min().unwrap() as f64)      // min
        Ok(scores.iter().sum::<u32>() as f64 / n as f64) // mean
    }
}

mod human;
pub use human::HumanAgent;

mod random;
pub use random::RandomAgent;

mod genetic;
pub use genetic::GeneticAgent;
