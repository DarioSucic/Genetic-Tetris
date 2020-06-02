use crate::*;

mod heuristics;
pub use heuristics::*;

pub use rand::seq::SliceRandom;
pub use winit::{
    DeviceId, ElementState, Event, KeyboardInput, ModifiersState, WindowEvent, WindowId,
};

pub fn gen_key_event(key: KeyCode, pressed: bool) -> Event {
    let input = KeyboardInput {
        modifiers: ModifiersState::default(),
        scancode: 0,
        state: match pressed {
            true => ElementState::Pressed,
            false => ElementState::Released,
        },
        virtual_keycode: Some(key),
    };

    let event = WindowEvent::KeyboardInput {
        device_id: unsafe { DeviceId::dummy() },
        input,
    };

    Event::WindowEvent {
        window_id: unsafe { WindowId::dummy() },
        event,
    }
}
