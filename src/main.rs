mod tetris;
use tetris::*;

mod agent;
use agent::*;

mod misc;
use misc::*;

pub enum DrawConfig {
    AllFrame,
    NoFrames,
    EveryNFrames(u64),
}

// Only affects non-human agents
const DRAW_CONFIG: DrawConfig = DrawConfig::AllFrame;
// const DRAW_CONFIG: DrawConfig = DrawConfig::NoFrames;
// const DRAW_CONFIG: DrawConfig = DrawConfig::EveryNFrames(1);

fn main() -> GameResult<()> {
    let config = conf::Conf {
        window_setup: conf::WindowSetup {
            title: "Genetic Tetris".to_string(),
            samples: conf::NumSamples::Zero,
            icon: "".to_string(),
            vsync: true,
            srgb: true,
        },
        backend: conf::Backend::OpenGL { major: 4, minor: 5 },
        modules: Default::default(),
        window_mode: conf::WindowMode {
            borderless: false,
            max_height: 0.0,
            min_height: 0.0,
            max_width: 0.0,
            min_width: 0.0,
            width: PIECE_SIZE * BOARD_WIDTH as f32,
            height: PIECE_SIZE * BOARD_HEIGHT as f32,
            fullscreen_type: conf::FullscreenType::Windowed,
            maximized: false,
            resizable: false,
        },
    };

    let (mut ctx, mut _events_loop) = ContextBuilder::new("GeneticTetris", "Dario Sucic")
        .conf(config)
        .build()
        .unwrap();

    if let DrawConfig::NoFrames = DRAW_CONFIG {
        drop(_events_loop);
    }

    let eval_iterations = 5;
    let num_generations = 3;
    let population_size = 500;
    let selection_size = population_size / 10;
    let mutation_probability = 0.15;

    println!("Training with hyperparameters:");
    println!("-  eval_iterations: {}", eval_iterations);
    println!("-  num_generations: {}", num_generations);
    println!("-  population_size: {}", population_size);
    println!("-  selection_size: {}", selection_size);
    println!("-  mutation_probability: {}", mutation_probability);
    println!();

    let (mut agent, score) = GeneticAgent::from_genetic(
        &mut ctx,
        eval_iterations,
        num_generations,
        population_size,
        selection_size,
        mutation_probability,
    )?;

    println!("Weights after training: {:?}", agent.weights);
    println!("Average training score: {}", score);

    let mut state = TetrisState::new(&mut ctx);
    agent.run(DRAW_CONFIG, &mut ctx, &mut state)?;

    println!("Achieved Score: {}", state.score);

    Ok(())
}

impl EventHandler for TetrisState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.is_over || !self.is_valid_move(self.pos, &self.current_piece) {
            self.is_over = true;
            return Ok(());
        }

        self.drop_count += 1;
        self.sub_count += 1;

        let per_second = self.drop_count % 144 == 0;
        let per_sub = self.sub_count % 24 == 0;

        if per_sub {
            self.pressed_map.insert(KeyCode::Left, false);
            self.pressed_map.insert(KeyCode::Right, false);
            if !per_second {
                self.pressed_map.insert(KeyCode::Down, false);
            }
        }

        if per_second {
            let mut new_pos = self.pos;
            new_pos[1] += 1;

            if self.is_valid_move(new_pos, &self.current_piece) {
                self.pos = new_pos;
            } else {
                self.lock_current_piece();
                self.current_piece = self.next_piece;
                self.next_piece = Piece::random(&mut self.rng);
                self.pos = [BOARD_WIDTH as i32 / 2 - 2, PIECE_SPAWN_OFFSET];
                self.propagate_lines();
                self.score += 1;
            }
        }

        self.rotation_handler(ctx);
        self.movement_handler(ctx);
        self.drop_handler(ctx);
        self.update_ghost_pos();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let rect = graphics::Rect::new(
            0.0,
            UNIT * PIECE_SPAWN_OFFSET as f32,
            UNIT * BOARD_WIDTH as f32,
            1.0,
        );

        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            Color::new(0.6, 0.0, 0.0, 1.0),
        )
        .unwrap();

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        self.current_piece.draw(ctx, self.pos, 1.0)?;
        self.current_piece.draw(ctx, self.ghost_pos, 0.25)?;
        self.draw_map(ctx)?;

        let score_text = graphics::Text::new(format!("Score: {}", self.score));
        graphics::draw(
            ctx,
            &score_text,
            graphics::DrawParam::new().color(Color::from_rgb(0, 0, 0)),
        )?;

        if self.is_over {
            self.fill_screen(ctx, Color::new(0.0, 0.0, 0.0, 0.9))?;
            self.draw_centered(ctx, "Game Over!", Color::new(1.0, 0.3, 0.3, 1.0))?;
        }

        graphics::present(ctx)
    }
}
