pub struct Tower {
    pub position: (f32, f32),
    pub health: i32,
}

impl Tower {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            health: 100,
        }
    }

}
