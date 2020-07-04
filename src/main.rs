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
    CJ McAllister   02 Sep 2018     Add basic ggez graphics
\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate rand;

extern crate cast_iron;
use cast_iron::{
    actor::Actor,
    ability::{
        Ability,
        aspect::*
    },
    environment::{
        Element,
        weather::Weather
    },
    polyfunc::PolyFunc
};

extern crate ggez;
use ggez::{
    Context as GgEzContext,
    ContextBuilder as GgEzContextBuilder,
    GameResult,
    conf as ggez_conf,
    event as ggez_event,
    graphics as ggez_gfx,
    mint as ggez_mint,
    timer as ggez_timer
};

///
// Module Declarations
/// 
pub mod game_assets;
use game_assets::colors::*;

pub mod weather_manager;
use weather_manager::WeatherManager;

pub mod world_grid_manager;
use world_grid_manager::*;


///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

const DEFAULT_WINDOW_SIZE_X: f32 = 1500.0;
const DEFAULT_WINDOW_SIZE_Y: f32 = 1000.0;
const DESIRED_FPS: u32 = 60;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

/// Primary Game Struct
struct SandCastingGameState {
    world_grid_manager: WorldGridManager,
    weather_manager:    WeatherManager
}

impl SandCastingGameState {
    pub fn new(_ctx: &mut GgEzContext) -> SandCastingGameState {
        // Load/create resources here: images, fonts, sounds, etc.
        SandCastingGameState{
            world_grid_manager: WorldGridManager::new(10),
            weather_manager:    WeatherManager::new()
        }
    }
}

impl ggez_event::EventHandler for SandCastingGameState {
    fn update(&mut self, ctx: &mut GgEzContext) -> GameResult<()> {
        while ggez_timer::check_update_time(ctx, DESIRED_FPS) {
            // Update weather
            self.weather_manager.update_weather(&ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut GgEzContext) -> GameResult<()> {
        ggez_gfx::clear(ctx, BLACK);
        
        // Get current window dimensions
        let (window_x, window_y) = ggez_gfx::size(ctx);
        let center = ggez_mint::Point2 {x: window_x / 2.0, y: window_y / 2.0};

        // Create a mesh builder for the base hex grid
        let mut base_grid_mesh_builder = ggez_gfx::MeshBuilder::new();
        self.world_grid_manager.draw_grid(center, &mut base_grid_mesh_builder);

        // Build the base hex grid mesh and draw it
        let base_grid_mesh = base_grid_mesh_builder.build(ctx)?;
        ggez_gfx::draw(ctx, &base_grid_mesh, ggez_gfx::DrawParam::default())?;

        ggez_gfx::present(ctx)
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
    let (mut ggez_context, mut ggez_event_loop) = GgEzContextBuilder::new("sand_casting", "CJ McAllister")
                                                  .window_setup(
                                                      ggez_conf::WindowSetup::default()
                                                      .title("Sand Casting - A Cast Iron Sandbox Game")
                                                      .vsync(true)
                                                    )
                                                  .window_mode(
                                                      ggez_conf::WindowMode::default()
                                                      .dimensions(DEFAULT_WINDOW_SIZE_X, DEFAULT_WINDOW_SIZE_Y)
                                                    )
                                                  .build()
                                                  .unwrap();

    // Use built context to create a GGEZ Event Handler instance
    let mut sand_casting_game_state = SandCastingGameState::new(&mut ggez_context);

    // Run the game!
    match ggez_event::run(&mut ggez_context, &mut ggez_event_loop, &mut sand_casting_game_state) {
        Ok(_)   => println!("Exited cleanly."),
        Err(e)  => println!("Error occured: {}", e)
    }
}
