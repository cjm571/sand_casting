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

use graphics::Graphics;
use graphics::line;
use graphics::types::*;

use std::f64;

use super::point::Point;

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

// Point array starts with the eastern-most point, and continues counter-clockwise.
#[derive(Debug, Copy, Clone)]
pub struct Hexagon {
    vertices:   [Point; 6],
    pub color:  Color
}

///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////
impl Hexagon {
    // Default Constructor
    pub fn new() -> Hexagon {
        // Build an array of default Points
        let vertices: [Point; 6] = [Point::new(); 6];

        Hexagon {
            vertices:   vertices,
            color:      [1.0, 1.0, 1.0, 1.0] // White
        }
    }

    // Specific Constructor
    pub fn from(center: Point, size: f64) -> Hexagon {
        // Compute vertices components
        let x_offset = size * (f64::consts::PI/3.0).cos();
        let y_offset = size * (f64::consts::PI/3.0).sin();

        let mut vertices: [Point; 6] = [Point::new(); 6];

        // NOTE: these are graphical coordinates, where (0, 0) is the top-left
        vertices[0] = Point::from(center.x + size, center.y);
        vertices[1] = Point::from(center.x + x_offset, center.y - y_offset);
        vertices[2] = Point::from(center.x - x_offset, center.y - y_offset);
        vertices[3] = Point::from(center.x - size, center.y);
        vertices[4] = Point::from(center.x - x_offset, center.y + y_offset);
        vertices[5] = Point::from(center.x + x_offset, center.y + y_offset);

        Hexagon {
            vertices:   vertices,
            color:      [1.0, 1.0, 1.0, 1.0] // White
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Utility Methods
    ///////////////////////////////////////////////////////////////////////////

    // Draw hexagon to the given graphics context
    pub fn draw<G>(&self, transform: Matrix2d, g: &mut G)
    where G: Graphics {
        // Build up an array of "lines" for use in the line() draw function
        let mut lines: [[f64; 4]; 6] = [[0.0; 4]; 6];

        for i in 0 ..=4 {
            lines[i] = [self.vertices[i].x, self.vertices[i].y,
                        self.vertices[i+1].x, self.vertices[i+1].y];
        }
        lines[5] = [self.vertices[5].x, self.vertices[5].y,
                    self.vertices[0].x, self.vertices[0].y];

        // Draw all lines of hexagon
        for i in 0 ..=5 {
            line(self.color, 0.5, lines[i], transform, g);
        }
    }

}
///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////