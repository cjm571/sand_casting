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

use std::env;

use cast_iron::{
    ability::{aspect::*, Ability},
    actor::Actor,
    context::ContextBuilder as CastIronContextBuilder,
    element::Element,
};

use ggez::{
    conf as ggez_conf, event as ggez_event, graphics as ggez_gfx,
    ContextBuilder as GgEzContextBuilder,
};

use mt_logger::{mt_log, mt_new, Level, OutputStream};

///
// Module Declarations
///
pub mod game_assets;
use game_assets::colors;

pub mod game_managers;

pub mod game_state;
use game_state::SandCastingGameState;

pub mod profiler;


///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

/* Window Appearance */
const DEFAULT_WINDOW_SIZE_X: f32 = 1000.0;
const DEFAULT_WINDOW_SIZE_Y: f32 = 1000.0;
const DESIRED_FPS: u32 = 60;

const DEFAULT_TEXT_SIZE: f32 = 16.0;
const DEFAULT_LINE_WIDTH: f32 = 2.0;
const DEFAULT_LINE_COLOR: ggez_gfx::Color = colors::WHITE;


/* Hex Grid */
/// Distance from centerpoint of hex to center of a side
const HEX_RADIUS_VERTEX: f32 = 25.0;

/// Distance from centerpoint of hex to center of a side
const HEX_RADIUS_SIDE: f32 = HEX_RADIUS_VERTEX * 0.866_025_4;


/* Mechanics */
/// Default hexagonal grid radius (in cells)
const DEFAULT_GRID_RADIUS: usize = 10;

/// Default maximum number of attempts before considering random mechanic generation a failure
const DEFAULT_MAX_RAND_ATTEMPTS: usize = 10;

/// Default maximum for the radius of resources (in cells)
const DEFAULT_MAX_RESOURCE_RADIUS: usize = 4;

/// Default maximum for the length of an obstacle (in cells)
const DEFAULT_MAX_OBSTACLE_LENGTH: usize = 10;

/// Default maximum intensity of a weather event
const DEFAULT_MAX_WEATHER_INTENSITY: f64 = 256.0;

/// Default maximum duration for a weather event (in seconds)
const DEFAULT_MAX_WEATHER_DURATION: f64 = 10.0;


fn main() {
    //OPT: *DESIGN* Replace this with pattern from once_cell example
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    // Initialize logger instance if specified
    if args.contains(&String::from("-log")) {
        mt_new!(None, Level::Info, OutputStream::Both);
    } else if args.contains(&String::from("-debug")) {
        mt_new!(None, Level::Debug, OutputStream::Both);
    } else if args.contains(&String::from("-trace")) {
        mt_new!(None, Level::Trace, OutputStream::Both);
    }

    // Create profiler instance, or disable if required
    let profiler_original;
    if args.contains(&String::from("-profile")) {
        profiler_original = profiler::Instance::default();
    } else {
        profiler_original = profiler::Instance::disabled();
    }

    // Create CastIron game context
    let ci_ctx = CastIronContextBuilder::default()
        .grid_radius(DEFAULT_GRID_RADIUS)
        .max_obstacle_len(DEFAULT_MAX_OBSTACLE_LENGTH)
        .max_rand_attempts(DEFAULT_MAX_RAND_ATTEMPTS)
        .max_resource_radius(DEFAULT_MAX_RESOURCE_RADIUS)
        .max_weather_duration(DEFAULT_MAX_WEATHER_DURATION)
        .max_weather_intensity(DEFAULT_MAX_WEATHER_INTENSITY)
        .build();

    mt_log!(Level::Debug, "CastIron context created.");

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

    // Initialize Actor
    let mut player_one: Actor = Actor::new_name_only("CJ McAllister");
    player_one.add_ability(lightning_bolt);
    player_one.add_ability(blood_drain);
    player_one.add_ability(null_abil);

    // Create a GGEZ Context and EventLoop
    let (mut ggez_ctx, ggez_event_loop) = GgEzContextBuilder::new("sand_casting", "CJ McAllister")
        .window_setup(
            ggez_conf::WindowSetup::default()
                .title("Sand Casting - A CastIron Sandbox Game")
                .vsync(false),
        )
        .window_mode(
            ggez_conf::WindowMode::default()
                .dimensions(DEFAULT_WINDOW_SIZE_X, DEFAULT_WINDOW_SIZE_Y),
        )
        .build()
        .unwrap();
    mt_log!(Level::Info, "ggez context, event loop created.");

    // Use built context to create a GGEZ Event Handler instance
    let sand_casting_game_state =
        SandCastingGameState::new(&profiler_original, &ci_ctx, &mut ggez_ctx);

    // Run the game!
    ggez_event::run(ggez_ctx, ggez_event_loop, sand_casting_game_state);
}
