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


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////
/* GGEZ STUFF
// ggez game state object
struct MainState {
    text:   graphics::Text,
    frames: usize,
}
impl MainState {
    // Constructor
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;
        let text = graphics::Text::new(ctx, "Hello World!", &font)?;

        let s = MainState {text, frames: 0};
        Ok(s)
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // Drawables are drawn from their top-left corner
        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::draw(ctx, &self.text, dest_point, 0.0)?;
        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }
}

 */
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
/* GGEZ STUFF
    // Set up ggez frame
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();

    // Add CARGO_MANIFEST_DIR/resources to path so ggez will look in our cargo project
    // directory for files
    let mut path = path::PathBuf::from("C:\\Users\\cjm57\\Documents\\Rust_Projects\\iron_sandboxide\\");
    path.push("resources");
    ctx.filesystem.mount(&path, true);

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
 */
    
    ///////////////
    // Main Loop //
    ///////////////
    
    const MAX_TICKS: u32 = 31;
    let mut tick: u32 = 0;

    while tick <= MAX_TICKS {
        println!("Tick {} Weather: {:?}", tick, thunderstorm.intensity(tick));

        tick = tick + 1;
    }
}

