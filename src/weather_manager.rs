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
        element::{
            Element,
            Elemental,
        },
        weather::{
            Intensity,
            Weather,
        },
    },
    logger,
    ci_log,
};

use ggez::{
    Context as GgEzContext,
    graphics as ggez_gfx,
    mint as ggez_mint,
    timer as ggez_timer,
};

use crate::game_assets::{
    colors,
};


///////////////////////////////////////////////////////////////////////////////
// Named Constants
///////////////////////////////////////////////////////////////////////////////

/// Default line width for the weather HUD outline
const DEFAULT_HUD_OUTLINE_LINE_WIDTH:   f32 = 3.0;
const DEFAULT_HUD_INT_BAR_LINE_WIDTH:   f32 = 5.0;

const DEFAULT_HUD_OUTLINE_LINE_COLOR:   ggez_gfx::Color = colors::MAGENTA;

const DEFAULT_HUD_LEFT_EDGE_POS:        f32 = 800.0;
const DEFAULT_HUD_TOP_EDGE_POS:         f32 = 64.0;
const DEFAULT_HUD_WIDTH:                f32 = 128.0;
const DEFAULT_HUD_HEIGHT:               f32 = DEFAULT_HUD_WIDTH;

const DEFAULT_INT_TEXT_POS:     ggez_mint::Point2<f32> = ggez_mint::Point2 {x: DEFAULT_HUD_LEFT_EDGE_POS,
                                                                            y: DEFAULT_HUD_TOP_EDGE_POS + DEFAULT_HUD_HEIGHT + 5.0};
const DEFAULT_ELEM_TEXT_POS:    ggez_mint::Point2<f32> = ggez_mint::Point2 {x: DEFAULT_HUD_LEFT_EDGE_POS,
                                                                            y: DEFAULT_HUD_TOP_EDGE_POS - 20.0};


///////////////////////////////////////////////////////////////////////////////
// Data structures
///////////////////////////////////////////////////////////////////////////////

pub struct WeatherManager {
    logger:         logger::Instance,
    active_weather: Weather,
    timeout_tick:   usize,
    prev_element:   Element,
    prev_intensity: Intensity,
    hud_elements:   HudElements
}

struct HudElements {
    pub frame_mesh:     ggez_gfx::Mesh,
    pub content_mesh:   ggez_gfx::Mesh,
    pub int_bar_mesh:   ggez_gfx::Mesh,
    pub text_int_str:   String,
    pub text_int_obj:   ggez_gfx::Text,
    pub text_elem_str:  String,
    pub text_elem_obj:  ggez_gfx::Text,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl WeatherManager {
    /// Fully-qualified constructor
    pub fn new(logger_original: &logger::Instance,
               active_weather:  Weather,
               timeout_tick:    usize,
               ggez_ctx:        &mut GgEzContext) -> Self {
        // Clone the logger instance so this module has its own sender to use
        let logger_clone = logger_original.clone();

        WeatherManager {
            logger:         logger_clone,
            active_weather, 
            timeout_tick,   
            prev_element:   Element::default(),
            prev_intensity: Intensity::default(),
            hud_elements:   HudElements::default(ggez_ctx),
        }
    }

    /// Default constructor
    pub fn default(logger_original: &logger::Instance, ggez_ctx: &mut GgEzContext) -> Self {
        // Clone the logger instance so this module has its own sender to use
        let logger_clone = logger_original.clone();

        WeatherManager {
            logger:         logger_clone,
            active_weather: Weather::default(),
            timeout_tick:   usize::default(),
            prev_element:   Element::default(),
            prev_intensity: Intensity::default(),
            hud_elements:   HudElements::default(ggez_ctx),
        }
    }


    /* Utility Methods */

    /// Updates the active weather if the current effect has timed out
    pub fn update_weather(&mut self, ggez_ctx: &mut GgEzContext) {
        let cur_tick = ggez_timer::ticks(ggez_ctx);

        // If current weather has timed out, randomly generate a new weather pattern
        if cur_tick >= self.timeout_tick {
            self.active_weather = Weather::rand_starting_at(cur_tick);
            ci_log!(self.logger, logger::FilterLevel::Info,
                "Tick {:>8}: Weather changed to Elem: {:?}, Duration: {}",
                cur_tick,
                self.active_weather.element(),
                self.active_weather.duration()
            );

            // Set the timeout to the duration of the new weather pattern
            self.timeout_tick = cur_tick + self.active_weather.duration();
        }

        // Check for change in intensity or element
        let cur_intensity = self.active_weather.intensity(cur_tick);
        if self.prev_intensity != cur_intensity || self.prev_element != self.active_weather.element(){
            // Update HUD content with new alpha level
            let mut content_color = colors::from_element(self.active_weather.element());
            content_color.a = cur_intensity.to_alpha();
            self.hud_elements.update_content_mesh(content_color, ggez_ctx);

            // Update intensity text
            self.hud_elements.update_text_elements(self.active_weather.element(), cur_intensity);

            // Update previous-state values
            self.prev_element   = self.active_weather.element();
            self.prev_intensity = self.active_weather.intensity(cur_tick);
        }

        // Update intensity bar
        self.hud_elements.update_int_bar_mesh(self.active_weather.intensity_exact(cur_tick), ggez_ctx);
    }

    pub fn draw(&self, ggez_ctx: &mut GgEzContext) {
        // Draw content mesh behind frame mesh
        ggez_gfx::draw(ggez_ctx, &self.hud_elements.content_mesh, ggez_gfx::DrawParam::default()).unwrap();
        ggez_gfx::draw(ggez_ctx, &self.hud_elements.frame_mesh, ggez_gfx::DrawParam::default()).unwrap();

        // Draw status text
        ggez_gfx::draw(ggez_ctx, &self.hud_elements.text_int_obj, (DEFAULT_INT_TEXT_POS, 0.0, colors::GREEN)).unwrap();
        ggez_gfx::draw(ggez_ctx, &self.hud_elements.text_elem_obj, (DEFAULT_ELEM_TEXT_POS, 0.0, colors::GREEN)).unwrap();

        // Draw intensity bar
        ggez_gfx::draw(ggez_ctx, &self.hud_elements.int_bar_mesh, ggez_gfx::DrawParam::default()).unwrap();
    }
}


impl HudElements {
    /// Default constructor
    fn default(ggez_ctx: &mut GgEzContext) -> Self {
        let mut hud_elements = Self {
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
            text_elem_str:  String::default(),
            text_elem_obj:  ggez_gfx::Text::default(),
            text_int_str:   String::default(),
            text_int_obj:   ggez_gfx::Text::default(),
        };

        // Do first 'updates' of the meshes so we have valid meshes from first use
        hud_elements.update_frame_mesh(ggez_ctx);
        hud_elements.update_content_mesh(colors::TRANSPARENT, ggez_ctx);
        hud_elements.update_int_bar_mesh(i32::default(), ggez_ctx);
        hud_elements.update_text_elements(Element::default(), Intensity::default());

        hud_elements
    }


    ///
    // Helper Methods
    ///

    //FEAT: Use like, a cool picture frame or something instead
    /// Updates the frame mesh for the HUD (just a square outline for now)
    fn update_frame_mesh(&mut self, ggez_ctx: &mut GgEzContext) {
        // Build a square in the top-right of the screen to hold the weather info
        let outline_rect = ggez_gfx::Rect::new(DEFAULT_HUD_LEFT_EDGE_POS,
                                               DEFAULT_HUD_TOP_EDGE_POS,
                                               DEFAULT_HUD_WIDTH,
                                               DEFAULT_HUD_HEIGHT);

        self.frame_mesh = ggez_gfx::Mesh::new_rectangle(ggez_ctx,
                                                        ggez_gfx::DrawMode::stroke(DEFAULT_HUD_OUTLINE_LINE_WIDTH),
                                                        outline_rect,
                                                        DEFAULT_HUD_OUTLINE_LINE_COLOR).unwrap();
    }

    //FEAT: Add graphics representing each element
    /// Updates the mesh for the HUD color (just a filled square for now)
    fn update_content_mesh(&mut self, color: ggez_gfx::Color, ggez_ctx: &mut GgEzContext) {
        // Build a square in the top-right of the screen to hold the weather info
        let color_rect = ggez_gfx::Rect::new(DEFAULT_HUD_LEFT_EDGE_POS,
                                             DEFAULT_HUD_TOP_EDGE_POS,
                                             DEFAULT_HUD_WIDTH,
                                             DEFAULT_HUD_HEIGHT);

        self.content_mesh = ggez_gfx::Mesh::new_rectangle(ggez_ctx,
                                                          ggez_gfx::DrawMode::fill(),
                                                          color_rect,
                                                          color).unwrap();
    }
    
    /// Updates the mesh for the HUD intensity bar
    fn update_int_bar_mesh(&mut self, exact_intensity: i32, ggez_ctx: &mut GgEzContext) {
        // Need a mesh builder with a dummy line to avoid an empty mesh
        let mut int_bar_mesh_builder = ggez_gfx::MeshBuilder::new();
        let dummy_line = [ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 1.0, y: 1.0}];
        int_bar_mesh_builder.line(&dummy_line, 1.0, colors::TRANSPARENT).unwrap();

        // Build a square in the top-right of the screen to hold the weather info
        let int_bar_line = [ggez_mint::Point2 {x: DEFAULT_HUD_LEFT_EDGE_POS - 5.0,
                                               y: DEFAULT_HUD_TOP_EDGE_POS + DEFAULT_HUD_HEIGHT},
                            ggez_mint::Point2 {x: DEFAULT_HUD_LEFT_EDGE_POS - 5.0,
                                               y: DEFAULT_HUD_TOP_EDGE_POS + DEFAULT_HUD_HEIGHT - (exact_intensity/2) as f32}];

        self.int_bar_mesh = int_bar_mesh_builder.line(&int_bar_line,
                                                     DEFAULT_HUD_INT_BAR_LINE_WIDTH,
                                                     colors::GREEN)
                                                     .unwrap()
                                                     .build(ggez_ctx)
                                                     .unwrap();
    }

    /// Updates text elements of the HUD
    fn update_text_elements(&mut self, element: Element, intensity: Intensity) {
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
