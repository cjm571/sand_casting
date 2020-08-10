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

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use cast_iron::{
    environment::{
        element::Element,
        weather::Weather
    },
    polyfunc::PolyFunc,
    logger::{
        LoggerInstance,
        LogLevel
    },
    ci_log
};

use ggez::{
    Context as GgEzContext,
    timer as ggez_timer
};

use rand::Rng;


///////////////////////////////////////////////////////////////////////////////
// Data structures
///////////////////////////////////////////////////////////////////////////////

pub struct WeatherManager {
    logger:         LoggerInstance,
    active_weather: Weather,
    timeout:        usize,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl WeatherManager {
    /// Fully-qualified constructor
    pub fn new(logger: &LoggerInstance, active_weather: Weather, timeout: usize) -> Self {
        // Clone the logger instance so this module has its own sender to use
        let logger_clone = logger.clone();

        WeatherManager {
            logger:         logger_clone,
            active_weather: active_weather,
            timeout:        timeout,
        }
    }

    /// Logger-only constructor
    pub fn new_logger_only(logger: &LoggerInstance) -> Self {
        // Clone the logger instance so this module has its own sender to use
        let logger_clone = logger.clone();

        WeatherManager {
            logger:         logger_clone,
            active_weather: Weather::default(),
            timeout:        usize::default(),
        }
    }


    ///
    // Accessor Methods
    ///

    pub fn update_weather(&mut self, ggez_ctx: &GgEzContext) {
        // Get current state info from GGEZ context
        let cur_tick = ggez_timer::ticks(ggez_ctx);

        // If current weather has timed out, randomly generate a new weather pattern
        if self.timeout == 0 {
            let mut rng = rand::thread_rng();

            let rand_element: Element = rng.gen();

            let rand_magnitude: usize = rng.gen();
            let rand_duration: usize = rng.gen();
            let rand_func = PolyFunc::new(rand_magnitude, rand_duration, cur_tick);

            self.active_weather = Weather::new(rand_element, rand_func);
            ci_log!(self.logger, LogLevel::DEBUG, 
                "Tick {:>8}: Weather changed to Mag: {:>3}  Dur: {:>3}  Elem: {:?}",
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
        }
    }
}