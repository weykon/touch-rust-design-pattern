pub struct State {
    num: i32,
    camera: Camera,
}

struct Camera {}
trait ForState {
    pub fn for_state(state: &mut State) {}
}

impl ForState for Camera {
    fn for_state(state: &mut State) {
        state.num += 1;
    }
}
impl State {
    fn new() {
        let camera = Camera {};
        Camera::for_state(self);
    }
}
