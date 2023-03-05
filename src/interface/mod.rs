use crate::Engine;

pub struct Interface {
    engine: Engine,
}

impl Interface {
    pub fn run(engine: Engine) {
        let interface = Self {
            engine
        };
        drop(interface);
    }
}