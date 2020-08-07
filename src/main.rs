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

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

extern crate rand;

extern crate cast_iron;
use cast_iron::{
    ability::{
        Ability,
        aspect::*
    },
    actor::Actor,
    context::Context as CastIronContext,
    environment::element::Element,
    debug_println, function_name
};
// use cast_iron::environment::resource;
// use cast_iron::environment::coords::Coords;

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

pub mod obstacle_manager;
use obstacle_manager::ObstacleManager;

pub mod resource_manager;
use resource_manager::ResourceManager;

pub mod weather_manager;
use weather_manager::WeatherManager;

pub mod world_grid_manager;
use world_grid_manager::*;


///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

/* Appearence */
const DEFAULT_WINDOW_SIZE_X: f32 = 1000.0;
const DEFAULT_WINDOW_SIZE_Y: f32 = 1000.0;
const DESIRED_FPS: u32 = 60;

const DEFAULT_LINE_WIDTH: f32 = 2.0;
const DEFAULT_LINE_COLOR: ggez_gfx::Color = WHITE;
const DEFAULT_FILL_COLOR: ggez_gfx::Color = GREY;

/* Hex Grid */
const GRID_CELL_SIZE: f32 = 25.0;
// CENTER_TO_SIDE_DIST = GRID_CELL_SIZE * sin(pi/3)
// Distance from centerpoint of hex to center of a side
static CENTER_TO_SIDE_DIST: f32 = GRID_CELL_SIZE * 0.86602540378;
// CENTER_TO_VERTEX_DIST = GRID_CELL_SIZE * cos(pi/3)
// Distance from centerpoint of hex to center of a side
static CENTER_TO_VERTEX_DIST: f32 = GRID_CELL_SIZE * 0.5;

/* Mechanics */
const DEFAULT_HEX_GRID_MAX_RADIAL_DISTANCE: usize = 10;
// const DEFAULT_MAX_OBSTACLE_LENGTH: u8 = 5;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

//OPT: *STYLE* This should move to its own file
/// Primary Game Struct
struct SandCastingGameState {
    avg_fps:            f64,                // Average FPS for display
    peak_fps:           f64,                // Peak FPS for display
    obstacle_manager:   ObstacleManager,    // Obstacle Manager instance
    resource_manager:   ResourceManager,    // Resource Manager instance
    weather_manager:    WeatherManager,     // Weather Manager instance
    world_grid_manager: WorldGridManager,   // World Grid Manager instance
}

//OPT: *STYLE* Probably needs a builder
impl SandCastingGameState {
    pub fn new(ctx: &mut GgEzContext) -> Self {
        // Load/create resources here: images, fonts, sounds, etc.
        SandCastingGameState{
            avg_fps:            0.0,
            peak_fps:           0.0,
            obstacle_manager:   ObstacleManager::new(ctx),
            resource_manager:   ResourceManager::new(ctx),
            weather_manager:    WeatherManager::default(),
            world_grid_manager: WorldGridManager::new(DEFAULT_HEX_GRID_MAX_RADIAL_DISTANCE, ctx),
        }
    }
}

impl ggez_event::EventHandler for SandCastingGameState {
    fn update(&mut self, ctx: &mut GgEzContext) -> GameResult<()> {
        while ggez_timer::check_update_time(ctx, DESIRED_FPS) {
            debug_println!("update(): Updating weather...");
            // Update weather
            self.weather_manager.update_weather(ctx);
            debug_println!("update(): Weather updated.");

            // Update the resource mesh
            debug_println!("update(): Updating resource mesh...");
            self.resource_manager.update_resource_mesh(ctx);
            debug_println!("update(): Resource mesh updated.");

            // Update the obstacle mesh
            debug_println!("update(): Updating obstacle mesh...");
            self.obstacle_manager.update_obstacle_mesh(ctx);
            debug_println!("update(): Obstacle mesh updated.");

            // Update FPS
            self.avg_fps = ggez_timer::fps(ctx);
            if self.avg_fps > self.peak_fps {
                self.peak_fps = self.avg_fps;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut GgEzContext) -> GameResult<()> {
        ggez_gfx::clear(ctx, BLACK);

        // Draw the hex grid
        ggez_gfx::draw(ctx, self.world_grid_manager.get_base_grid_mesh(), ggez_gfx::DrawParam::default())?;

        // Draw resources
        ggez_gfx::draw(ctx, self.resource_manager.get_resource_mesh(), ggez_gfx::DrawParam::default())?;

        // Draw obstacles
        ggez_gfx::draw(ctx, self.obstacle_manager.get_obstacle_mesh(), ggez_gfx::DrawParam::default())?;

        // Draw the FPS counters
        let avg_fps_pos = ggez_mint::Point2 {x: 0.0, y: 0.0};
        let avg_fps_str = format!("Avg. FPS: {:.0}", self.avg_fps);
        let avg_fps_display = ggez_gfx::Text::new((avg_fps_str, ggez_gfx::Font::default(), 16.0));
        ggez_gfx::draw(ctx, &avg_fps_display, (avg_fps_pos, 0.0, GREEN)).unwrap();

        let peak_fps_pos = ggez_mint::Point2 {x: 0.0, y: 20.0};
        let peak_fps_str = format!("Peak FPS: {:.0}", self.peak_fps);
        let peak_fps_display = ggez_gfx::Text::new((peak_fps_str, ggez_gfx::Font::default(), 16.0));
        ggez_gfx::draw(ctx, &peak_fps_display, (peak_fps_pos, 0.0, GREEN)).unwrap();

        ggez_gfx::present(ctx)
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

fn main() {
    // Initialize Abilities
    let null_abil: Ability = Ability::new_name_only("Null");

    let mut lightning_bolt: Ability = Ability::new_name_only("Lightning Bolt");
    lightning_bolt.set_potency(20);
    lightning_bolt.set_aesthetics(Aesthetics::Impressive);
    lightning_bolt.set_element(Element::Electric);
    lightning_bolt.set_method(Method::Wand);
    lightning_bolt.set_morality(Morality::Neutral);
    lightning_bolt.set_school(School::Destruction);

    let mut blood_drain: Ability = Ability::new_name_only("Blood Drain");
    blood_drain.set_potency(50);
    blood_drain.set_aesthetics(Aesthetics::Ugly);
    blood_drain.set_element(Element::Dark);
    blood_drain.set_method(Method::Manual);
    blood_drain.set_morality(Morality::Evil);
    blood_drain.set_school(School::Destruction);

    // Intialize Actor
    let mut player_one: Actor = Actor::new_name_only("CJ McAllister");
    player_one.add_ability(lightning_bolt);
    player_one.add_ability(blood_drain);
    player_one.add_ability(null_abil);

    // Create CastIron Context
    let mut ci_ctx = CastIronContext::default();

    // Create a GGEZ Context and EventLoop
    let (mut ggez_context, mut ggez_event_loop) = GgEzContextBuilder::new("sand_casting", "CJ McAllister")
                                                  .window_setup(
                                                      ggez_conf::WindowSetup::default()
                                                      .title("Sand Casting - A CastIron Sandbox Game")
                                                      .vsync(false)
                                                    )
                                                  .window_mode(
                                                      ggez_conf::WindowMode::default()
                                                      .dimensions(DEFAULT_WINDOW_SIZE_X, DEFAULT_WINDOW_SIZE_Y)
                                                    )
                                                  .build()
                                                  .unwrap();

    // Use built context to create a GGEZ Event Handler instance
    let mut sand_casting_game_state = SandCastingGameState::new(&mut ggez_context);

    // Create random resources
    for _i in 0..3 {
        sand_casting_game_state.resource_manager.add_rand_resource(&mut ci_ctx, &mut ggez_context).unwrap();
    }

    // Create random obstacles
    for _i in 0..1 {
        sand_casting_game_state.obstacle_manager.add_rand_obstacle(&mut ci_ctx, &mut ggez_context).unwrap();
    }

    // Run the game!
    match ggez_event::run(&mut ggez_context, &mut ggez_event_loop, &mut sand_casting_game_state) {
        Ok(_)   => println!("Exited cleanly."),
        Err(e)  => println!("Error occured: {}", e)
    }
}
