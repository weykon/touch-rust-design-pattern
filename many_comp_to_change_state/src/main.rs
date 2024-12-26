
use std::cell::RefCell;

pub struct State {
    num: i32,
    camera: Camera,
    init_comps: RefCell<Vec<Box<dyn Fn(&mut State)>>>,
}

trait ForState {
    fn for_state(state: &mut State);
}

struct Camera {}

struct Buffer {}

struct Texture {}

impl ForState for Camera {
    fn for_state(state: &mut State) {
        state.num += 1;
    }
}

impl ForState for Buffer {
    fn for_state(state: &mut State) {
        state.num += 1;
    }
}

impl State {
    fn new() -> Self {
        let mut state = State {
            num: 1,
            camera: Camera {},
            init_comps: RefCell::new(vec![]),
        };

        state.add_init_fn(Box::new(Camera::for_state));
        state.add_init_fn(Box::new(Buffer::for_state));

        state
    }

    fn add_init_fn(&self, init_fn: Box<dyn Fn(&mut State)>) {
        self.init_comps.borrow_mut().push(init_fn);
    }

    fn init(&self) {
        for init_fn in self.init_comps.borrow_mut().drain(..) {
            init_fn(self);
        }
    }
}

fn main() {
    let state = State::new();
    state.init();
}

