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
    logger,
    ci_log,
};
// use cast_iron::environment::resource;
// use cast_iron::environment::coords::Coords;

extern crate ggez;
use ggez::{
    ContextBuilder as GgEzContextBuilder,
    conf as ggez_conf,
    event as ggez_event,
    graphics as ggez_gfx,
};

///
// Module Declarations
///
pub mod game_assets;
use game_assets::colors;

pub mod game_managers;

pub mod game_state;
use game_state::SandCastingGameState;


///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

/* Appearence */
const DEFAULT_WINDOW_SIZE_X:    f32 = 1000.0;
const DEFAULT_WINDOW_SIZE_Y:    f32 = 1000.0;
const DESIRED_FPS:              u32 = 60;

const DEFAULT_TEXT_SIZE:        f32 = 16.0;
const DEFAULT_LINE_WIDTH:       f32 = 2.0;
const DEFAULT_LINE_COLOR:       ggez_gfx::Color = colors::WHITE;
const DEFAULT_FILL_COLOR:       ggez_gfx::Color = colors::GREY;

/* Hex Grid */
const GRID_CELL_SIZE: f32 = 25.0;
// CENTER_TO_SIDE_DIST = GRID_CELL_SIZE * sin(pi/3)
// Distance from centerpoint of hex to center of a side
static CENTER_TO_SIDE_DIST: f32 = GRID_CELL_SIZE * 0.866_025_4;
// CENTER_TO_VERTEX_DIST = GRID_CELL_SIZE * cos(pi/3)
// Distance from centerpoint of hex to center of a side
static CENTER_TO_VERTEX_DIST: f32 = GRID_CELL_SIZE * 0.5;

/* Mechanics */
const DEFAULT_HEX_GRID_MAX_RADIAL_DISTANCE: usize = 10;
// const DEFAULT_MAX_OBSTACLE_LENGTH: u8 = 5;


fn main() {
    // Create logger instance
    let logger_original = logger::Instance::default();

    // Create CastIron game context
    let ci_ctx = CastIronContext::default();
    ci_log!(logger_original, logger::FilterLevel::Debug, "CastIron context created.");

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

    //FEAT: Make actors do stuff
    // Intialize Actor
    let mut player_one: Actor = Actor::new_name_only("CJ McAllister");
    player_one.add_ability(lightning_bolt);
    player_one.add_ability(blood_drain);
    player_one.add_ability(null_abil);

    // Create a GGEZ Context and EventLoop
    let (mut ggez_ctx, mut ggez_event_loop) = GgEzContextBuilder::new("sand_casting", "CJ McAllister")
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
    ci_log!(logger_original, logger::FilterLevel::Info, "ggez context, event loop created.");

    // Use built context to create a GGEZ Event Handler instance
    let mut sand_casting_game_state = SandCastingGameState::new(&logger_original, &mut ggez_ctx);

    // Create random resources
    for _i in 0..3 {
        sand_casting_game_state.resource_manager().add_rand_resource(&ci_ctx, &mut ggez_ctx).unwrap();
    }
    ci_log!(logger_original, logger::FilterLevel::Info, "Resources generated.");

    // Create random obstacles
    for _i in 0..3 {
        sand_casting_game_state.obstacle_manager().add_rand_obstacle(&ci_ctx, &mut ggez_ctx).unwrap();
    }
    ci_log!(logger_original, logger::FilterLevel::Info, "Obstacles generated.");

    // Run the game!
    match ggez_event::run(&mut ggez_ctx, &mut ggez_event_loop, &mut sand_casting_game_state) {
        Ok(_)   => println!("Exited cleanly."),
        Err(e)  => println!("Error occured: {}", e)
    }
}
