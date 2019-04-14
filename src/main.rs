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

extern crate cast_iron;
use cast_iron::actor::Actor;
use cast_iron::ability::Ability;
use cast_iron::ability::aspect::*;
use cast_iron::environment::Element;
use cast_iron::environment::weather::Weather;
use cast_iron::environment::world_grid::*;
use cast_iron::polyfunc::PolyFunc;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::types::*;
use graphics::line;
use graphics::Graphics;

use std::f64;
use std::f64::consts::PI;
use std::thread;
use std::time::Duration;

pub mod shape;
use shape::point::Point;
use shape::hexagon::Hexagon;


///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

const WINDOW_X: u32 = 800;
const WINDOW_Y: u32 = 600;

const GRID_SIZE: f64 = 50.0;

// Y_OFFSET = GRID_SIZE * sin(pi/3) * 2
// Distance from centerpoint of hex to center of a side 
static Y_OFFSET: f64 = GRID_SIZE * 0.86602540378;

#[allow(dead_code)]
const BLACK:    Color = [0.0, 0.0, 0.0, 1.0];
#[allow(dead_code)]
const WHITE:    Color = [1.0, 1.0, 1.0, 1.0];
#[allow(dead_code)]
const RED:      Color = [1.0, 0.0, 0.0, 1.0];
#[allow(dead_code)]
const GREEN:    Color = [0.0, 1.0, 0.0, 1.0];
#[allow(dead_code)]
const BLUE:     Color = [0.0, 0.0, 1.0, 1.0];
#[allow(dead_code)]
const YELLOW:   Color = [1.0, 1.0, 0.0, 1.0];
#[allow(dead_code)]
const CYAN:     Color = [0.0, 1.0, 1.0, 1.0];
#[allow(dead_code)]
const PURPLE:   Color = [1.0, 0.0, 1.0, 1.0];
#[allow(dead_code)]
const GREY:     Color = [0.5, 0.5, 0.5, 1.0];

const WORLD_GRID: WorldGrid = WorldGrid {size:10};

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

pub struct App {
    gl: GlGraphics,         // OpenGL drawing backend
    rotation: f64,          // Rotation for the square
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::clear;
        let center = Point::from(args.width as f64 / 2.0, args.height as f64 / 2.0);

        //TODO: TEMP CODE, DELETE
        let mut tempHex: Hexagon = Hexagon::from(center, GRID_SIZE);
        tempHex.color = RED;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            draw_hex_grid(center, c.transform, gl);
            tempHex.draw(c.transform, gl);
        });
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

    // Piston graphics stuff
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "Cast(Iron) Sandb(Oxide)",
            [WINDOW_X, WINDOW_Y]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    // Create a new game and run it
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0
    };
    
    /*  *  *  *  *  *  *  *  *  *  *\
     *    T I M I N G   L O O P    *
    \*  *  *  *  *  *  *  *  *  *  */
     
    thread::spawn(move || {
        const MAX_TICKS: u32 = 31;
        let mut tick: u32 = 0;
        
        while tick <= MAX_TICKS {
            println!("Tick {}: Weather: {:?}", tick, thunderstorm.intensity(tick));

            tick = tick + 1;
            thread::sleep(Duration::from_secs(1));
        }
    });


    /*  *  *  *  *  *  *  *  *\
     *   M A I N   L O O P   *
    \*  *  *  *  *  *  *  *  */
    let mut events = Events::new(EventSettings {
        max_fps:        144,
        ups:            DEFAULT_UPS,
        ups_reset:      DEFAULT_UPS_RESET,
        swap_buffers:   true,
        bench_mode:     false,
        lazy:           false,
    });

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}

/// Draws a baseline hex grid to the graphics window.
fn draw_hex_grid<G>(center: Point, transform: Matrix2d, g: &mut G)
where G: Graphics {
    // Draw GRID_SIZE-width hexagon sides recursively
    recursive_hex_draw(WHITE, center, 0, transform, g);

    // Draw spokes recursively in all directions
    for (_dir, theta) in HEX_VERTICES.iter() {
        // Determine origin point for current direction
        let origin = Point::from(
            center.x + (GRID_SIZE * theta.cos()),
            center.y - (GRID_SIZE * theta.sin())
        );
        recursive_spoke_draw(WHITE, origin, *theta, 0, transform, g);
    }
}

/// Draws a hex grid at the given level using recursive calls radiating out
/// from the given center.
fn recursive_hex_draw<G>(color: Color, center: Point, level: u32, transform: Matrix2d, g: &mut G)
where G: Graphics {
    // Final level exit case
    if level == WORLD_GRID.size {
        return;
    }

    // HEX_SIE to be used to correctly translate levels > 0
    static HEX_SIZE: f64 = Y_OFFSET * 2.0;
    
    // Draw a parallel line and dispatch a spoke draw call at the current level
    // for each intercardinal direction.
    for (_dir, theta) in HEX_SIDES.iter() {
        // Calculate parallel line endpoints
        let mut x = center.x + GRID_SIZE * (theta - PI/6.0).cos();
        let mut y = center.y - GRID_SIZE * (theta - PI/6.0).sin();
        let mut endpt_a: Point = Point::from(x, y);

        x = center.x + GRID_SIZE * (theta + PI/6.0).cos();
        y = center.y - GRID_SIZE * (theta + PI/6.0).sin();
        let mut endpt_b: Point = Point::from(x, y);

        // Translate lines based on level
        endpt_a.x = endpt_a.x + level as f64 * (HEX_SIZE * theta.cos());
        endpt_a.y = endpt_a.y - level as f64 * (HEX_SIZE * theta.sin());
        endpt_b.x = endpt_b.x + level as f64 * (HEX_SIZE * theta.cos());
        endpt_b.y = endpt_b.y - level as f64 * (HEX_SIZE * theta.sin());

        // Draw the line
        line(color, 0.5, [endpt_a.x, endpt_a.y, endpt_b.x, endpt_b.y], transform, g);
    }
    
    // Make the recursive call
    recursive_hex_draw(color, center, level+1, transform, g);
}

/// Draws a spoke (i.e. -<) from a point in the given direction.
/// Recursively spawns two more spoke draws at the endpoint
fn recursive_spoke_draw<G>(color: Color, origin: Point, theta: f64, level: u32, transform: Matrix2d, g: &mut G)
where G: Graphics {
    // Final level exit case
    if level == WORLD_GRID.size {
        return;
    }

    let mut lines: [[f64; 4]; 3] = [[0.0; 4]; 3];
    let mut endpoints: [Point; 3] = [Point::new(); 3];

    // Calculate endpoint of stem
    endpoints[0] = Point::from(
        origin.x + (GRID_SIZE * theta.cos()),
        origin.y - (GRID_SIZE * theta.sin())
    );
    lines[0] = [origin.x, origin.y,
                endpoints[0].x, endpoints[0].y];

    // Calculate branch endpoints
    endpoints[1] = Point::from(
        endpoints[0].x + (GRID_SIZE * (theta + PI/3.0).cos()),
        endpoints[0].y - (GRID_SIZE * (theta + PI/3.0).sin())
    );
    endpoints[2] = Point::from(
        endpoints[0].x + (GRID_SIZE * (theta - PI/3.0).cos()),
        endpoints[0].y - (GRID_SIZE * (theta - PI/3.0).sin())
    );
    lines[1] = [endpoints[0].x, endpoints[0].y,
                endpoints[1].x, endpoints[1].y];
    lines[2] = [endpoints[0].x, endpoints[0].y,
                endpoints[2].x, endpoints[2].y];

    // Draw lines
    for i in 0..=2 {
        line (color, 0.5, lines[i], transform, g);
    }

    // Make the recursive calls
    recursive_spoke_draw(color, endpoints[1], theta, level+1, transform, g);
    recursive_spoke_draw(color, endpoints[2], theta, level+1, transform, g);
}
 
 