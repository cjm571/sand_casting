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

Changelog:
    CJ McAllister   16 Nov 2018     File created
    CJ McAllister   08 Jul 2020     Renamed hexagon.rs -> hex_grid_cell.rs
                                    Removed mentions of old n busted Piston engine

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::f32::consts::PI;

use cast_iron::hex_direction_provider::*;

use ggez::{
    graphics as ggez_gfx,
    mint as ggez_mint
};


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

///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////
impl HexGridCell {
    /// Constructor
    pub fn new(center: ggez_mint::Point2<f32>, size: f32) -> Self {
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

        HexGridCell {
            center:     center,
            color:      ::DEFAULT_FILL_COLOR,
            vertices:   vertices,
        }
    }


    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////
    
    pub fn get_center(&self) -> ggez_mint::Point2<f32> {
        self.center
    }

    pub fn get_color(&self) -> ggez_gfx::Color {
        self.color
    }

    pub fn get_vertices(&self) -> [ggez_mint::Point2<f32>; 6] {
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
        let origin_centerpoint = self.get_center();
        let mut radial_vertices = [ggez_mint::Point2{x: 0.0, y: 0.0}; 6];

        for level in 0..radius {
            // Create an iterator starting at the East vertex and going COUNTER-CLOCKWISE as required by GGEZ draw calls
            let mut i = 0;
            let direction_provider: HexDirectionProvider<HexVertices> = HexDirectionProvider::new(HexVertices::EAST);
            for vertex in direction_provider {
                let theta: f32 = vertex.into();
                // Add PI/6 to theta to rotate the standard flat-up hex to point-up
                // This is important as all radial groups of hexes will effectively be large point-up hexes
                let adj_theta = theta + PI/6.0;

                radial_vertices[i].x = origin_centerpoint.x + (::CENTER_TO_SIDE_DIST*2.0*adj_theta.cos());
                radial_vertices[i].y = origin_centerpoint.y - (::CENTER_TO_SIDE_DIST*2.0*adj_theta.sin());

                // Inflate the vertices based on level
                radial_vertices[i].x = radial_vertices[i].x + (::CENTER_TO_SIDE_DIST*2.0*adj_theta.cos()) * level as f32;
                radial_vertices[i].y = radial_vertices[i].y - (::CENTER_TO_SIDE_DIST*2.0*adj_theta.sin()) * level as f32;

                // Create hex cells at each vertex
                let vert_hex = HexGridCell::new(radial_vertices[i], ::GRID_CELL_SIZE);
                vert_hex.add_to_mesh(cur_fill_color, outline_color, resource_mesh_builder);

                // Create interstitial hex(es) if level requires
                for j in 0..level {
                    let inter_hex_theta = adj_theta + 4.0*PI/6.0;
                    
                    let inter_hex_center = ggez_mint::Point2 {
                        x: radial_vertices[i].x + (::CENTER_TO_SIDE_DIST*2.0*inter_hex_theta.cos()) * (j+1) as f32,
                        y: radial_vertices[i].y - (::CENTER_TO_SIDE_DIST*2.0*inter_hex_theta.sin()) * (j+1) as f32
                    };

                    let inter_hex = HexGridCell::new(inter_hex_center, ::GRID_CELL_SIZE);
                    inter_hex.add_to_mesh(cur_fill_color, outline_color, resource_mesh_builder);
                }

                i = i + 1;
            }

            //OPT: A logarithmic scale would probably be prettier
            //FIXME: Need named const for the minimum alpha value
            if has_gradient == true && cur_fill_color.a > 0.2 {
                // Transparentize color such that we get to mostly transparent at the furthest level, but not fully transparent
                cur_fill_color.a = cur_fill_color.a - (1.0/(radius) as f32);
            }
        }
    }


    ///////////////////////////////////////////////////////////////////////////
    //  Helper Functions
    ///////////////////////////////////////////////////////////////////////////
    
    //OPT: Probably should return Result instead of panicking
    /// Adds the fill portion of a hex cell to the given Mesh
    fn add_hex_fill_to_mesh(&self, color: ggez_gfx::Color, mesh_builder: &mut ggez_gfx::MeshBuilder) {
        match mesh_builder.polygon(ggez_gfx::DrawMode::fill(), &self.vertices, color) {
            Ok(_mb) => (),
            _       => panic!("Failed to add filled hexagon to mesh_builder")
        }
    }

    //OPT: Probably should return Result instead of panicking
    /// Adds the outline portion of a hex cell to the given Mesh
    fn add_hex_outline_to_mesh(&self, color: ggez_gfx::Color, mesh_builder: &mut ggez_gfx::MeshBuilder) {
        // Build up an array of lines for use in the outline
        let mut lines: [[ggez_mint::Point2<f32>; 2]; 6] = [[ggez_mint::Point2 {x: 0.0, y: 0.0}; 2]; 6];

        for i in 0 ..=4 {
            lines[i] = [ggez_mint::Point2 {x: self.vertices[i].x,   y: self.vertices[i].y},
                        ggez_mint::Point2 {x: self.vertices[i+1].x, y: self.vertices[i+1].y}];
        }
        lines[5] = [ggez_mint::Point2 {x: self.vertices[5].x, y: self.vertices[5].y},
                    ggez_mint::Point2 {x: self.vertices[0].x, y: self.vertices[0].y}];

        // Add the outline of hexagon
        for i in 0 ..=5 {
            match mesh_builder.line(&lines[i], 1.0, color) {
                Ok(_mb) => (),
                _       => panic!("Failed to add line to mesh_builder")
            }
        }
    }
}
///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////