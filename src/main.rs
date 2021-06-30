extern crate sdl2;

// Modules
mod battle;
mod overworld;
mod player;

use player::Player;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
//use sdl2::render::Texture;
use std::collections::HashSet;
use std::path::Path;

//use std::time::Duration;
//use std::thread;

const TITLE: &str = "Monster Town Week 3";
const TILE_SIZE: u32 = 16;

const VSYNC: bool = true;
// Camera
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

const MAX_SPEED: i32 = 5;

const BUFFER_FRAMES: u32 = 0;

// Im not sure what these are used for
const SCALE_UP: i16 = 3;
const HELP_WHERE_DOES_THIS_COME_FROM: i32 = 1250;

enum Map {
  Overworld,
  Battle,
}

fn check_collision(a: &Rect, b: &Rect) -> bool {
  if a.bottom() < b.top() || a.top() > b.bottom() || a.right() < b.left() || a.left() > b.right() {
    false
  } else {
    true
  }
}

pub fn init(
  title: &str,
  vsync: bool,
  width: u32,
  height: u32,
) -> Result<(sdl2::render::WindowCanvas, sdl2::EventPump), String> {
  let sdl_cxt = sdl2::init()?;
  let video_subsys = sdl_cxt.video()?;

  let window = video_subsys
    .window(title, width, height)
    .build()
    .map_err(|e| e.to_string())?;

  let wincan = window.into_canvas().accelerated();

  // Check if we should lock to vsync
  let wincan = if vsync {
    wincan.present_vsync()
  } else {
    wincan
  };

  let wincan = wincan.build().map_err(|e| e.to_string())?;

  let event_pump = sdl_cxt.event_pump()?;

  let _cam = Rect::new(0, 0, CAM_W, CAM_H);

  Ok((wincan, event_pump))
}

fn run(
  wincan: &mut sdl2::render::WindowCanvas,
  event_pump: &mut sdl2::EventPump,
) -> Result<(), String> {
  wincan.set_blend_mode(BlendMode::Blend);

  let mut loaded_map = Map::Battle;

  let texture_creator = wincan.texture_creator();

  let gym = texture_creator.load_texture("images/GymV6.png")?;
  let second_gym = texture_creator.load_texture("images/GymV7.png")?;

  let battle_bg = texture_creator.load_texture("images/battle_bg.png")?;

  let player_monster = "deer pokemon";
  let enemy_monster = "Chromacat";

  let pi = format!("images/{}.png", player_monster);
  let ei = format!("images/{}.png", enemy_monster);

  let player_texture = texture_creator.load_texture(pi)?;
  let enemy_texture = texture_creator.load_texture(ei)?;
  let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
  let font_path = Path::new(r"./fonts/framd.ttf");
  let font = ttf_context.load_font(font_path, 256)?;

  let player_moves = vec!["a", "b", "c", "d"]
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>();

  let player_e = vec!["w", "x", "y", "z"]
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>();

  let (attacks, effects) = battle::create_attack_tuples(&texture_creator, &font, &player_moves, &player_e)?;

  let player_mon = &player_monster.to_string();
  let enemy_mon = &enemy_monster.to_string();

  let player_health: f32 = 100.0;
  let enemy_health: f32 = 100.0;

  let (player_name_tup, enemy_name_tup) = battle::create_name_tuples(&texture_creator, &font, &player_mon, &enemy_mon)?;



  let mut battle_init = battle::Battle {
    player_name: &player_name_tup,
    enemy_name: &enemy_name_tup,
    background_texture: &battle_bg,
    player_texture: &player_texture,
    enemy_texture: &enemy_texture,
    font: &font,
    player_attacks: &attacks,
    player_attack_effects: &effects,
    player_health: player_health,
    enemy_health: enemy_health,
  };

  let mut current_choice: i32 = 0;
  let mut selection_buffer = 0;

  // Player Creation from mod player.rs
  let mut p = Player::create(
    Rect::new(
      0,
      0,
      TILE_SIZE * SCALE_UP as u32,
      TILE_SIZE * SCALE_UP as u32,
    ),
    texture_creator.load_texture("images/Character.png")?,
  );

  'gameloop: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'gameloop,
        _ => {}
      }
    }

    // Implement Keystate
    let keystate: HashSet<Keycode> = event_pump
      .keyboard_state()
      .pressed_scancodes()
      .filter_map(Keycode::from_scancode)
      .collect();

    match loaded_map {
      Map::Overworld => {
        overworld::draw_overworld(wincan)?;

        // Create the Town Gym
        let gym_box = Rect::new(340, 90, 150, 150);
        wincan.copy(&gym, None, gym_box)?;
        // Create Second Town Gym
        let second_gym_box = Rect::new(1110, 450, 150, 150);
        wincan.copy(&second_gym, None, second_gym_box)?;

        let movement_direction;
        let speed_update;

        if keystate.contains(&Keycode::W) || keystate.contains(&Keycode::Up) {
          movement_direction = 1;
        } else if keystate.contains(&Keycode::A) || keystate.contains(&Keycode::Left) {
          movement_direction = 2;
        } else if keystate.contains(&Keycode::S) || keystate.contains(&Keycode::Down) {
          movement_direction = 3;
        } else if keystate.contains(&Keycode::D) || keystate.contains(&Keycode::Right) {
          movement_direction = 4;
        } else {
          movement_direction = 0;
        }

        let x_limits = (0, HELP_WHERE_DOES_THIS_COME_FROM as i32);

        match movement_direction {
          1 => {
            speed_update = (0, -MAX_SPEED as i32);
          }
          2 => {
            speed_update = (-MAX_SPEED as i32, 0);
          }
          3 => {
            speed_update = (0, MAX_SPEED as i32);
          }
          4 => {
            speed_update = (MAX_SPEED as i32, 0);
          }
          _ => {
            speed_update = (0, 0);
          }
        }
        p.update_pos(speed_update, x_limits, (0, 700));

        // Collision Check With First Gym
        let player_box = Rect::new(p.x(), p.y(), p.height(), p.width());
        if check_collision(&player_box, &gym_box) {
          p.set_x(p.x() - speed_update.0);
        }
        if check_collision(&player_box, &gym_box) {
          p.set_y(p.y() - speed_update.1);
        }

        // Collision Check with Second Gym
        if check_collision(&player_box, &second_gym_box) {
          p.set_x(p.x() - speed_update.0);
        }
        if check_collision(&player_box, &second_gym_box) {
          p.set_y(p.y() - speed_update.1);
        }

        let battle_box = Rect::new(835, 565, 32, 32);
        if check_collision(&player_box, &battle_box) {
          loaded_map = Map::Battle;

          //battle::Battle { player_name, enemy_name, player_texture, enemy_texture, font, } =
          /*
          battle_init = battle::Battle {
            player_name: player_monster,
            enemy_name: enemy_monster,
            background_texture: &battle_bg,
            player_texture: &player_texture,
            enemy_texture: &enemy_texture,
            font: &font,
            player_moves: &player_moves,
            test: &test,
            player_attacks: &attacks,
          };
          */

          p.set_x(p.x() - speed_update.0);
          p.set_y(p.y() - speed_update.1);
        }

        // Convert player's map position to be camera-relative
        let player_cam_pos = Rect::new(
          p.x(),
          p.y(),
          TILE_SIZE * SCALE_UP as u32,
          TILE_SIZE * SCALE_UP as u32,
        );

        wincan.copy(p.texture(), None, player_cam_pos)?;

        wincan.present();
      }
      Map::Battle => {
        
        battle::better_draw_battle(wincan, &battle_init, current_choice as usize, None)?;

        if keystate.contains(&Keycode::A) || keystate.contains(&Keycode::Left) {
          if selection_buffer > 0 {
            continue;
          } else {
            current_choice -= 1;
            current_choice = if current_choice > 3 {
              0
            } else if current_choice < 0 {
              3
            } else {
              current_choice
            };

            battle::better_draw_battle(wincan, &battle_init, current_choice as usize, None)?;
            selection_buffer = BUFFER_FRAMES;
            wincan.present();
          }
        }
        if keystate.contains(&Keycode::D) || keystate.contains(&Keycode::Right) {
          if selection_buffer > 0 {
            continue;
          } else {
            current_choice += 1;
            current_choice = if current_choice > 3 {
              0
            } else if current_choice < 0 {
              3
            } else {
              current_choice
            };
            selection_buffer = BUFFER_FRAMES;
            battle::better_draw_battle(wincan, &battle_init, current_choice as usize, None)?;
          }
        }
        if keystate.contains(&Keycode::Return) {
          let f = format!("You selected move #{}!", current_choice + 1);
          battle::better_draw_battle(wincan, &battle_init, current_choice as usize, Some(f))?;
        }
        if keystate.contains(&Keycode::E) {
          let screen = Rect::new(0, 0, CAM_W, CAM_H);
          wincan.set_draw_color(Color::RGBA(0, 0, 0, 15));
          for _i in 0..50 {
            wincan.fill_rect(screen)?;
            wincan.present();
          }
          loaded_map = Map::Overworld;
        }
        if keystate.contains(&Keycode::K) {
          for i in 0..101 {
            let k: i32 = ((i - 100) as i32).abs();
            battle_init.set_enemy_health(k as f32);
            battle_init.set_player_health(100.0);
            battle::better_draw_battle(wincan, &battle_init, current_choice as usize, None)?;
          }
        }
        if selection_buffer > 0 {
          selection_buffer -= 1;
        }
      }
    }
  }

  Ok(())
}
fn main() {
  println!("\nRunning {}:", TITLE);
  print!("\tInitting...");
  match init(TITLE, VSYNC, CAM_W, CAM_H) {
    Err(e) => println!("\n\t\tFailed to init: {}", e),
    Ok(d) => {
      println!("DONE");

      let (mut wincan, mut event_pump) = d;

      print!("\tRunning...");
      match run(&mut wincan, &mut event_pump) {
        Err(e) => println!("\n\t\tEncountered error while running: {}", e),
        Ok(_) => println!("DONE\nExiting cleanly"),
      };
    }
  };
}
