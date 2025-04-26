pub struct Tower {
    pub position: (f32, f32),
    pub health: i32,
}

impl Tower {
    pub fn new(path: Vec<(f32, f32)>) -> Self {
        Self {
            position: (0.0, 0.0),
            health: 100,
        }
    }

}
