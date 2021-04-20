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

use std::{
    error::Error,
    fmt,
};

use cast_iron::{
    context::Context as CastIronContext, 
    logger,
    ci_log,
};

use dd_statechart::{
    StateChart,
    StateChartError,
    event::Event,
};

use ggez::{
    Context as GgEzContext,
    GameResult as GgEzGameResult,
    event as ggez_event,
    graphics as ggez_gfx,
    input::mouse as ggez_mouse,
    input::keyboard as ggez_kb,
    mint as ggez_mint,
    timer as ggez_timer,
};

use crate::{
    game_assets::{
        colors,
        hex_grid_cell::HexGridCell,
    },
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
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

//FIXME: These probably should be relative to window size
// Position of debug info text in window
const DEBUG_POS_STATE: ggez_mint::Point2<f32> = ggez_mint::Point2 {x: 0.0, y: 800.0};


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

//TODO: Rename and refactor this - very likely does not need to keep clones of the logger and profiler
/// Primary Game Struct
pub struct SandCastingGameState {
    initialized:        bool,               // Flag indicating if game has been initialized
    debug_display:      bool,               // Flag indicating if debug info should be displayed
    ci_ctx:             CastIronContext,    // CastIron engine context
    logger:             logger::Instance,   // Instance of CastIron Logger
    profiler:           profiler::Instance, // Instance of SandCasting performance profiler
    actor_manager:      ActorManager,       // Actor Manager instance
    obstacle_manager:   ObstacleManager,    // Obstacle Manager instance
    resource_manager:   ResourceManager,    // Resource Manager instance
    statechart:         StateChart,         // StateChart covering all game states
    weather_manager:    WeatherManager,     // Weather Manager instance
    world_grid_manager: WorldGridManager,   // World Grid Manager instance
}

#[derive(Debug, PartialEq)]
pub enum GameStateError {
    // Wrappers
    StateChartError(StateChartError),
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

/// Constructor
impl SandCastingGameState {
    pub fn new(
        logger_original: &logger::Instance,
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
            debug_display:      false,
            ci_ctx:             ctx_clone,
            logger:             logger_clone,
            profiler:           profiler_clone,
            actor_manager:      ActorManager::new(logger_original, ggez_ctx),
            obstacle_manager:   ObstacleManager::new(logger_original, ggez_ctx),
            resource_manager:   ResourceManager::new(logger_original, ggez_ctx),
            statechart:         StateChart::from("./res/default.scxml").unwrap(), //FIXME: Hardcoded for now, make this a global const or something
            weather_manager:    WeatherManager::default(logger_original, profiler_original, ci_ctx, ggez_ctx),
            world_grid_manager: WorldGridManager::new(
                                    logger_original,
                                    ::DEFAULT_GRID_RADIUS,
                                    ci_ctx,
                                    ggez_ctx),
        }
    }


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */

    //TODO: This does not need nea mut self
    pub fn initialized(&mut self) -> bool {
        self.initialized
    }

    //TODO: These should not give out mutable references
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

    pub fn active_state_ids(&self) -> Vec<&str> {
        self.statechart.active_state_ids()
    }


    /*  *  *  *  *  *  *  *\
     *  Utility Methods   *
    \*  *  *  *  *  *  *  */

    pub fn process_event(&mut self, event: &Event) -> Result<(), GameStateError> {
        // Pass the event to the StateChart
        self.statechart.process_external_event(event).map_err(GameStateError::StateChartError)
    }


    /*  *  *  *  *  *  *  *\
     *   Helper Methods   *
    \*  *  *  *  *  *  *  */

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

    fn draw_debug_info(&self, ggez_ctx: &mut GgEzContext) {
        // Draw active State(s)
        let state_str = format!("Active State(s): {:?}", self.statechart.active_state_ids());
        let state_display = ggez_gfx::Text::new((state_str, ggez_gfx::Font::default(), ::DEFAULT_TEXT_SIZE));
        ggez_gfx::draw(ggez_ctx, &state_display, (DEBUG_POS_STATE, 0.0, colors::YELLOW)).unwrap(); //FIXME: NOOOOOO UNWRAP
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

/*  *  *  *  *  *  *  *  *  *\
 *   SandCastingGameState   *
\*  *  *  *  *  *  *  *  *  */

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

        if self.debug_display {
            // Draw performance stats
            self.profiler.draw_fps_stats(ctx);
            draw_timings.push(profiler::StackedTime{label: String::from("FPS"), time: ggez_timer::time_since_start(ctx)});

            // Draw Debug info
            self.draw_debug_info(ctx);
            draw_timings.push(profiler::StackedTime{label: String::from("Debug Info"), time: ggez_timer::time_since_start(ctx)});
        }

        let res = ggez_gfx::present(ctx);
        draw_timings.push(profiler::StackedTime{label: String::from("Present"), time: ggez_timer::time_since_start(ctx)});

        // Send stacked timings to profiler
        self.profiler.send_stacked_draw_time(start_time, draw_timings).unwrap();
        
        res
    }

    fn mouse_button_down_event(&mut self, ggez_ctx: &mut GgEzContext, button: ggez_mouse::MouseButton, x: f32, y: f32) {
        // Pack up event coordinates
        let event_coords = ggez_mint::Point2 {x, y};

        // Handle each button as appropriate
        match button {
            ggez_mouse::MouseButton::Left => {
                // Determine which hex the mouse event occurred in
                if let Ok(event_hex_pos) = HexGridCell::pixel_to_hex_coords(event_coords, &self.ci_ctx, ggez_ctx) {
                    ci_log!(self.logger, logger::FilterLevel::Debug, "Event ({:?}) occurred at position: {}", button, event_hex_pos);

                    self.world_grid_manager.toggle_cell_highlight(&event_hex_pos, ggez_ctx).unwrap();
                }
                else {
                    ci_log!(self.logger, logger::FilterLevel::Debug, "Event ({:?}) occurred outside hex grid at pixel coords ({}, {})", button, event_coords.x, event_coords.y);
                }
            },
            _ => {
                ci_log!(self.logger, logger::FilterLevel::Warning, "Mouse Event ({:?}) unimplemented!", button);
            }
        }
    }

    fn key_down_event(&mut self, _ggez_ctx: &mut GgEzContext, keycode: ggez_kb::KeyCode, keymods: ggez_kb::KeyMods, repeat: bool) {
        // Ignore repeat inputs (for now)
        if repeat {
            return;
        }
        
        // Otherwise, check the Mod + Key tuple and handle accordingly
        match (keymods, keycode) {
            // Toggle debug display
            (ggez_kb::KeyMods::NONE, ggez_kb::KeyCode::D) => {
                if self.debug_display {
                    self.debug_display = false;
                    ci_log!(self.logger, logger::FilterLevel::Debug, "Debug display disabled");
                }
                else {
                    self.debug_display = true;
                    ci_log!(self.logger, logger::FilterLevel::Debug, "Debug display enabled");
                }
            },
            _ => {
                ci_log!(self.logger, logger::FilterLevel::Warning, "Keyboard Event ({:?} + {:?}) unimplemented!", keymods, keycode);
            }
        }
    }
}


/*  *  *  *  *  *  *  *  *  *\
 *      GameStateError      *
\*  *  *  *  *  *  *  *  *  */

impl Error for GameStateError {}

impl fmt::Display for GameStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::StateChartError(sc_err) => {
                write!(f, "StateChartError '{}' encountered", sc_err)
            },
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{
        cast_iron::{
            context::Context as CastIronContext,
        },
        dd_statechart::event::Event,
        game_state::SandCastingGameState,
        ggez::ContextBuilder as GgEzContextBuilder,
        logger,
        profiler,
    };


    type TestResult = Result<(), Box<dyn Error>>;


    #[test]
    fn statechart_test() -> TestResult {
        let logger = logger::Instance::new(logger::FilterLevel::Warning as u8, logger::OutputType::Console);
        let profiler = profiler::Instance::disabled();
        let ci_ctx = CastIronContext::default();
        let (mut ggez_ctx, mut _event_loop) = GgEzContextBuilder::new("test", "CJ McAllister").build()?;

        let mut game_state = SandCastingGameState::new(&logger, &profiler, &ci_ctx, &mut ggez_ctx);

        // Initial State should be 'idle'
        assert_eq!(
            game_state.active_state_ids(),
            vec!["idle"],
        );

        // Send a combat trigger and verify that the Active State indicates combat has begun
        game_state.process_event(&Event::from("combat.enter")?)?;
        assert_eq!(
            game_state.active_state_ids(),
            vec!["combat"],
        );
        
        Ok(())
    }
}