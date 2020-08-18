/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : weather_manager.rs

Copyright (C) 2020 CJ McAllister
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
        element::Elemental,
        weather::Weather
    },
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


///////////////////////////////////////////////////////////////////////////////
// Data structures
///////////////////////////////////////////////////////////////////////////////

pub struct WeatherManager {
    logger:         LoggerInstance,
    active_weather: Weather,
    timeout_tick:   usize,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl WeatherManager {
    /// Fully-qualified constructor
    pub fn new(logger: &LoggerInstance, active_weather: Weather, timeout_tick: usize) -> Self {
        // Clone the logger instance so this module has its own sender to use
        let logger_clone = logger.clone();

        WeatherManager {
            logger:         logger_clone,
            active_weather: active_weather,
            timeout_tick:   timeout_tick,
        }
    }

    /// Default constructor
    pub fn default(logger: &LoggerInstance) -> Self {
        // Clone the logger instance so this module has its own sender to use
        let logger_clone = logger.clone();

        WeatherManager {
            logger:         logger_clone,
            active_weather: Weather::default(),
            timeout_tick:   usize::default(),
        }
    }


    ///
    // Utility Methods
    ///

    pub fn update_weather(&mut self, ggez_ctx: &GgEzContext) {
        let cur_tick = ggez_timer::ticks(ggez_ctx);

        // If current weather has timed out, randomly generate a new weather pattern
        if cur_tick >= self.timeout_tick {
            self.active_weather = Weather::rand_starting_at(cur_tick);
            ci_log!(self.logger, LogLevel::INFO,
                "Tick {:>8}: Weather changed to Elem: {:?}, Duration: {}",
                cur_tick,
                self.active_weather.get_element(),
                self.active_weather.get_duration()
            );
            
            // Set the timeout to the duration of the new weather pattern
            self.timeout_tick = cur_tick + self.active_weather.get_duration();
        }
    }
}
