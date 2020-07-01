/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : hexagon.rs

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
    This module defines a hexagon for use in Piston graphics draw calls.

    Based on the graphics::types::Polygon type.

Changelog:
    CJ McAllister   16 Nov 2018     File created

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
use std::f32;

use ggez::{
    graphics as ggez_gfx,
    mint as ggez_mint
};

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

// Point array starts with the eastern-most point, and continues counter-clockwise.
#[derive(Debug, Copy, Clone)]
pub struct Hexagon {
    vertices:   [ggez_mint::Point2<f32>; 6],
    pub color:  [f32; 4]
}

///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////
impl Hexagon {
    // Default Constructor
    pub fn new() -> Hexagon {
        // Build an array of default Points
        let vertices: [ggez_mint::Point2<f32>; 6] = [ggez_mint::Point2{x: 0.0, y: 0.0}; 6];

        Hexagon {
            vertices:   vertices,
            color:      [1.0, 1.0, 1.0, 1.0]
        }
    }

    // Specific Constructor
    pub fn from(center: ggez_mint::Point2<f32>, size: f32) -> Hexagon {
        // Compute vertices components
        let x_offset = size * (f32::consts::PI/3.0).cos();
        let y_offset = size * (f32::consts::PI/3.0).sin();

        // NOTE: these are graphical coordinates, where (0, 0) is the top-left
        let mut vertices: [ggez_mint::Point2<f32>; 6] = [ggez_mint::Point2{x: 0.0, y: 0.0}; 6];
        vertices[0] = ggez_mint::Point2{ x: center.x + size,       y: center.y};
        vertices[1] = ggez_mint::Point2{ x: center.x + x_offset,   y: center.y - y_offset};
        vertices[2] = ggez_mint::Point2{ x: center.x - x_offset,   y: center.y - y_offset};
        vertices[3] = ggez_mint::Point2{ x: center.x - size,       y: center.y};
        vertices[4] = ggez_mint::Point2{ x: center.x - x_offset,   y: center.y + y_offset};
        vertices[5] = ggez_mint::Point2{ x: center.x + x_offset,   y: center.y + y_offset};

        Hexagon {
            vertices:   vertices,
            color:      [1.0, 1.0, 1.0, 1.0] // White
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Utility Methods
    ///////////////////////////////////////////////////////////////////////////

    // Draw hexagon to the given graphics context
    pub fn draw<G>(&self) {
        // Build up an array of "lines" for use in the line() draw function
        let mut lines: [[f32; 4]; 6] = [[0.0; 4]; 6];

        for i in 0 ..=4 {
            lines[i] = [self.vertices[i].x, self.vertices[i].y,
                        self.vertices[i+1].x, self.vertices[i+1].y];
        }
        lines[5] = [self.vertices[5].x, self.vertices[5].y,
                    self.vertices[0].x, self.vertices[0].y];

        // Draw all lines of hexagon
        for i in 0 ..=5 {
            //FIXME: ggez draws
        }
    }

}
///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////