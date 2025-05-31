use macroquad::prelude::*;
use macroquad::audio::*;

#[derive(PartialEq)]
pub enum AnimationState {
    Idle,
    Walking,
    Running,
    Shoting,
    Recharging,
    Death,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    width: f32,
    height: f32,
    speed: f32,
    frame: usize,
    frame_timer: f32,
    frame_speed: f32,
    state: AnimationState,
    idle_texture: Texture2D,
    walk_texture: Texture2D,
    run_texture: Texture2D,
    pub facing_left: bool,
    shot_texture: Texture2D,
    recharge_texture: Texture2D,
    is_recharging: bool,
    shots_fired: u32,
    is_shooting: bool,
    pub did_shoot: bool,
    is_hit: bool,
    hit_timer: f32,
    health: i32,
    pub is_dead: bool,
    death_texture: Texture2D,
    show_death_menu: bool,
    ammo_texture: Texture2D,
    pub score: u32,
    shoot_sound: Sound,
    death_sound: Sound,
    recharging_sound: Sound,
}

impl Player {
    pub fn new(idle_texture: Texture2D, walk_texture: Texture2D,run_texture: Texture2D,shot_texture: Texture2D,recharge_texture:Texture2D,death_texture: Texture2D,ammo_texture: Texture2D, shoot_sound:Sound, death_sound: Sound, recharging_sound: Sound) -> Self {
        let ground_level = screen_height() * 0.8;
        Self {
            x: screen_width() / 2.0,
            y: ground_level - 50.0,
            width: 64.0,
            height: 64.0,
            speed: 300.0,
            frame: 0,
            frame_timer: 0.0,
            frame_speed: 0.1,
            state: AnimationState::Idle,
            idle_texture,
            walk_texture,
            run_texture,
            shot_texture,
            recharge_texture,
            facing_left : false,
            is_recharging: false,
            is_shooting: false,
            shots_fired: 12,
            did_shoot: false,
            is_hit: false,
            hit_timer: 0.0,
            health: 100,
            is_dead: false,
            death_texture,
            show_death_menu: false,
            ammo_texture,
            score: 0,
            shoot_sound,
            death_sound,
            recharging_sound
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_dead {
            self.frame_timer += dt;
            if self.frame_timer > self.frame_speed {
                self.frame += 1;
                self.frame_timer = 0.0;

                if self.frame >= 4 {
                    self.show_death_menu = true; 
                    self.frame = 3;
                }
            }
            return;
        }

        if is_key_down(KeyCode::Space) && self.shots_fired > 0 && !self.is_recharging {
            if !self.is_shooting {
                self.state = AnimationState::Shoting;
                self.frame = 0;
                self.frame_timer = 0.0;
                self.is_shooting = true;
            }
        }

        if self.is_shooting {
            self.frame_timer += dt;
            if self.frame_timer > self.frame_speed {
                self.frame += 1;
                self.frame_timer = 0.0;
                if self.frame == 2 {
                    play_sound(&self.shoot_sound, PlaySoundParams { looped: false, volume: 1.0 });
                    self.did_shoot = true;
                    self.score += 10;
                } else {
                    self.did_shoot = false;
                }
                if self.frame >= 4 {
                    self.shots_fired -= 1;
                    if self.shots_fired > 0 && is_key_down(KeyCode::Space) {
                        self.frame = 0;
                    } else {
                        self.state = AnimationState::Idle;
                        self.frame = 0;
                        self.is_shooting = false;
                    }
                }
            }
            return;
        }

        if (is_key_pressed(KeyCode::R) || (is_key_down(KeyCode::Space) && self.shots_fired == 0)) && !self.is_recharging {
            self.is_recharging = true;
            play_sound(&self.recharging_sound, PlaySoundParams { looped: false, volume: 1.5 });
            self.state = AnimationState::Recharging;
            self.frame = 0;
            self.frame_timer = 0.0;
        }
        
        if self.is_recharging {
            self.frame_timer += dt;
            if self.frame_timer > self.frame_speed {
                self.frame += 1;
                self.frame_timer = 0.0;

                if self.frame >= 13 {
                    self.is_recharging = false;
                    self.shots_fired = 12;
                    self.state = AnimationState::Idle;
                    self.frame = 0;
                }
            }
            return;
        }

        let mut moving = false;
        let mut running = false;

        if is_key_down(KeyCode::Left) {
            self.x -= self.speed * dt;
            moving = true;
            self.facing_left = true;
            self.is_recharging = false;
        }
        if is_key_down(KeyCode::Right) {
            self.x += self.speed * dt;
            moving = true;
            self.facing_left = false;
            self.is_recharging = false;
        }
        if is_key_down(KeyCode::Up) {
            self.y -= self.speed * dt;
            moving = true;
            self.is_recharging = false;
        }
        if is_key_down(KeyCode::Down) {
            self.y += self.speed * dt;
            moving = true;
            self.is_recharging = false;
        }
        if is_key_down(KeyCode::LeftShift) {
            self.speed =500.0;
            running = true;
            self.is_recharging = false;
        } else {
            self.speed = 300.0;
        }

        self.state = if running && moving {
            AnimationState::Running
        } else if moving {
            AnimationState::Walking
        } else {
            AnimationState::Idle
        };

        self.x = self.x.clamp(0.0 - self.width * 0.9, screen_width() - self.width * 1.6);
        let ground_level = screen_height() * 0.8;
        let max_y = screen_height() - self.height * 2.5;
        self.y = self.y.clamp(ground_level - self.height * 2.5, max_y);

        if self.is_hit {
            self.hit_timer += dt;
            if self.hit_timer > 0.5 {
                self.is_hit = false;
                self.hit_timer = 0.0;
            }
        }

        self.frame_timer += dt;
        if self.frame_timer > self.frame_speed {
            self.frame = (self.frame + 1) % match self.state {
                AnimationState::Idle => 7,
                AnimationState::Walking => 7,
                AnimationState::Running => 8,
                AnimationState::Shoting => 4,
                AnimationState::Recharging => 13,
                AnimationState::Death => 4,
            };
            self.frame_timer = 0.0;
        }
    }

    pub fn hit(&mut self) {
        if !self.is_hit && !self.is_dead {
            self.is_hit = true;
            self.frame = 0;
            self.frame_timer = 0.0;
            self.health -= 10;

            if self.health <= 0 {
                play_sound(&self.death_sound, PlaySoundParams { looped: false, volume: 1.0 });
                self.die();
            }
        }
    }

    fn die(&mut self) {
        if !self.is_dead {
            self.is_dead = true;
            self.frame = 0;
            self.frame_timer = 0.0;
            self.state = AnimationState::Death;
            self.show_death_menu = false;
        }
    }

    pub fn draw(&self) {
        let (texture, frame_count) = match self.state {
            AnimationState::Idle => (self.idle_texture.clone(), 7),
            AnimationState::Walking => (self.walk_texture.clone(), 7),
            AnimationState::Running => (self.run_texture.clone(), 8),
            AnimationState::Shoting => (self.shot_texture.clone(), 4),
            AnimationState::Recharging => (self.recharge_texture.clone(), 13),
            AnimationState::Death => (self.death_texture.clone(), 4)
        };
        let frame_width = texture.width() / frame_count as f32;
        let src = Rect::new(self.frame as f32 * frame_width, 0.0, frame_width, 128.0);
        let color = if self.is_hit {
            RED
        } else {
            WHITE
        };
        draw_texture_ex(
            &texture,
            self.x,
            self.y,
            color,
            DrawTextureParams {
                source: Some(src),
                dest_size: Some(Vec2::new(2.5*self.width, 2.5*self.height)),
                flip_x: self.facing_left,
                ..Default::default()
            },
        );

        self.draw_ui();
        
        if self.is_dead && self.show_death_menu {
            self.draw_death_menu();
        }
    }

    fn draw_ui(&self) {
        let screen_w = screen_width();
        let padding = 40.0;
        let bar_width = 200.0;
        let bar_height = 30.0;

        // Health bar
        let health_x = padding;
        draw_rectangle(health_x, 20.0, bar_width, bar_height, DARKGRAY);
        draw_rectangle(health_x + 2.0, 22.0, self.health as f32 * (bar_width - 4.0) / 100.0, 26.0, RED);
        let health_text = format!("Health: {:.0}%", self.health);
        let health_text_x = health_x + (bar_width - measure_text(&health_text, None, 20, 1.0).width) / 2.0;
        draw_text(&health_text, health_text_x, 40.0, 20.0, WHITE);

        // Ammo display
        let ammo_box_w = 100.0;
        let ammo_x = (screen_w - ammo_box_w) / 2.0;
        draw_rectangle_lines(ammo_x, 20.0, ammo_box_w, 32.0, 3.0, DARKGRAY);
        draw_texture_ex(
            &self.ammo_texture,
            ammo_x + 5.0,
            20.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(32.0, 32.0)),
                ..Default::default()
            },
        );
        let ammo_text = format!("x{}", self.shots_fired);
        draw_text(&ammo_text, ammo_x + 40.0, 45.0, 32.0, WHITE);

        // Score display
        let score_text = format!("Score: {}", self.score);
        let score_box_w = 200.0;
        let score_x = screen_w - score_box_w - padding;
        draw_rectangle_lines(score_x, 20.0, score_box_w, 32.0, 3.0, DARKGRAY);
        let score_text_x = score_x + 10.0;
        draw_text(&score_text, score_text_x, 47.0, 32.0, WHITE);
    }

    fn draw_death_menu(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        let box_width = 300.0;
        let box_height = 150.0;
        let box_x = (screen_w - box_width) / 2.0;
        let box_y = (screen_h - box_height) / 2.0;

        draw_rectangle(box_x, box_y, box_width, box_height, Color::new(0.0, 0.0, 0.0, 0.7));

        let title = "You Died!";
        let title_size = 32.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        let title_x = (screen_w - title_width) / 2.0;
        let title_y = box_y + 40.0;
        draw_text(title, title_x, title_y, title_size, RED);

        let restart = "Press R to Restart";
        let restart_size = 20.0;
        let restart_width = measure_text(restart, None, restart_size as u16, 1.0).width;
        let restart_x = (screen_w - restart_width) / 2.0;
        let restart_y = title_y + 35.0;
        draw_text(restart, restart_x, restart_y, restart_size, WHITE);

        #[cfg(not(target_arch = "wasm32"))]
        {
            let exit = "Press ESC to Exit";
            let exit_width = measure_text(exit, None, restart_size as u16, 1.0).width;
            let exit_x = (screen_w - exit_width) / 2.0;
            let exit_y = restart_y + 25.0;
            draw_text(exit, exit_x, exit_y, restart_size, WHITE);
        }

    }
}