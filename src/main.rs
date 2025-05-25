use macroquad::prelude::*;
use macroquad::audio::*;

mod weather;
mod player;
mod enemy;

use weather::{Raindrop, Splash, LightningManager};
use player::Player;
use enemy::EnemyManager;

#[macroquad::main("Rainstorm with Lightning")]
async fn main() {
    let idle_texture = load_texture("./assets/player/Idle.png").await.unwrap();
    let walk_texture = load_texture("./assets/player/Walk.png").await.unwrap();
    let run_texture = load_texture("./assets/player/Run.png").await.unwrap();
    let shot_texture = load_texture("./assets/player/Shot.png").await.unwrap();
    let recharge_texture = load_texture("./assets/player/Recharge.png").await.unwrap();
    let death_texture = load_texture("./assets/player/Dead.png").await.unwrap();
    let enemy_texture = load_texture("./assets/enemy/Walk.png").await.unwrap();
    let enemy_death_texture = load_texture("./assets/enemy/Death.png").await.unwrap();
    let enemy_attack_texture = load_texture("./assets/enemy/Attack.png").await.unwrap();
    let ammo_texture = load_texture("./assets/game/ammo.png").await.unwrap();

    let rain_sound = load_sound("./assets/sound/rain.wav").await.unwrap();
    let thunder_sound = load_sound("./assets/sound/thunder.wav").await.unwrap();
    let shoot_sound = load_sound("./assets/sound/shoot.wav").await.unwrap();
    let death_sound = load_sound("./assets/sound/player_death.wav").await.unwrap();
    let zombie_attack_sound = load_sound("./assets/sound/zombie_attack.wav").await.unwrap();
    let recharging_sound = load_sound("./assets/sound/recharging.wav").await.unwrap();

    play_sound(&rain_sound, PlaySoundParams { looped: true, volume: 0.5 });


    let mut raindrops: Vec<Raindrop> = (0..300).map(|_| Raindrop::new()).collect();
    let mut splashes: Vec<Splash> = vec![];
    let mut lightning = LightningManager::new(thunder_sound);
    let mut player = Player::new(idle_texture.clone(), walk_texture.clone(),run_texture.clone(),shot_texture.clone(),recharge_texture.clone(), death_texture.clone(),ammo_texture.clone(),shoot_sound.clone(), death_sound.clone(), recharging_sound.clone());
    let mut enemy_manager = EnemyManager::new(enemy_texture, enemy_death_texture, enemy_attack_texture, zombie_attack_sound.clone());  
    
    loop {
        let dt = get_frame_time();
        lightning.update(dt);

        let brightness = if lightning.is_flashing() { 1.0 } else { 0.0 };
        let bg_color = Color::new(brightness * 0.2, brightness * 0.2, brightness * 0.3, 1.0);
        clear_background(bg_color);

        for drop in raindrops.iter_mut() {
            drop.update(dt);
            drop.draw(brightness);
            if drop.is_near_ground() {
                splashes.push(Splash::new(drop.x, drop.y + drop.length));
                *drop = Raindrop::new();
            }
        }

        for splash in splashes.iter_mut() {
            splash.update(dt);
            splash.draw(brightness);
        }

        splashes.retain(|s| !s.is_dead());

        lightning.draw();

        player.update(dt);  
        player.draw();

        if !player.is_dead {
            enemy_manager.update(dt, &mut player);
        }
        enemy_manager.draw();
        
        if player.is_dead {
            if is_key_pressed(KeyCode::R) {
                player = Player::new(idle_texture.clone(), walk_texture.clone(), run_texture.clone(), shot_texture.clone(), recharge_texture.clone(), death_texture.clone(),ammo_texture.clone(),shoot_sound.clone(), death_sound.clone(), recharging_sound.clone());
                enemy_manager.reset();
            }else if is_key_pressed(KeyCode::Escape) {
                break;
            }

        }
        
        next_frame().await;
    }
}