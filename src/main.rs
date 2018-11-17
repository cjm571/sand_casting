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

use std::f64;
use std::thread;
use std::time::Duration;

pub mod shape;
use shape::hexagon::Hexagon;
use shape::point::Point;


///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

const WINDOW_X: u32 = 800;
const WINDOW_Y: u32 = 600;

const GRID_SIZE: f64 = 50.0;

// X_OFFSET = (GRID_SIZE/2) + (GRID_SIZE * cos(pi/3))
static X_OFFSET: f64 = GRID_SIZE + (GRID_SIZE * 0.5);
// Y_OFFSET = GRID_SIZE * sin(pi/3) * 2
static Y_OFFSET: f64 = GRID_SIZE * 0.86602540378;

const BLACK:    Color = [0.0, 0.0, 0.0, 1.0];
const WHITE:    Color = [1.0, 1.0, 1.0, 1.0];
const RED:      Color = [1.0, 0.0, 0.0, 1.0];
const GREEN:    Color = [0.0, 1.0, 0.0, 1.0];
const BLUE:     Color = [0.0, 0.0, 1.0, 1.0];
const YELLOW:   Color = [1.0, 1.0, 0.0, 1.0];
const CYAN:     Color = [0.0, 1.0, 1.0, 1.0];
const PURPLE:   Color = [1.0, 0.0, 1.0, 1.0];
const GREY:     Color = [0.5, 0.5, 0.5, 1.0];

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

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let mut grid_elem = Hexagon::from(center, GRID_SIZE);
            grid_elem.draw(c.transform, gl);

            let mut next_hex;
            
            // Draw E line of hexes
            for i in 0..=5 {
                next_hex = Point::from(center.x + (GRID_SIZE * 3.0 * i as f64), center.y);
                grid_elem = Hexagon::from(next_hex, GRID_SIZE);
                grid_elem.draw(c.transform, gl);
            }
            // Draw N line of hexes
            for i in 0..=5 {
                next_hex = Point::from(center.x, center.y - (Y_OFFSET * 2.0 * i as f64));
                grid_elem = Hexagon::from(next_hex, GRID_SIZE);
                grid_elem.draw(c.transform, gl);
            }
            // Draw W line of hexes
            for i in 0..=5 {
                next_hex = Point::from(center.x - (GRID_SIZE * 3.0 * i as f64), center.y);
                grid_elem = Hexagon::from(next_hex, GRID_SIZE);
                grid_elem.draw(c.transform, gl);
            }
            // Draw S line of hexes
            for i in 0..=5 {
                next_hex = Point::from(center.x, center.y + (Y_OFFSET * 2.0 * i as f64));
                grid_elem = Hexagon::from(next_hex, GRID_SIZE);
                grid_elem.draw(c.transform, gl);
            }

            // Draw NE line of hexes
            for i in 0..=5 {
                next_hex = Point::from(center.x + (X_OFFSET * i as f64), center.y - (Y_OFFSET * i as f64));
                grid_elem = Hexagon::from(next_hex, GRID_SIZE);
                grid_elem.draw(c.transform, gl);
            }
            // Draw NW line of hexes
            for i in 0..=5 {
                next_hex = Point::from(center.x - (X_OFFSET * i as f64), center.y - (Y_OFFSET * i as f64));
                grid_elem = Hexagon::from(next_hex, GRID_SIZE);
                grid_elem.draw(c.transform, gl);
            }
            // Draw SW line of hexes
            for i in 0..=5 {
                next_hex = Point::from(center.x - (X_OFFSET * i as f64), center.y + (Y_OFFSET * i as f64));
                grid_elem = Hexagon::from(next_hex, GRID_SIZE);
                grid_elem.draw(c.transform, gl);
            }
            // Draw SE line of hexes
            for i in 0..=5 {
                next_hex = Point::from(center.x + (X_OFFSET * i as f64), center.y + (Y_OFFSET * i as f64));
                grid_elem = Hexagon::from(next_hex, GRID_SIZE);
                grid_elem.draw(c.transform, gl);
            }
        });
    }

    fn update (&mut self, args: &UpdateArgs) {
        // Rotate 2rad/s
        self.rotation += 2.0 * args.dt;
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


    ///////////////
    // Main Loop //
    ///////////////
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

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

