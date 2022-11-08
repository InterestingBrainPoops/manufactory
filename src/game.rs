use std::collections::HashMap;

pub struct Game {
    pub tick: u64,
    pub resources: HashMap<String, f64>,
}
