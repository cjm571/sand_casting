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
    pub fn new(logger_original: &logger::Instance,
               profiler_original: &profiler::Instance,
               ci_ctx: &CastIronContext,
               ggez_ctx: &mut GgEzContext) -> Self {
        //NOTE: Load/create resources here: images, fonts, sounds, etc.

        // Clone the logger, profiler instances for use by this module
        let logger_clone = logger_original.clone();
        let profiler_clone = profiler_original.clone();

        // Clone context for use by submodules
        let ctx_clone = ci_ctx.clone();

        SandCastingGameState{
            initialized:        false,
            ci_ctx:             ctx_clone,
            logger:             logger_clone,
            profiler:           profiler_clone,
            actor_manager:      ActorManager::new(logger_original, ggez_ctx),
            obstacle_manager:   ObstacleManager::new(logger_original, ggez_ctx),
            resource_manager:   ResourceManager::new(logger_original, ggez_ctx),
            weather_manager:    WeatherManager::default(logger_original, profiler_original, ci_ctx, ggez_ctx),
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
            self.profiler.update_fps_stats(ggez_ctx).unwrap();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut GgEzContext) -> GgEzGameResult<()> {
        // After the first frame, send previous frame's time delta to the profiler
        if ggez_timer::ticks(ctx) > 1 {
            self.profiler.send_frame_delta(ctx).unwrap();
        }
        
        // Get draw start time and set up vec for stacked draw time
        let start_time = ggez_timer::time_since_start(ctx);
        let mut draw_timings = Vec::new();

        ggez_gfx::clear(ctx, colors::BLACK);
        draw_timings.push(profiler::StackedTime{label: String::from("Clear"), time: ggez_timer::time_since_start(ctx)});
        
        // Draw the weather HUD
        self.weather_manager.draw(ctx);
        draw_timings.push(profiler::StackedTime{label: String::from("Weather"), time: ggez_timer::time_since_start(ctx)});
        
        // Draw the hex grid
        self.world_grid_manager.draw(ctx);
        draw_timings.push(profiler::StackedTime{label: String::from("WorldGrid"), time: ggez_timer::time_since_start(ctx)});

        // Draw resources
        self.resource_manager.draw(ctx);
        draw_timings.push(profiler::StackedTime{label: String::from("Resources"), time: ggez_timer::time_since_start(ctx)});

        // Draw obstacles
        self.obstacle_manager.draw(ctx);
        draw_timings.push(profiler::StackedTime{label: String::from("Obstacles"), time: ggez_timer::time_since_start(ctx)});

        // Draw actors
        self.actor_manager.draw(ctx);
        draw_timings.push(profiler::StackedTime{label: String::from("Actors"), time: ggez_timer::time_since_start(ctx)});

        // Draw performance stats
        self.profiler.draw_fps_stats(ctx);
        draw_timings.push(profiler::StackedTime{label: String::from("FPS"), time: ggez_timer::time_since_start(ctx)});

        let res = ggez_gfx::present(ctx);
        draw_timings.push(profiler::StackedTime{label: String::from("Present"), time: ggez_timer::time_since_start(ctx)});

        // Send stacked timings to profiler
        self.profiler.send_stacked_draw_time(start_time, draw_timings).unwrap();
        
        res
    }
}
