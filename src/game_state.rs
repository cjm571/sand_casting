/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : game_state.rs

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
    context::Context as CastIronContext, 
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
    game_assets::colors,
    game_managers::{
        DrawableMechanic,
        actor_manager::ActorManager,
        obstacle_manager::ObstacleManager,
        resource_manager::ResourceManager,
        weather_manager::WeatherManager,
        world_grid_manager::WorldGridManager,
    },
    profiler,
};


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

/// Primary Game Struct
pub struct SandCastingGameState {
    initialized:        bool,               // Flag indicating if game has been initialized
    peak_fps:           f64,                // Peak FPS for display
    ci_ctx:             CastIronContext,    // CastIron engine context
    logger:             logger::Instance,   // Instance of CastIron Logger
    profiler:           profiler::Instance, // Instance of SandCasting performance profiler
    actor_manager:      ActorManager,       // Actor Manager instance
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
    pub fn new(logger_original: &logger::Instance, ci_ctx: &CastIronContext, ggez_ctx: &mut GgEzContext) -> Self {
        //NOTE: Load/create resources here: images, fonts, sounds, etc.

        // Clone the logger instance so this module has its own sender to use
        let logger_clone = logger_original.clone();

        // Clone context for use by submodules
        let ctx_clone = ci_ctx.clone();

        SandCastingGameState{
            initialized:        false,
            peak_fps:           0.0,
            ci_ctx:             ctx_clone,
            logger:             logger_clone,
            profiler:           profiler::Instance::default(),
            actor_manager:      ActorManager::new(logger_original, ggez_ctx),
            obstacle_manager:   ObstacleManager::new(logger_original, ggez_ctx),
            resource_manager:   ResourceManager::new(logger_original, ggez_ctx),
            weather_manager:    WeatherManager::default(logger_original, ci_ctx, ggez_ctx),
            world_grid_manager: WorldGridManager::new(logger_original, ::DEFAULT_HEX_GRID_MAX_RADIAL_DISTANCE, ggez_ctx),
        }
    }


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */

    pub fn initialized(&mut self) -> bool {
        self.initialized
    }

    pub fn actor_manager(&mut self) -> &mut ActorManager {
        &mut self.actor_manager
    }

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


    /*  *  *  *  *  *  *\
     *  Utiliy Methods *
    \*  *  *  *  *  *  */

    fn initialize(&mut self, ggez_ctx: &mut GgEzContext) {
        // Create random resources
        for _i in 0..3 {
            self.resource_manager.add_rand_instance(&self.ci_ctx, ggez_ctx).unwrap();
        }
        ci_log!(self.logger, logger::FilterLevel::Info, "Resources generated.");

        // Create random obstacles
        for _i in 0..3 {
            self.obstacle_manager.add_rand_instance(&self.ci_ctx, ggez_ctx).unwrap();
        }
        ci_log!(self.logger, logger::FilterLevel::Info, "Obstacles generated.");
        
        // Create random actors
        for _i in 0..3 {
            self.actor_manager.add_rand_instance(&self.ci_ctx, ggez_ctx).unwrap();
        }
        ci_log!(self.logger, logger::FilterLevel::Info, "Actors generated.");

        ci_log!(self.logger, logger::FilterLevel::Info, "First-frame initialization complete.");
        self.initialized = true;
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

//FEAT: Add performance metrics to update, draw
//FEAT: Handle mouse events
//FEAT: Handle keyboard events, '~' for a debug view would be cool
impl ggez_event::EventHandler for SandCastingGameState {
    fn update(&mut self, ggez_ctx: &mut GgEzContext) -> GgEzGameResult<()> {
        // Check if first-frame initialization is required
        if !self.initialized() {
            self.initialize(ggez_ctx);
        }

        // Check if we've reached an update
        while ggez_timer::check_update_time(ggez_ctx, ::DESIRED_FPS) {
            // Update weather
            ci_log!(self.logger, logger::FilterLevel::Trace, "Updating weather...");
            self.weather_manager.update_weather(&self.ci_ctx, ggez_ctx);

            // Update FPS
            self.profiler.update_avg_fps(ggez_ctx).unwrap();
            if self.profiler.avg_fps() > self.peak_fps {
                self.peak_fps = self.profiler.avg_fps();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut GgEzContext) -> GgEzGameResult<()> {
        ggez_gfx::clear(ctx, colors::BLACK);
        
        // Draw the weather HUD
        self.weather_manager.draw(ctx);
        
        //OPT: *DESIGN* Other managers should have a draw() call like weather manager
        // Draw the hex grid
        self.world_grid_manager.draw(ctx);

        // Draw resources
        self.resource_manager.draw(ctx);

        // Draw obstacles
        self.obstacle_manager.draw(ctx);

        // Draw actors
        self.actor_manager.draw(ctx);

        //FEAT: Could make a 'performance manager' or something to encapsulate things like FPS counters
        // Draw the FPS counters
        let avg_fps_pos = ggez_mint::Point2 {x: 0.0, y: 0.0};
        let avg_fps_str = format!("Avg. FPS: {:.0}", self.profiler.avg_fps());
        let avg_fps_display = ggez_gfx::Text::new((avg_fps_str, ggez_gfx::Font::default(), ::DEFAULT_TEXT_SIZE));
        ggez_gfx::draw(ctx, &avg_fps_display, (avg_fps_pos, 0.0, colors::GREEN)).unwrap();

        let peak_fps_pos = ggez_mint::Point2 {x: 0.0, y: 20.0};
        let peak_fps_str = format!("Peak FPS: {:.0}", self.peak_fps);
        let peak_fps_display = ggez_gfx::Text::new((peak_fps_str, ggez_gfx::Font::default(), ::DEFAULT_TEXT_SIZE));
        ggez_gfx::draw(ctx, &peak_fps_display, (peak_fps_pos, 0.0, colors::GREEN)).unwrap();

        ggez_gfx::present(ctx)
    }
}
