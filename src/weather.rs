use macroquad::prelude::*;
use macroquad::audio::*;

// ---------------------------
// Raindrop
// ---------------------------
pub struct Raindrop {
    pub x: f32,
    pub y: f32,
    speed: f32,
    pub length: f32,
}

impl Raindrop {
    pub fn new() -> Self {
        Self {
            x: rand::gen_range(0., screen_width()),
            y: rand::gen_range(-500., 0.),
            speed: rand::gen_range(300., 600.),
            length: rand::gen_range(10., 20.),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.y += self.speed * dt;
    }

    pub fn draw(&self, brightness: f32) {
        let c = 0.5 + brightness * 0.5;
        draw_line(self.x, self.y, self.x, self.y + self.length, 2.0, Color::new(c, c, c, 1.0));
    }

    pub fn is_near_ground(&self) -> bool {
        let ground_level = screen_height() * 0.8;
        self.y + self.length >= ground_level && rand::gen_range(0.0, 1.0) < 0.1
    }
}

// ---------------------------
// Splash
// ---------------------------
pub struct Splash {
    x: f32,
    y: f32,
    radius: f32,
    lifetime: f32,
}

impl Splash {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, radius: 2.0, lifetime: 0.5 }
    }

    pub fn update(&mut self, dt: f32) {
        self.radius += 20.0 * dt;
        self.lifetime -= dt;
    }

    pub fn draw(&self, brightness: f32) {
        let alpha = self.lifetime.clamp(0.0, 0.5) / 0.5;
        let b = 1.0 + brightness * 0.5;
        draw_line(self.x, self.y, self.x + self.radius, self.y, 2.0, Color::new(b, b, b, alpha));
    }

    pub fn is_dead(&self) -> bool {
        self.lifetime <= 0.0
    }
}

// ---------------------------
// LightningBolt
// ---------------------------
pub struct LightningBolt {
    points: Vec<(f32, f32)>,
    lifetime: f32,
}

impl LightningBolt {
    pub fn new() -> Self {
        let mut points = Vec::new();
        let mut x = rand::gen_range(screen_width() * 0.2, screen_width() * 0.8);
        let mut y = 0.0;
        points.push((x, y));

        while y < screen_height() * 0.7 {
            let (dx, dy) = if y < screen_height() * 0.4 {
                // Top 40%: mostly horizontal
                (
                    rand::gen_range(-50.0, 50.0),
                    rand::gen_range(5.0, 15.0),
                )
            } else {
                // Lower 60%: mostly vertical
                (
                    rand::gen_range(-15.0, 15.0),
                    rand::gen_range(20.0, 40.0),
                )
            };
            x = (x + dx).clamp(0.0, screen_width());
            y += dy;
            points.push((x, y));
        }

        Self {
            points,
            lifetime: 0.15,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.lifetime -= dt;
    }

    pub fn draw(&self) {
        let alpha = self.lifetime.clamp(0.0, 0.15) / 0.15;
        for i in 0..self.points.len() - 1 {
            let (x1, y1) = self.points[i];
            let (x2, y2) = self.points[i + 1];
            let brightness = rand::gen_range(0.8, 1.0);
            draw_line(
                x1,
                y1,
                x2,
                y2,
                2.0,
                Color::new(brightness, brightness, brightness, alpha),
            );
        }
    }

    pub fn is_dead(&self) -> bool {
        self.lifetime <= 0.0
    }
}

// ---------------------------
// LightningManager
// ---------------------------
pub struct LightningManager {
    bolts: Vec<LightningBolt>,
    timer: f32,
    cooldown: f32,
    flashes: u8,
    flash_timer: f32,
    thunder_sound: Sound,
}

impl LightningManager {
    pub fn new(thunder_sound: Sound) -> Self {
        Self {
            bolts: vec![],
            timer: 0.0,
            cooldown: rand::gen_range(13.0, 20.0),
            flashes: 0,
            flash_timer: 0.0,
            thunder_sound,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.timer += dt;

        if self.timer > self.cooldown && self.flashes == 0 {
            self.bolts.push(LightningBolt::new());
            play_sound(&self.thunder_sound, PlaySoundParams { looped: false, volume: 1.0 });
            self.flashes = rand::gen_range(2, 4);
            self.flash_timer = 0.1;
            self.timer = 0.0;
        }

        if self.flashes > 0 {
            self.flash_timer -= dt;
            if self.flash_timer <= 0.0 {
                self.flashes -= 1;
                if self.flashes > 0 {
                    self.bolts.push(LightningBolt::new());
                    self.flash_timer = rand::gen_range(0.05, 0.15);
                }
            }
        }

        for bolt in self.bolts.iter_mut() {
            bolt.update(dt);
        }

        self.bolts.retain(|b| !b.is_dead());

        if self.bolts.is_empty() && self.flashes == 0 {
            self.cooldown = rand::gen_range(4.0, 8.0);
        }
    }

    pub fn draw(&self) {
        for bolt in &self.bolts {
            bolt.draw();
        }
    }

    pub fn is_flashing(&self) -> bool {
        !self.bolts.is_empty()
    }
}