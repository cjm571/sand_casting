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

use cast_iron::hex_direction_provider::*;

use ggez::{
    graphics as ggez_gfx,
    mint as ggez_mint,
};


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
    center:     ggez_mint::Point2<f32>,
    color:      ggez_gfx::Color,
    vertices:   [ggez_mint::Point2<f32>; 6],
}

//TODO: Proper implementation of an error type
pub struct HexGridCellError;

///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////
impl HexGridCell {
    /// Constructor
    pub fn new(center: ggez_mint::Point2<f32>, color: ggez_gfx::Color, size: f32) -> Self {
        // Compute vertices components
        let x_offset = size * (PI/3.0).cos();
        let y_offset = size * (PI/3.0).sin();

        // NOTE: these are graphical coordinates, where (0, 0) is the top-left
        let mut vertices: [ggez_mint::Point2<f32>; 6] = [ggez_mint::Point2{x: 0.0, y: 0.0}; 6];
        vertices[0] = ggez_mint::Point2{ x: center.x + size,       y: center.y};
        vertices[1] = ggez_mint::Point2{ x: center.x + x_offset,   y: center.y - y_offset};
        vertices[2] = ggez_mint::Point2{ x: center.x - x_offset,   y: center.y - y_offset};
        vertices[3] = ggez_mint::Point2{ x: center.x - size,       y: center.y};
        vertices[4] = ggez_mint::Point2{ x: center.x - x_offset,   y: center.y + y_offset};
        vertices[5] = ggez_mint::Point2{ x: center.x + x_offset,   y: center.y + y_offset};

        Self {center, color, vertices}
    }


    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    pub fn center(&self) -> ggez_mint::Point2<f32> {
        self.center
    }

    pub fn color(&self) -> ggez_gfx::Color {
        self.color
    }

    pub fn vertices(&self) -> [ggez_mint::Point2<f32>; 6] {
        self.vertices
    }


    ///////////////////////////////////////////////////////////////////////////
    //  Utility Methods
    ///////////////////////////////////////////////////////////////////////////

    /// Add hexagon to the given mesh builder
    pub fn add_to_mesh(&self, fill_color: ggez_gfx::Color, outline_color: ggez_gfx::Color, mesh_builder: &mut ggez_gfx::MeshBuilder) {
        // Add the filled hexagon
        self.add_hex_fill_to_mesh(fill_color, mesh_builder);

        // Add the outline of the hexagon
        self.add_hex_outline_to_mesh(outline_color, mesh_builder);
    }

    pub fn add_radials_to_mesh(
        &self,
        fill_color: ggez_gfx::Color,
        outline_color: ggez_gfx::Color,
        radius: usize,
        has_gradient: bool,
        resource_mesh_builder: &mut ggez_gfx::MeshBuilder
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
            let direction_provider: HexDirectionProvider<HexVertices> = HexDirectionProvider::new(HexVertices::EAST);
            for (i, vertex) in direction_provider.enumerate() {
                let theta: f32 = vertex.into();
                // Add PI/6 to theta to rotate the standard flat-up hex to point-up
                // This is important as all radial groups of hexes will effectively be large point-up hexes
                let adj_theta = theta + PI/6.0;

                radial_vertices[i].x = origin_centerpoint.x + (::CENTER_TO_SIDE_DIST*2.0*adj_theta.cos());
                radial_vertices[i].y = origin_centerpoint.y - (::CENTER_TO_SIDE_DIST*2.0*adj_theta.sin());

                // Inflate the vertices based on level
                radial_vertices[i].x += (::CENTER_TO_SIDE_DIST*2.0*adj_theta.cos()) * level as f32;
                radial_vertices[i].y -= (::CENTER_TO_SIDE_DIST*2.0*adj_theta.sin()) * level as f32;

                // Create hex cells at each vertex
                let vert_hex = HexGridCell::new(radial_vertices[i], ::DEFAULT_FILL_COLOR, ::GRID_CELL_SIZE);
                vert_hex.add_to_mesh(cur_fill_color, outline_color, resource_mesh_builder);

                // Create interstitial hex(es) if level requires
                for j in 0..level {
                    let inter_hex_theta = adj_theta + 4.0*PI/6.0;

                    let inter_hex_center = ggez_mint::Point2 {
                        x: radial_vertices[i].x + (::CENTER_TO_SIDE_DIST*2.0*inter_hex_theta.cos()) * (j+1) as f32,
                        y: radial_vertices[i].y - (::CENTER_TO_SIDE_DIST*2.0*inter_hex_theta.sin()) * (j+1) as f32
                    };

                    let inter_hex = HexGridCell::new(inter_hex_center, ::DEFAULT_FILL_COLOR, ::GRID_CELL_SIZE);
                    inter_hex.add_to_mesh(cur_fill_color, outline_color, resource_mesh_builder);
                }
            }

            //FEAT: *DESIGN* A logarithmic scale would probably be prettier
            if has_gradient && cur_fill_color.a > MIN_ALPHA_VAL {
                // Transparentize color such that we get to mostly transparent at the furthest level, but not fully transparent
                cur_fill_color.a -= 1.0/radius as f32;
            }
        }
    }


    ///////////////////////////////////////////////////////////////////////////
    //  Helper Functions
    ///////////////////////////////////////////////////////////////////////////

    /// Adds the fill portion of a hex cell to the given Mesh
    fn add_hex_fill_to_mesh(&self, color: ggez_gfx::Color, mesh_builder: &mut ggez_gfx::MeshBuilder) {
        mesh_builder.polygon(ggez_gfx::DrawMode::fill(), &self.vertices, color).unwrap();
    }

    /// Adds the outline portion of a hex cell to the given Mesh
    fn add_hex_outline_to_mesh(&self, color: ggez_gfx::Color, mesh_builder: &mut ggez_gfx::MeshBuilder) {
        mesh_builder.polygon(ggez_gfx::DrawMode::stroke(::DEFAULT_LINE_WIDTH), &self.vertices, color).unwrap();
    }
}
///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////