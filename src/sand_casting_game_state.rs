/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : sand_casting_game_state.rs

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
    Defines structures and functions that make up the Sand Casting game state.

    This includes ggez event-handling functions such as update() and draw().

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use cast_iron::{
    logger,
    ci_log,
};

use ggez::{
    Context as GgEzContext,
    GameResult as GgEzGameResult,
    event as ggez_event,
    graphics as ggez_gfx,
    mint as ggez_mint,
    timer as ggez_timer,
};

use crate::{
    game_assets::colors::*,
    obstacle_manager::ObstacleManager,
    resource_manager::ResourceManager,
    weather_manager::WeatherManager,
    world_grid_manager::WorldGridManager,
};


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

/// Primary Game Struct
pub struct SandCastingGameState {
    avg_fps:            f64,                // Average FPS for display
    peak_fps:           f64,                // Peak FPS for display
    logger:             logger::Instance,     // Instance of CastIron Logger
    obstacle_manager:   ObstacleManager,    // Obstacle Manager instance
    resource_manager:   ResourceManager,    // Resource Manager instance
    weather_manager:    WeatherManager,     // Weather Manager instance
    world_grid_manager: WorldGridManager,   // World Grid Manager instance
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

/// Constructor
impl SandCastingGameState {
    pub fn new(logger_original: &logger::Instance, ggez_ctx: &mut GgEzContext) -> Self {
        //NOTE: Load/create resources here: images, fonts, sounds, etc.

        // Clone the logger instance so this module has its own sender to use
        let logger_clone = logger_original.clone();

        SandCastingGameState{
            avg_fps:            0.0,
            peak_fps:           0.0,
            logger:             logger_clone,
            obstacle_manager:   ObstacleManager::new(logger_original, ggez_ctx),
            resource_manager:   ResourceManager::new(logger_original, ggez_ctx),
            weather_manager:    WeatherManager::default(logger_original, ggez_ctx),
            world_grid_manager: WorldGridManager::new(logger_original, ::DEFAULT_HEX_GRID_MAX_RADIAL_DISTANCE, ggez_ctx),
        }
    }


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */

    pub fn obstacle_manager(&mut self) -> &mut ObstacleManager {
        &mut self.obstacle_manager
    }

    pub fn resource_manager(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    pub fn weather_manager(&mut self) -> &mut WeatherManager {
        &mut self.weather_manager
    }

    pub fn world_grid_manager(&mut self) -> &mut WorldGridManager {
        &mut self.world_grid_manager
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

//FEAT: Handle mouse events
//FEAT: Handle keyboard events, '~' for a debug view would be cool
impl ggez_event::EventHandler for SandCastingGameState {
    fn update(&mut self, ggez_ctx: &mut GgEzContext) -> GgEzGameResult<()> {
        while ggez_timer::check_update_time(ggez_ctx, ::DESIRED_FPS) {
            // Update weather
            ci_log!(self.logger, logger::FilterLevel::Trace, "Updating weather...");
            self.weather_manager.update_weather(ggez_ctx);

            // Update FPS
            self.avg_fps = ggez_timer::fps(ggez_ctx);
            if self.avg_fps > self.peak_fps {
                self.peak_fps = self.avg_fps;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut GgEzContext) -> GgEzGameResult<()> {
        ggez_gfx::clear(ctx, BLACK);

        // Draw the weather HUD
        self.weather_manager.draw(ctx);

        //OPT: *DESIGN* Other managers should have a draw() call like weather manager
        // Draw the hex grid
        ggez_gfx::draw(ctx, self.world_grid_manager.base_grid_mesh(), ggez_gfx::DrawParam::default())?;

        // Draw resources
        ggez_gfx::draw(ctx, self.resource_manager.resource_mesh(), ggez_gfx::DrawParam::default())?;

        // Draw obstacles
        ggez_gfx::draw(ctx, self.obstacle_manager.obstacle_mesh(), ggez_gfx::DrawParam::default())?;

        //OPT: *DESIGN* Could make a 'performance manager' or something to encapsulate things like FPS counters
        // Draw the FPS counters
        let avg_fps_pos = ggez_mint::Point2 {x: 0.0, y: 0.0};
        let avg_fps_str = format!("Avg. FPS: {:.0}", self.avg_fps);
        let avg_fps_display = ggez_gfx::Text::new((avg_fps_str, ggez_gfx::Font::default(), ::DEFAULT_TEXT_SIZE));
        ggez_gfx::draw(ctx, &avg_fps_display, (avg_fps_pos, 0.0, GREEN)).unwrap();

        let peak_fps_pos = ggez_mint::Point2 {x: 0.0, y: 20.0};
        let peak_fps_str = format!("Peak FPS: {:.0}", self.peak_fps);
        let peak_fps_display = ggez_gfx::Text::new((peak_fps_str, ggez_gfx::Font::default(), ::DEFAULT_TEXT_SIZE));
        ggez_gfx::draw(ctx, &peak_fps_display, (peak_fps_pos, 0.0, GREEN)).unwrap();

        ggez_gfx::present(ctx)
    }
}