pub struct Enemy {
    pub position: (f32, f32),
    pub speed: f32,
    pub health: i32,
    pub path: Vec<(f32, f32)>,
    pub current_target: usize,
    pub finished: bool,
}

impl Enemy {
    pub fn new(path: Vec<(f32, f32)>) -> Self {
        Self {
            position: path[0],
            speed: 5.0,
            health: 100,
            path,
            current_target: 1,
            finished: false,
        }
    }

    pub fn update(&mut self) {
        if self.finished || self.current_target >= self.path.len() {
            self.finished = true;
            return;
        }

        let (tx, ty) = self.path[self.current_target]; // Target
        let (x, y) = self.position;
        let dx = tx - x;
        let dy = ty - y;
        let dist = (dx * dx + dy * dy).sqrt(); // Distance to Target

        if dist < self.speed {
            // When close enough, jump on Target and use new Target
            self.position = (tx, ty);
            self.current_target += 1;
        } else {
            // Move towards Target
            self.position.0 += self.speed * dx / dist;
            self.position.1 += self.speed * dy / dist;
        }
    }
}
