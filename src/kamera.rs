pub struct Kamera {
    x: f32,
    y: f32,
    zoom: f32
}

impl Kamera {
    pub fn new() -> Kamera {
        Kamera {
            x: 0.0,
            y: 0.0,
            zoom: 1.0
        }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn change_zoom(&mut self, zoom: f32) {
        self.zoom += zoom;
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }
}