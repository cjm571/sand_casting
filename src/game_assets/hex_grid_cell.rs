/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : hex_grid_cell.rs

Copyright (C) 2018 CJ McAllister
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
    This module defines a hexagonal grid cell for use in GGEZ graphics draw calls.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::f32::consts::PI;

use cast_iron::{
    context::Context as CastIronContext,
    coords,
    hex_directions
};

use ggez::{
    Context as GgEzContext,
    graphics as ggez_gfx,
    mint as ggez_mint,
};

use crate::game_assets::colors;


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

/// Minimum value for alpha reduction on radials
const MIN_ALPHA_VAL: f32 = 0.1;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

// Point array starts with the eastern-most point, and continues counter-clockwise.
#[derive(Debug, Copy, Clone)]
pub struct HexGridCell {
    center:     ggez_mint::Point2<f32>,         // Pixel-coords centerpoint
    vertices:   [ggez_mint::Point2<f32>; 6],    // Pixel-coords of vertices
    highlight:  bool,                           // Indicates if cell should be highlighted in world grid
}

pub struct HexGridCellError;

///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////
impl HexGridCell {
    /// Pixel-coords-based constructor
    pub fn new_from_pixel_coords(center: ggez_mint::Point2<f32>, radius: f32) -> Self {
        // Compute vertices components
        let x_offset = radius * (PI/3.0).cos();
        let y_offset = radius * (PI/3.0).sin();

        // NOTE: these are graphical coordinates, where (0, 0) is the top-left
        let mut vertices: [ggez_mint::Point2<f32>; 6] = [ggez_mint::Point2{x: 0.0, y: 0.0}; 6];
        vertices[0] = ggez_mint::Point2{ x: center.x + radius,     y: center.y};
        vertices[1] = ggez_mint::Point2{ x: center.x + x_offset,   y: center.y - y_offset};
        vertices[2] = ggez_mint::Point2{ x: center.x - x_offset,   y: center.y - y_offset};
        vertices[3] = ggez_mint::Point2{ x: center.x - radius,     y: center.y};
        vertices[4] = ggez_mint::Point2{ x: center.x - x_offset,   y: center.y + y_offset};
        vertices[5] = ggez_mint::Point2{ x: center.x + x_offset,   y: center.y + y_offset};

        Self {center, vertices, highlight: false}
    }

    /// Hex-coords-based constructor
    pub fn new_from_hex_coords(center: &coords::Position, radius: f32, ggez_ctx: &GgEzContext) -> Self {
        // Convert to pixel coords and use the pixel coords constructor
        let pixel_center = Self::hex_to_pixel_coords(center, ggez_ctx);
        
        Self::new_from_pixel_coords(pixel_center, radius)
    }


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */

    pub fn center(&self) -> ggez_mint::Point2<f32> {
        self.center
    }

    pub fn vertices(&self) -> [ggez_mint::Point2<f32>; 6] {
        self.vertices
    }

    pub fn highlighted(&self) -> bool {
        self.highlight
    }

    
    /*  *  *  *  *  *  *  *\
     *  Mutator Methods   *
    \*  *  *  *  *  *  *  */

    pub fn set_highlight(&mut self, highlight: bool) {
        self.highlight = highlight;
    }

    pub fn toggle_highlight(&mut self) {
        self.highlight = !self.highlight;
    }


    /*  *  *  *  *  *  *  *\
     *  Utility Methods   *
    \*  *  *  *  *  *  *  */

    //OPT: *DESIGN* Fill/outline color should be intrinsic components of the HexGridCell object, not passed-in parameters
    /// Add hexagon to the given mesh builder
    pub fn add_to_mesh(&self, fill_color: ggez_gfx::Color, outline_color: ggez_gfx::Color, mesh_builder: &mut ggez_gfx::MeshBuilder) {
        // Add the filled hexagon
        self.add_hex_fill_to_mesh(fill_color, mesh_builder);

        // Add the outline of the hexagon
        self.add_hex_outline_to_mesh(outline_color, mesh_builder);

        // Add highlight if necessary
        if self.highlight {
            self.add_highlight_to_mesh(mesh_builder);
        }
    }

    //OPT: *DESIGN* This should be a static helper function
    pub fn add_radials_to_mesh(
        &self,
        fill_color: ggez_gfx::Color,
        outline_color: ggez_gfx::Color,
        radius: usize,
        has_gradient: bool,
        mesh_builder: &mut ggez_gfx::MeshBuilder
    ) {
        // In order to reliably construct radiating hexes:
        // 1. Take the origin hex cell
        // 2. Rotate its vertices by PI/6
        // 3. Inflate the hex based on current radial level
        // 4. Construct the appropriate number of hexes to fit along the lines between those vertices

        // Copy original fill color to allow for transparentization across levels
        let mut cur_fill_color = fill_color;

        // Get origin hex vertices
        let origin_centerpoint = self.center();
        let mut radial_vertices = [ggez_mint::Point2{x: 0.0, y: 0.0}; 6];

        for level in 0..radius {
            // Create an iterator starting at the East vertex and going COUNTER-CLOCKWISE as required by GGEZ draw calls
            let direction_provider: hex_directions::Provider<hex_directions::Vertex> = hex_directions::Provider::new(hex_directions::Vertex::EAST);
            for (i, vertex) in direction_provider.enumerate() {
                let theta: f32 = vertex.into();
                // Add PI/6 to theta to rotate the standard flat-up hex to point-up
                // This is important as all radial groups of hexes will effectively be large point-up hexes
                let adj_theta = theta + PI/6.0;

                radial_vertices[i].x = origin_centerpoint.x + (::HEX_RADIUS_SIDE*2.0*adj_theta.cos());
                radial_vertices[i].y = origin_centerpoint.y - (::HEX_RADIUS_SIDE*2.0*adj_theta.sin());

                // Inflate the vertices based on level
                radial_vertices[i].x += (::HEX_RADIUS_SIDE*2.0*adj_theta.cos()) * level as f32;
                radial_vertices[i].y -= (::HEX_RADIUS_SIDE*2.0*adj_theta.sin()) * level as f32;

                // Create hex cells at each vertex
                let vert_hex = HexGridCell::new_from_pixel_coords(radial_vertices[i], ::HEX_RADIUS_VERTEX);
                vert_hex.add_to_mesh(cur_fill_color, outline_color, mesh_builder);

                // Create interstitial hex(es) if level requires
                for j in 0..level {
                    let inter_hex_theta = adj_theta + 4.0*PI/6.0;

                    let inter_hex_center = ggez_mint::Point2 {
                        x: radial_vertices[i].x + (::HEX_RADIUS_SIDE*2.0*inter_hex_theta.cos()) * (j+1) as f32,
                        y: radial_vertices[i].y - (::HEX_RADIUS_SIDE*2.0*inter_hex_theta.sin()) * (j+1) as f32
                    };

                    let inter_hex = HexGridCell::new_from_pixel_coords(inter_hex_center, ::HEX_RADIUS_VERTEX);
                    inter_hex.add_to_mesh(cur_fill_color, outline_color, mesh_builder);
                }
            }

            if has_gradient && cur_fill_color.a > MIN_ALPHA_VAL {
                // Transparentize color such that we get to mostly transparent at the furthest level, but not fully transparent
                cur_fill_color.a -= 1.0/radius as f32;
            }
        }
    }


    /*  *  *  *  *  *  *  *\
     *  Utility Functions *
    \*  *  *  *  *  *  *  */

    //OPT: *DESIGN* Is this the right place for these?
    pub fn pixel_to_hex_coords(cart_coords: ggez_mint::Point2<f32>, ci_ctx: &CastIronContext, ggez_ctx: &GgEzContext) -> Result<coords::Position, coords::CoordsError> {
        // Get pixel centerpoint of game window
        let (window_x, window_y) = ggez_gfx::size(ggez_ctx);
        let window_center = ggez_mint::Point2 {
            x: window_x / 2.0,
            y: window_y / 2.0
        };

        // Calculate pixel deltas from center
        let x_delta = cart_coords.x - window_center.x;
        let y_delta = cart_coords.y - window_center.y;

        // Calculate the delta along the X and Z planes, and calculate Y based on the results
        let x = (2.0/3.0 * x_delta) / ::HEX_RADIUS_VERTEX;
        let z = (-1.0/3.0 * x_delta + (3.0_f32).sqrt()/3.0 * y_delta) / ::HEX_RADIUS_VERTEX;
        let y = -x - z;

        // Compose into a position, and return
        Self::hex_round(x, y, z, ci_ctx)
    }

    pub fn hex_to_pixel_coords(hex_pos: &coords::Position, ggez_ctx: &GgEzContext) -> ggez_mint::Point2<f32> {
        // Get pixel centerpoint of game window
        let (window_x, window_y) = ggez_gfx::size(ggez_ctx);
        let window_center = ggez_mint::Point2 {
            x: window_x / 2.0,
            y: window_y / 2.0
        };

        // Calculate x, y offsets
        let x_offset = hex_pos.x() as f32 * ::HEX_RADIUS_VERTEX * 3.0 / 2.0;
        let y_offset = (-hex_pos.y() as f32 * f32::from(hex_directions::Side::NORTHWEST).sin() * (::HEX_RADIUS_SIDE * 2.0)) +
                       (-hex_pos.z() as f32 * f32::from(hex_directions::Side::SOUTHWEST).sin() * (::HEX_RADIUS_SIDE * 2.0));

        ggez_mint::Point2 {
            x: window_center.x + x_offset,
            y: window_center.y + y_offset,
        }
    }



    /*  *  *  *  *  *  *  *\
     *  Helper Methods    *
    \*  *  *  *  *  *  *  */

    /// Adds the fill portion of a hex cell to the given Mesh
    fn add_hex_fill_to_mesh(&self, color: ggez_gfx::Color, mesh_builder: &mut ggez_gfx::MeshBuilder) {
        mesh_builder.polygon(ggez_gfx::DrawMode::fill(), &self.vertices, color).unwrap();
    }

    /// Adds the outline portion of a hex cell to the given Mesh
    fn add_hex_outline_to_mesh(&self, color: ggez_gfx::Color, mesh_builder: &mut ggez_gfx::MeshBuilder) {
        mesh_builder.polygon(ggez_gfx::DrawMode::stroke(::DEFAULT_LINE_WIDTH), &self.vertices, color).unwrap();
    }

    fn add_highlight_to_mesh(&self, mesh_builder: &mut ggez_gfx::MeshBuilder) {
        mesh_builder.polygon(ggez_gfx::DrawMode::fill(), &self.vertices, colors::HILITE_STD).unwrap();
    }


    /*  *  *  *  *  *  *  *\
     *  Helper Functions  *
    \*  *  *  *  *  *  *  */

    fn hex_round(x: f32, y: f32, z: f32, ci_ctx: &CastIronContext) -> Result<coords::Position, coords::CoordsError> {
        // Round all floating coords to nearest integer
        let rounded_x = x.round() as i32;
        let rounded_y = y.round() as i32;
        let rounded_z = z.round() as i32;

        // NOTE: Rounding may have broken the x + y + z == 0 constraint
        // To combat this, we'll reset the coordinate component with the largest delta from the nearest integer
        // to what is required by the constraint.
        let delta_x = (x - rounded_x as f32).abs();
        let delta_y = (y - rounded_y as f32).abs();
        let delta_z = (z - rounded_z as f32).abs();

        if delta_x > delta_y && delta_x > delta_z {
            // X has largest delta, recalculate it
            let recalc_x = -rounded_y - rounded_z;

            coords::Position::new(recalc_x, rounded_y, rounded_z, ci_ctx)
        }
        else if delta_y > delta_z {
            // Y has largest delta, recalculate it
            let recalc_y = -rounded_x - rounded_z;

            coords::Position::new(rounded_x, recalc_y, rounded_z, ci_ctx)
        }
        else {
            // Z has largest delta, recalculate it
            let recalc_z = -rounded_x - rounded_y;

            coords::Position::new(rounded_x, rounded_y, recalc_z, ci_ctx)
        }
    }
}
