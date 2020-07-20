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
    CJ McAllister   03 Jul 2020     Completed removal of piston-2d
\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

extern crate rand;

extern crate cast_iron;
use cast_iron::{
    actor::Actor,
    ability::{
        Ability,
        aspect::*
    },
    environment::{
        element::Element,
        coords::Coords,
        resource
    }
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
// Y_OFFSET = GRID_CELL_SIZE * sin(pi/3) * 2
// Distance from centerpoint of hex to center of a side 
static Y_OFFSET: f32 = GRID_CELL_SIZE * 0.86602540378;
// X_OFFSET = GRID_CELL_SIZE * cos(pi/3) * 2
// Distance from centerpoint of hex to center of a side 
static X_OFFSET: f32 = GRID_CELL_SIZE * 0.5;

/* Mechanics */
const DEFAULT_HEX_GRID_MAX_RADIAL_DISTANCE: u8 = 10;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

/// Primary Game Struct
struct SandCastingGameState {
    avg_fps: f64,                           // Average FPS for display
    resource_manager: ResourceManager,      // Resource Manager instance
    weather_manager: WeatherManager,        // Weather Manager instance
    world_grid_manager: WorldGridManager,   // World Grid Manager instance
}

impl SandCastingGameState {
    pub fn new(ctx: &mut GgEzContext) -> Self {
        // Load/create resources here: images, fonts, sounds, etc.
        SandCastingGameState{
            avg_fps: 0.0,
            resource_manager: ResourceManager::new(ctx),
            weather_manager: WeatherManager::new(),
            world_grid_manager: WorldGridManager::new(DEFAULT_HEX_GRID_MAX_RADIAL_DISTANCE, ctx),
        }
    }
}

impl ggez_event::EventHandler for SandCastingGameState {
    fn update(&mut self, ctx: &mut GgEzContext) -> GameResult<()> {
        while ggez_timer::check_update_time(ctx, DESIRED_FPS) {
            // Update weather
            self.weather_manager.update_weather(ctx);

            // Update the resource mesh
            self.resource_manager.update_resource_mesh(ctx);

            // Update average FPS
            self.avg_fps = ggez_timer::fps(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut GgEzContext) -> GameResult<()> {
        ggez_gfx::clear(ctx, BLACK);
        
        // Draw the hex grid
        ggez_gfx::draw(ctx, self.world_grid_manager.get_base_grid_mesh(), ggez_gfx::DrawParam::default())?;

        // Draw the resource grid
        ggez_gfx::draw(ctx, self.resource_manager.get_resource_mesh(), ggez_gfx::DrawParam::default())?;

        // Draw the FPS counter
        let fps_pos = ggez_mint::Point2 {x: 0.0, y: 0.0};
        let fps_str = format!("FPS: {:.0}", self.avg_fps);
        let fps_display = ggez_gfx::Text::new((fps_str, ggez_gfx::Font::default(), 16.0));
        ggez_gfx::draw(ctx, &fps_display, (fps_pos, 0.0, GREEN)).unwrap();

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

    //FIXME: TEST CODE, DELETE
    // Add resources to the grid
    let pond = resource::Resource::new(Element::Ice, resource::State::Low, Coords::new(1, -3, 2).unwrap(), 1);
    let campfire = resource::Resource::new(Element::Fire, resource::State::Overflow, Coords::new(5, 5, -10).unwrap(), 3);
    let powerline = resource::Resource::new(Element::Water, resource::State::Overflow, Coords::new(0, 4, -4).unwrap(), 2);
    sand_casting_game_state.resource_manager.add_resource(pond, &mut ggez_context).unwrap();
    sand_casting_game_state.resource_manager.add_resource(campfire, &mut ggez_context).unwrap();
    sand_casting_game_state.resource_manager.add_resource(powerline, &mut ggez_context).unwrap();

    // Run the game!
    match ggez_event::run(&mut ggez_context, &mut ggez_event_loop, &mut sand_casting_game_state) {
        Ok(_)   => println!("Exited cleanly."),
        Err(e)  => println!("Error occured: {}", e)
    }
}
