use crate::*;

use rand::rngs::SmallRng;
use rand_distr::StandardNormal;

// Somewhat arbitrarily chosen
const STD_DEV: f64 = 100.0;

fn sample_std(rng: &mut SmallRng) -> f64 {
    let r: f64 = rng.sample(StandardNormal);
    r * STD_DEV
}

#[derive(Debug, Clone, Copy)]
pub struct GeneticAgent {
    pub weights: [f64; N_HEURISTICS],
}

impl GeneticAgent {
    // Initialize agent with random weights from a normal distribution [µ=0, σ=50]
    pub fn new() -> GeneticAgent {
        let mut weights: [f64; N_HEURISTICS] = [0.0; N_HEURISTICS];
        let mut rng = SmallRng::from_entropy();

        for w in weights.iter_mut() {
            *w = sample_std(&mut rng);
        }

        GeneticAgent { weights }
    }

    pub fn breed(&self, other: &GeneticAgent, p: f64) -> GeneticAgent {
        let mut weights = [0.0; N_HEURISTICS];
        let mut rng = SmallRng::from_entropy();

        for i in 0..N_HEURISTICS {
            let r = rng.gen_bool(p);
            weights[i] = if r { self.weights[i] } else { other.weights[i] };
        }

        GeneticAgent { weights }
    }

    #[allow(dead_code)]
    pub fn mutate_random_weight(&mut self, rng: &mut SmallRng) {
        if let Some(w) = self.weights.choose_mut(rng) {
            *w = sample_std(rng);
        }
    }

    pub fn nudge_random_weight(&mut self, rng: &mut SmallRng) {
        if let Some(w) = self.weights.choose_mut(rng) {
            *w += sample_std(rng) / STD_DEV;
        }
    }

    fn loss_function(&self, board: &TetrisBoard) -> f64 {
        self.weights
            .iter()
            .zip(&HEURISTICS)
            .fold(0.0, |acc, (w, h)| acc + w * h(board))
    }

    pub fn from_genetic(
        ctx: &mut Context,
        eval_iterations: usize,
        num_generations: usize,
        population_size: usize,
        selection_size: usize,
        mutation_probability: f64,
    ) -> GameResult<(GeneticAgent, f64)> {
        let mut population = (0..population_size)
            .map(|_| GeneticAgent::new())
            .collect::<Vec<GeneticAgent>>();

        let mut rng = SmallRng::from_entropy();
        let mut state = TetrisState::new(ctx); // Reuse to avoid allocations
        let mut fitness_values = vec![0.0; population_size];
        let mut selection = vec![population[0]; selection_size];

        let mut best_score = 0.0;
        let mut best_agent = population[0];

        for generation in 0..num_generations {
            // Calculate population's fitness values
            for (agent, fitness) in population.iter_mut().zip(fitness_values.iter_mut()) {
                *fitness = agent.evaluate(&mut state, ctx, eval_iterations)?;
            }

            // Keep track of fittest individual and score
            let (i, &score) = fitness_values
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap())
                .unwrap();

            best_score = score;
            best_agent = population[i];

            println!("Generation {} :: {}", generation+1, best_score);

            // Avoid unnecessary computation on the last iteration
            if generation == num_generations - 1 {
                break;
            }

            // Normalize fitness values
            let fitness_sum = fitness_values.iter().sum::<f64>();
            fitness_values.iter_mut().for_each(|x| *x /= fitness_sum);

            // Select individuals weighted by fitness.
            // Uses roulette wheel selection
            // Source: https://en.wikipedia.org/wiki/Selection_(genetic_algorithm)

            fn roulette_wheel<T: Copy>(a: &[T], ps: &[f64], r: f64) -> T {
                let mut total = 0.0;
                for (&x, p) in a.iter().zip(ps) {
                    total += p;
                    if total > r {
                        return x;
                    }
                }
                unreachable!("Roulette wheel called on unnormalized probabilities")
            }

            selection.iter_mut().for_each(|individual| {
                let r = rng.gen();
                *individual = roulette_wheel(&population, &fitness_values, r);
            });

            population.iter_mut().for_each(|individual| {
                let parent_a = selection.choose(&mut rng).unwrap();
                let parent_b = selection.choose(&mut rng).unwrap();
                let mut child = parent_a.breed(parent_b, 0.5);
                if rng.gen_bool(mutation_probability) {
                    child.nudge_random_weight(&mut rng);
                }
                *individual = child;
            });
        }

        Ok((best_agent, best_score))
    }

    #[allow(dead_code)]
    pub fn train_mutation_only(
        &mut self,
        ctx: &mut Context,
        n: usize,
        k: usize,
        mut best_score: f64,
    ) -> GameResult<f64> {
        let mut best_weights = self.weights;
        let mut state = TetrisState::new(ctx);

        let mut rng = SmallRng::from_entropy();

        for _ in 0..k {
            let average_score = self.evaluate(&mut state, ctx, n)?;

            if average_score > best_score {
                best_weights = self.weights;
                best_score = average_score;
            }

            self.mutate_random_weight(&mut rng);
        }
        self.weights = best_weights;
        Ok(best_score)
    }
}

impl Agent for GeneticAgent {
    fn get_action(&mut self, state: &mut TetrisState) -> Option<KeyCode> {
        if let Some(key) = state.pick_move_by_key(|board| self.loss_function(board)) {
            Some(key)
        } else {
            None
        }
    }
}
