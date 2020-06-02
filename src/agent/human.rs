use crate::*;

pub struct HumanAgent {
    events_loop: EventsLoop,
}

impl HumanAgent {
    #[allow(dead_code)]
    pub fn new(events_loop: EventsLoop) -> HumanAgent {
        HumanAgent { events_loop }
    }
}

impl Agent for HumanAgent {
    fn run(
        &mut self,
        _draw: DrawConfig,
        ctx: &mut Context,
        state: &mut TetrisState,
    ) -> GameResult<()> {
        // Simply hand off to the built-in event loop, as it will handle
        // keyboard input for us. For the same reason we do not need to
        // implement the get_action function.
        event::run(ctx, &mut self.events_loop, state)
    }
}
