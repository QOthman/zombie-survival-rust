use macroquad::prelude::*;
use crate::player::Player;
use macroquad::audio::*;

#[derive(PartialEq)]
pub enum EnemyState {
    Alive,
    Dying,
    Dead,
    Attacking,
}

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    width: f32,
    height: f32,
    speed: f32,
    pub frame: usize,
    pub frame_timer: f32,
    frame_speed: f32,
    enemy_texture: Texture2D,
    death_texture: Texture2D,
    enemy_atack_texture: Texture2D,
    facing_left: bool,
    pub state: EnemyState,
    did_attack: bool,
    zombie_attack: Sound,
}

impl Enemy {
    pub fn new(enemy_texture: Texture2D,death_texture: Texture2D,enemy_atack_texture: Texture2D,speed_multiplier: f32, zombie_attack: Sound) -> Self {
        let ground_level = screen_height() * 0.8;
        
        let from_left = rand::gen_range(0.0, 1.0) < 0.5;
        let (x, facing_left) = if from_left {
            (-64.0, false) 
        } else {
            (screen_width() + 64.0, true)
        };
        
        Self {
            x,
            y: rand::gen_range(ground_level / 1.25, screen_height()/1.25 - 64.0),
            width: 64.0,
            height: 64.0,
            speed: rand::gen_range(80.0, 150.0)* speed_multiplier,
            frame: 0,
            frame_timer: 0.0,
            frame_speed: 0.1,
            enemy_texture,
            death_texture,
            facing_left,
            state: EnemyState::Alive,
            enemy_atack_texture,
            did_attack: false,
            zombie_attack,
        }
    }

    pub fn update(&mut self, dt: f32, player_x: f32, player_y: f32, player: &mut Player) {
        match self.state {
            EnemyState::Alive => {
                let dx = player_x - self.x;
                let dy = player_y - self.y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < 25.0 {
                    self.state = EnemyState::Attacking;
                    self.frame = 0;
                    self.frame_timer = 0.0;
                    self.did_attack = false;
                } else {
                    self.x += self.speed * dx / distance * dt;
                    self.y += self.speed * dy / distance * dt;
                }

                self.facing_left = dx < 0.0;

                self.frame_timer += dt;
                if self.frame_timer > self.frame_speed {
                    self.frame = (self.frame + 1) % 10;
                    self.frame_timer = 0.0;
                }
            }
            EnemyState::Attacking => {
                self.frame_timer += dt;
                if self.frame_timer > 0.1 {
                    self.frame += 1;
                    self.frame_timer = 0.0;

                    if self.frame == 2 && !self.did_attack {
                        play_sound(&self.zombie_attack, PlaySoundParams { looped: false, volume: 1.0 });
                        player.hit(); 
                        self.did_attack = true;
                    }

                    if self.frame >= 5 {
                        self.state = EnemyState::Alive;
                        self.frame = 0;
                        self.did_attack = false;
                    }
                }
            }
            EnemyState::Dying => {
                self.frame_timer += dt;
                if self.frame_timer > self.frame_speed {
                    self.frame += 1;
                    self.frame_timer = 0.0;
                    if self.frame >= 9 {
                        self.state = EnemyState::Dead;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn draw(&self) {
        let (texture, frame, total_frames) = match self.state {
            EnemyState::Alive => (self.enemy_texture.clone(), self.frame, 10),
            EnemyState::Dying => (self.death_texture.clone(), self.frame, 9),
            EnemyState::Attacking => (self.enemy_atack_texture.clone(), self.frame, 5),
            EnemyState::Dead => return,
        };

        let frame_width = texture.width() / total_frames as f32;
        let src = Rect::new(frame as f32 * frame_width, 0.0, frame_width, texture.height());

        draw_texture_ex(
            &texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                source: Some(src),
                dest_size: Some(Vec2::new(self.width * 2.5, self.height * 2.5)),
                flip_x: self.facing_left,
                ..Default::default()
            },
        );
    }

    pub fn is_off_screen(&self) -> bool {
        self.x < -100.0 || self.x > screen_width() + 100.0
    }
}

// ---------------------------
// EnemyManager
// ---------------------------
pub struct EnemyManager {
    enemies: Vec<Enemy>,
    spawn_timer: f32,
    spawn_cooldown: f32,
    enemy_texture: Texture2D,
    death_texture: Texture2D,
    enemy_atack_texture: Texture2D,
    difficulty_timer: f32,
    difficulty_level: u32,
    zombie_attack: Sound,
}


impl EnemyManager {
    pub fn new(enemy_texture: Texture2D, death_texture: Texture2D,enemy_atack_texture: Texture2D,zombie_attack: Sound) -> Self {
        Self {
            enemies: vec![],
            spawn_timer: 0.0,
            spawn_cooldown: 3.0,
            enemy_texture,
            death_texture,
            enemy_atack_texture,
            difficulty_timer: 0.0,
            difficulty_level:1,
            zombie_attack,
        }
    }

    pub fn update(&mut self, dt: f32, player: &mut Player) {
        self.spawn_timer += dt;

        self.difficulty_timer += dt;
        if self.difficulty_timer >= 15.0 {
            self.difficulty_level += 1;
            self.difficulty_timer = 0.0;
            let min_cooldown = 0.5;
            self.spawn_cooldown = (self.spawn_cooldown * 0.9).max(min_cooldown);
        }

        if self.spawn_timer > self.spawn_cooldown {
            self.enemies.push(Enemy::new(self.enemy_texture.clone(), self.death_texture.clone(), self.enemy_atack_texture.clone(),1.0 + (self.difficulty_level as f32 * 0.1), self.zombie_attack.clone()));
            self.spawn_timer = 0.0;
        }

        for enemy in self.enemies.iter_mut() {
            enemy.update(dt, player.x, player.y,player);
        }
        if player.did_shoot {
            for enemy in self.enemies.iter_mut() {
                if enemy.state == EnemyState::Alive || enemy.state == EnemyState::Attacking{
                    let in_range_x = player.x - enemy.x <= 400.0 && player.x - enemy.x >= -400.0;
                    let in_range_y = (player.y - enemy.y).abs() < 40.0;
                    let facing_correct = player.facing_left == (enemy.x < player.x);

                    if in_range_x && in_range_y && facing_correct {
                        enemy.state = EnemyState::Dying;
                        enemy.frame = 0;
                        enemy.frame_timer = 0.0;
                        break;
                    }
                }
            }

            player.did_shoot = false;
        }
        self.enemies.retain(|e| e.state != EnemyState::Dead && !e.is_off_screen());
    }

    pub fn reset(&mut self) {
        self.enemies.clear();
        self.difficulty_level = 1;
        self.difficulty_timer = 0.0;
        self.spawn_timer = 0.0;
        self.spawn_cooldown = 3.0;
    }

    pub fn draw(&self) {
        for enemy in &self.enemies {
            enemy.draw();
        }
    }
}