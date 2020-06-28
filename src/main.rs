/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : main.rs

Copyright (C) 2017 CJ McAllister
    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 3 of the License, or
    (at your option) any later version.
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software Foundation,
    Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301  USA

Purpose:
    Main loop for the CastIron sample game.

Changelog:
    CJ McAllister   22 Nov 2017     File created
    CJ McAllister   18 Jan 2018     Add main loop, weather
    CJ Mcallister   02 Sep 2018     Add basic ggez graphics
\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate ggez;

extern crate cast_iron;
use cast_iron::actor::Actor;
use cast_iron::ability::Ability;
use cast_iron::ability::aspect::*;
use cast_iron::environment::Element;
use cast_iron::environment::weather::Weather;
use cast_iron::polyfunc::PolyFunc;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::conf;

use std::f64;
use std::thread;
use std::time::{Duration, Instant};

pub mod shape;
use shape::point::Point;

pub mod world_grid_manager;
use world_grid_manager::*;


///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
const BLACK:    [f64; 4] = [0.0, 0.0, 0.0, 1.0];
#[allow(dead_code)]
const WHITE:    [f64; 4] = [1.0, 1.0, 1.0, 1.0];
#[allow(dead_code)]
const RED:      [f64; 4] = [1.0, 0.0, 0.0, 1.0];
#[allow(dead_code)]
const GREEN:    [f64; 4] = [0.0, 1.0, 0.0, 1.0];
#[allow(dead_code)]
const BLUE:     [f64; 4] = [0.0, 0.0, 1.0, 1.0];
#[allow(dead_code)]
const YELLOW:   [f64; 4] = [1.0, 1.0, 0.0, 1.0];
#[allow(dead_code)]
const CYAN:     [f64; 4] = [0.0, 1.0, 1.0, 1.0];
#[allow(dead_code)]
const PURPLE:   [f64; 4] = [1.0, 0.0, 1.0, 1.0];
#[allow(dead_code)]
const GREY:     [f64; 4] = [0.5, 0.5, 0.5, 1.0];

const DEFAULT_WINDOW_SIZE_X: f32 = 800.0;
const DEFAULT_WINDOW_SIZE_Y: f32 = 600.0;



///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

/// Primary Game Struct
struct SandCastingGame {
    world_grid_manager: WorldGridManager
}

impl SandCastingGame {
    pub fn new(_ctx: &mut Context) -> SandCastingGame {
        // Load/create resources here: images, fonts, sounds, etc.
        SandCastingGame{
            world_grid_manager: WorldGridManager::new(10)
        }
    }
}

impl EventHandler for SandCastingGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        //TODO: Draw code here...

        graphics::present(ctx)
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

fn main() {
    // Initialize Abilities
    let null_abil: Ability = Ability::new("Null");

    let mut lightning_bolt: Ability = Ability::new("Lightning Bolt");
    lightning_bolt.set_potency(20);
    lightning_bolt.set_aesthetics(Aesthetics::Impressive);
    lightning_bolt.set_element(Element::Electric);
    lightning_bolt.set_method(Method::Wand);
    lightning_bolt.set_morality(Morality::Neutral);
    lightning_bolt.set_school(School::Destruction);

    let mut blood_drain: Ability = Ability::new("Blood Drain");
    blood_drain.set_potency(50);
    blood_drain.set_aesthetics(Aesthetics::Ugly);
    blood_drain.set_element(Element::Dark);
    blood_drain.set_method(Method::Manual);
    blood_drain.set_morality(Morality::Evil);
    blood_drain.set_school(School::Destruction);

    // Intialize Actor
    let mut player_one: Actor = Actor::new("CJ McAllister");
    player_one.add_ability(lightning_bolt);
    player_one.add_ability(blood_drain);
    player_one.add_ability(null_abil);

    // Intialize Weather
    let thunder_func: PolyFunc = PolyFunc::from(150, 10, 15);
    let thunderstorm: Weather = Weather::from(Element::Electric, thunder_func);

    // Create a GGEZ Context and EventLoop
    let (mut ggez_context, mut ggez_event_loop) = ContextBuilder::new("sand_casting", "CJ McAllister")
                                                  .window_setup(conf::WindowSetup::default().title("Sand Casting - A Cast Iron Sandbox Game"))
                                                  .window_mode(conf::WindowMode::default().dimensions(DEFAULT_WINDOW_SIZE_X, DEFAULT_WINDOW_SIZE_Y))
                                                  .build()
                                                  .unwrap();

    // Use built context to create a GGEZ Event Handler instance
    let mut sand_casting_game_state = SandCastingGame::new(&mut ggez_context);
    
    // Kick off the timing thread
    thread::spawn(move || {
        const MAX_TICKS: u32 = 31;
        let mut tick: u32 = 0;
        
        while tick <= MAX_TICKS {
            println!("Tick {}: Weather: {:?}", tick, thunderstorm.intensity(tick));

            tick = tick + 1;
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Run the game!
    match event::run(&mut ggez_context, &mut ggez_event_loop, &mut sand_casting_game_state) {
        Ok(_)   => println!("Exited cleanly."),
        Err(e)  => println!("Error occured: {}", e)
    }
}
