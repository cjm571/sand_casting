/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : game_managers/weather_manager.rs

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
    context::Context as CastIronContext,
    element::{
        Element,
        Elemental,
    },
    logger,
    mechanics::weather,
    Randomizable,
    ci_log,
};

use ggez::{
    Context as GgEzContext,
    graphics as ggez_gfx,
    mint as ggez_mint,
    timer as ggez_timer,
};

use crate::{
    game_assets::colors,
    profiler,
};


///////////////////////////////////////////////////////////////////////////////
// Named Constants
///////////////////////////////////////////////////////////////////////////////

// Default line features for the weather HUD
const HUD_OUTLINE_LINE_WIDTH:   f32 = 3.0;
const HUD_INT_BAR_LINE_WIDTH:   f32 = 5.0;
const HUD_OUTLINE_LINE_COLOR:   ggez_gfx::Color = colors::MAGENTA;

// Offset of text from HUD frame
const HUD_TEXT_OFFSET:          f32 = 5.0;


///////////////////////////////////////////////////////////////////////////////
// Data structures
///////////////////////////////////////////////////////////////////////////////

pub struct WeatherManager {
    logger:         logger::Instance,
    profiler:       profiler::Instance,
    active_weather: weather::Event,
    timeout_ms:     u128,
    prev_intensity: weather::Intensity,
    hud_elements:   HudElements
}

struct HudElements {
    pub frame_pos:      ggez_mint::Point2<f32>,
    pub frame_size:     f32,
    pub frame_mesh:     ggez_gfx::Mesh,
    pub content_mesh:   ggez_gfx::Mesh,
    pub int_bar_mesh:   ggez_gfx::Mesh,
    pub text_elem_pos:  ggez_mint::Point2<f32>,
    pub text_elem_str:  String,
    pub text_elem_obj:  ggez_gfx::Text,
    pub text_int_pos:   ggez_mint::Point2<f32>,
    pub text_int_str:   String,
    pub text_int_obj:   ggez_gfx::Text,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl WeatherManager {
    /// Fully-qualified constructor
    pub fn new(logger_original:     &logger::Instance,
               profiler_original:   &profiler::Instance,
               active_weather:      weather::Event,
               timeout_ms:          u128,
               ci_ctx:              &CastIronContext, 
               ggez_ctx:            &mut GgEzContext) -> Self {
        // Clone the logger, profiler instances for use by this module
        let logger_clone = logger_original.clone();
        let profiler_clone = profiler_original.clone();

        WeatherManager {
            logger:         logger_clone,
            profiler:       profiler_clone,
            active_weather, 
            timeout_ms,
            prev_intensity: weather::Intensity::default(),
            hud_elements:   HudElements::default(ci_ctx, ggez_ctx),
        }
    }

    /// Default staticructor
    pub fn default(logger_original: &logger::Instance,
                   profiler_original:   &profiler::Instance,
                   ci_ctx: &CastIronContext,
                   ggez_ctx: &mut GgEzContext) -> Self {
        // Clone the logger, profiler instances for use by this module
        let logger_clone = logger_original.clone();
        let profiler_clone = profiler_original.clone();

        WeatherManager {
            logger:         logger_clone,
            profiler:       profiler_clone,
            active_weather: weather::Event::default(),
            timeout_ms:     u128::default(),
            prev_intensity: weather::Intensity::default(),
            hud_elements:   HudElements::default(ci_ctx, ggez_ctx),
        }
    }


    /*  *  *  *  *  *  *  *
     *  Utility Methods   *
     *  *  *  *  *  *  *  */

    /// Updates the active weather if the current effect has timed out
    pub fn update_weather(&mut self, ci_ctx: &CastIronContext, ggez_ctx: &mut GgEzContext) {
        //OPT: *PERFORMANCE* Would it be faster to use 2 usizes for seconds and milli/nanoseconds?
        let elapsed_time = ggez_timer::time_since_start(ggez_ctx);
        let mut new_weather_generated = false;

        // If current weather has timed out, randomly generate a new weather pattern
        if elapsed_time.as_millis() >= self.timeout_ms {
            // Send WEATHER_GEN event marker to profiler
            self.profiler.mark_event(String::from("WEATHER_GEN_START"), ggez_ctx).unwrap();

            self.active_weather = weather::Event::rand(ci_ctx).starting_at(elapsed_time);

            // Log weather change
            ci_log!(self.logger, logger::FilterLevel::Info,
                "GameTime: {:.3}s: Weather changed to Elem: {:?}, Duration: {:.3}s",
                elapsed_time.as_secs_f64(),
                self.active_weather.element(),
                self.active_weather.duration().as_secs_f64()
            );

            // Set the timeout to the duration of the new weather pattern
            self.timeout_ms = elapsed_time.as_millis() + self.active_weather.duration().as_millis();

            new_weather_generated = true;
            
            // Send WEATHER_GEN event marker to profiler
            self.profiler.mark_event(String::from("WEATHER_GEN_STOP"), ggez_ctx).unwrap();
        }

        // Check for change in weather event
        let cur_intensity = self.active_weather.intensity(elapsed_time.as_secs_f64());
        if self.prev_intensity != cur_intensity || new_weather_generated {
            // Send WEATHER_GEN event marker to profiler
            self.profiler.mark_event(String::from("WEATHER_CHANGE_START"), ggez_ctx).unwrap();

            // Update HUD content with new alpha level
            let mut content_color = colors::from_element(self.active_weather.element());
            content_color.a = cur_intensity.to_alpha();
            self.hud_elements.update_content_mesh(content_color, ggez_ctx);

            // Update intensity text
            self.hud_elements.update_text_elements(self.active_weather.element(), cur_intensity);

            // Update previous-state values
            self.prev_intensity = self.active_weather.intensity(elapsed_time.as_secs_f64());

            // Send WEATHER_GEN event marker to profiler
            self.profiler.mark_event(String::from("WEATHER_CHANGE_STOP"), ggez_ctx).unwrap();
        }

        // Update intensity bar
        self.hud_elements.update_int_bar_mesh(self.active_weather.intensity_exact(elapsed_time.as_secs_f64()), ci_ctx, ggez_ctx);
    }

    pub fn draw(&self, ggez_ctx: &mut GgEzContext) {
        // Draw HUD elements
        self.hud_elements.draw(ggez_ctx);
    }
}


impl HudElements {
    /// Default staticructor
    fn default(ci_ctx: &CastIronContext, ggez_ctx: &mut GgEzContext) -> Self {
        // Grab window dimensions so we can place the HUD appropriately
        let (window_x, window_y) = ggez_gfx::size(ggez_ctx);

        let calc_frame_pos = ggez_mint::Point2{ x: 3.0 * window_x / 4.0,
                                                y: window_y / 16.0};
        let calc_frame_size = window_x / 10.0;

        let mut hud_elements = Self {
            frame_pos:      calc_frame_pos,
            frame_size:     calc_frame_size,
            frame_mesh:     ggez_gfx::MeshBuilder::new()
                                    .line(&[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                                          ::DEFAULT_LINE_WIDTH,
                                          ::DEFAULT_LINE_COLOR)
                                    .unwrap()
                                    .build(ggez_ctx)
                                    .unwrap(),
            content_mesh:   ggez_gfx::MeshBuilder::new()
                                    .line(&[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                                          ::DEFAULT_LINE_WIDTH,
                                          ::DEFAULT_LINE_COLOR)
                                    .unwrap()
                                    .build(ggez_ctx)
                                    .unwrap(),
            int_bar_mesh:   ggez_gfx::MeshBuilder::new()
                                    .line(&[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                                            ::DEFAULT_LINE_WIDTH,
                                            ::DEFAULT_LINE_COLOR)
                                    .unwrap()
                                    .build(ggez_ctx)
                                    .unwrap(),
            text_elem_pos:  ggez_mint::Point2{ x: calc_frame_pos.x,
                                               y: calc_frame_pos.y - ::DEFAULT_TEXT_SIZE - HUD_TEXT_OFFSET},
            text_elem_str:  String::default(),
            text_elem_obj:  ggez_gfx::Text::default(),
            text_int_pos:   ggez_mint::Point2{ x: calc_frame_pos.x,
                                               y: calc_frame_pos.y + calc_frame_size + HUD_TEXT_OFFSET},
            text_int_str:   String::default(),
            text_int_obj:   ggez_gfx::Text::default(),
        };

        // Do first 'updates' of the meshes so we have valid meshes from first use
        hud_elements.update_frame_mesh(ggez_ctx);
        hud_elements.update_content_mesh(colors::TRANSPARENT, ggez_ctx);
        hud_elements.update_int_bar_mesh(f64::default(), ci_ctx, ggez_ctx);
        hud_elements.update_text_elements(Element::default(), weather::Intensity::default());

        hud_elements
    }


    /*  *  *  *  *  *  *  *
     *  Utility Methods   *
     *  *  *  *  *  *  *  */

    pub fn draw(&self, ggez_ctx: &mut GgEzContext) {
        // Draw status text
        ggez_gfx::draw(ggez_ctx, &self.text_int_obj, (self.text_int_pos, 0.0, colors::GREEN)).unwrap();
        ggez_gfx::draw(ggez_ctx, &self.text_elem_obj, (self.text_elem_pos, 0.0, colors::GREEN)).unwrap();
    
        // WORKAROUND - avoid flickering on intel graphics
        ggez::graphics::apply_transformations(ggez_ctx).unwrap();

        // Draw content mesh behind frame mesh
        ggez_gfx::draw(ggez_ctx, &self.content_mesh, ggez_gfx::DrawParam::default()).unwrap();
        ggez_gfx::draw(ggez_ctx, &self.frame_mesh, ggez_gfx::DrawParam::default()).unwrap();

        // Draw intensity bar
        ggez_gfx::draw(ggez_ctx, &self.int_bar_mesh, ggez_gfx::DrawParam::default()).unwrap();
    }

    //FEAT: Use like, a cool picture frame or something instead
    /// Updates the frame mesh for the HUD (just a square outline for now)
    fn update_frame_mesh(&mut self, ggez_ctx: &mut GgEzContext) {
        // Build a square in the top-right of the screen to hold the weather info
        let outline_rect = ggez_gfx::Rect::new(self.frame_pos.x,
                                               self.frame_pos.y,
                                               self.frame_size,
                                               self.frame_size);

        self.frame_mesh = ggez_gfx::Mesh::new_rectangle(ggez_ctx,
                                                        ggez_gfx::DrawMode::stroke(HUD_OUTLINE_LINE_WIDTH),
                                                        outline_rect,
                                                        HUD_OUTLINE_LINE_COLOR).unwrap();
    }

    //FEAT: Add graphics representing each element
    /// Updates the mesh for the HUD color (just a filled square for now)
    fn update_content_mesh(&mut self, color: ggez_gfx::Color, ggez_ctx: &mut GgEzContext) {
        // Build a square in the top-right of the screen to hold the weather info
        let color_rect = ggez_gfx::Rect::new(self.frame_pos.x,
                                             self.frame_pos.y,
                                             self.frame_size,
                                             self.frame_size);

        self.content_mesh = ggez_gfx::Mesh::new_rectangle(ggez_ctx,
                                                          ggez_gfx::DrawMode::fill(),
                                                          color_rect,
                                                          color).unwrap();
    }
    
    /// Updates the mesh for the HUD intensity bar
    fn update_int_bar_mesh(&mut self, exact_intensity: f64, ci_ctx: &CastIronContext, ggez_ctx: &mut GgEzContext) {
        // Need a mesh builder with a dummy line to avoid an empty mesh
        let mut int_bar_mesh_builder = ggez_gfx::MeshBuilder::new();
        let dummy_line = [ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 1.0, y: 1.0}];
        int_bar_mesh_builder.line(&dummy_line, 1.0, colors::TRANSPARENT).unwrap();

        let drawable_intensity: f32 = (exact_intensity as f32 / ci_ctx.max_weather_intensity() as f32) * self.frame_size;

        // Build a square in the top-right of the screen to hold the weather info
        let int_bar_line = [ggez_mint::Point2 {x: self.frame_pos.x - 5.0,
                                               y: self.frame_pos.y + self.frame_size},
                            ggez_mint::Point2 {x: self.frame_pos.x - 5.0,
                                               y: self.frame_pos.y + self.frame_size - drawable_intensity}];

        self.int_bar_mesh = int_bar_mesh_builder.line(&int_bar_line,
                                                     HUD_INT_BAR_LINE_WIDTH,
                                                     colors::GREEN)
                                                     .unwrap()
                                                     .build(ggez_ctx)
                                                     .unwrap();
    }

    /// Updates text elements of the HUD
    fn update_text_elements(&mut self, element: Element, intensity: weather::Intensity) {
        // Update element text
        self.text_elem_str = String::from(element);
        self.text_elem_obj = ggez_gfx::Text::new((self.text_elem_str.as_str(),
                                                  ggez_gfx::Font::default(),
                                                  ::DEFAULT_TEXT_SIZE));

        // Update intensity text
        self.text_int_str = String::from(intensity);
        self.text_int_obj = ggez_gfx::Text::new((self.text_int_str.as_str(),
                                                 ggez_gfx::Font::default(),
                                                 ::DEFAULT_TEXT_SIZE));
    }
}
