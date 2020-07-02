/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : weather.rs

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
    This module manages weather effects over the course of the game, including
    but not limited to generating random weather events.

Changelog:
    CJ McAllister   02 Jul 2020     File created

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use cast_iron::{
    environment::{
        Element,
        weather
    },
    polyfunc::PolyFunc
};

use ggez::{
    Context as GgEzContext,
    timer as ggez_timer
};


///////////////////////////////////////////////////////////////////////////////
// Data structures
///////////////////////////////////////////////////////////////////////////////

pub struct WeatherManager {
    pub active_weather:     weather::Weather, //FIXME: Needs to be public?
    pub timeout:            u8
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl WeatherManager {
    pub fn new() -> WeatherManager {
        WeatherManager {
            active_weather:     weather::Weather::new(),
            timeout:            0
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    pub fn update_weather(&mut self, ctx: &GgEzContext) {
        // Get current state info from GGEZ context
        let cur_tick = ggez_timer::ticks(ctx) as u32;
        
        // If current weather has timed out, randomly generate a new weather pattern
        if self.timeout == 0 {
            let rand_element    = Element::from((rand::random::<u8>() % 8) + 1); //FIXME: this sucks
            
            let rand_magnitude  = rand::random::<u8>();
            let rand_duration   = rand::random::<u8>();
            let rand_func = PolyFunc::from(rand_magnitude, rand_duration, cur_tick);

            self.active_weather = weather::Weather::from(rand_element, rand_func);
            println!(
                "Tick {:>3}: Weather changed to Mag: {:>3}  Dur: {:>3}  Elem: {:?}",
                cur_tick,
                rand_magnitude,
                rand_duration,
                rand_element
            );

            // Set the timeout to the duration of the new weather pattern
            self.timeout = rand_duration;
        }
        else { // Otherwise, decrement the timeout
            self.timeout = self.timeout - 1;
            println!(
                "Tick {:>3}: Weather (TO: {:>3}, Kind: {:?}, Int: {:?})",
                cur_tick,
                self.timeout,
                self.active_weather.kind(),
                self.active_weather.intensity(cur_tick)
            );
        }
    }
}