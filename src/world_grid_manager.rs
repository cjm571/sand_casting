/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : environment\world_grid_manager.rs

Copyright (C) 2017 CJ McAllister
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
    This module rovides functions to determine interactions between various objects
    in the world grid.
    
Changelog:

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use graphics::types::*;
use graphics::line;
use graphics::Graphics;

use std::f64::consts::PI;
use std::collections::HashMap;

use super::shape::point::Point;
use super::*;


///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

const GRID_CELL_SIZE: f64 = 50.0;

// Y_OFFSET = GRID_CELL_SIZE * sin(pi/3) * 2
// Distance from centerpoint of hex to center of a side 
static Y_OFFSET: f64 = GRID_CELL_SIZE * 0.86602540378;


///////////////////////////////////////////////////////////////////////////////
// Data structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Hash)]
pub enum Direction {
    EAST,
    NORTHEAST,
    NORTH,
    NORTHWEST,
    WEST,
    SOUTHWEST,
    SOUTH,
    SOUTHEAST
}
// Equivalence comparison
impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        self == other
    }
}
impl Eq for Direction {}

lazy_static! {
    pub static ref HEX_SIDES: HashMap<Direction, f64> = {
        let mut m = HashMap::new();

        m.insert(Direction::NORTHEAST, PI/6.0);
        m.insert(Direction::NORTH,     PI/2.0);
        m.insert(Direction::NORTHWEST, 5.0*PI/6.0);
        m.insert(Direction::SOUTHWEST, 7.0*PI/6.0);
        m.insert(Direction::SOUTH,     3.0*PI/2.0);
        m.insert(Direction::SOUTHEAST, 11.0*PI/6.0);

        m
    };
}

lazy_static! {
    pub static ref HEX_VERTICES: HashMap<Direction, f64> = {
        let mut m = HashMap::new();

        m.insert(Direction::EAST,       0.0);
        m.insert(Direction::NORTHEAST,  PI/3.0);
        m.insert(Direction::NORTHWEST,  2.0*PI/3.0);
        m.insert(Direction::WEST,       PI);
        m.insert(Direction::SOUTHWEST,  4.0*PI/3.0);
        m.insert(Direction::SOUTHEAST,  5.0*PI/3.0);

        m
    };
}

#[derive(Copy, Clone)]
pub struct WorldGridManager {
    pub max_radial_distance: u32, // Maximum value for an axis of the hex grid
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl WorldGridManager {
    pub fn new(max_radial_distance: u32) -> WorldGridManager {
        WorldGridManager {
            max_radial_distance: max_radial_distance,
        }
    }    

    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////
     
    pub fn get_grid_size(self) -> u32 {
        self.max_radial_distance
    }

    /// Draws a baseline hex grid to the graphics window.
    pub fn draw_grid<G>(self, center: Point, transform: Matrix2d, g: &mut G)
    where G: Graphics {
        // Draw GRID_CELL_SIZE-width hexagon sides recursively
        self.recursive_hex_draw(WHITE, center, 0, transform, g);

        // Draw spokes recursively in all directions
        for (_dir, theta) in HEX_VERTICES.iter() {
            // Determine origin point for current direction
            let origin = Point::from(
                center.x + (GRID_CELL_SIZE * theta.cos()),
                center.y - (GRID_CELL_SIZE * theta.sin())
            );
            self.recursive_spoke_draw(WHITE, origin, *theta, 0, transform, g);
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Helper Functions
    ///////////////////////////////////////////////////////////////////////////

    /// Draws a hex grid at the given level using recursive calls radiating out
    /// from the given center.
    fn recursive_hex_draw<G>(self, color: Color, center: Point, level: u32, transform: Matrix2d, g: &mut G)
    where G: Graphics {
        // Final level exit case
        if level == self.max_radial_distance {
            return;
        }

        // HEX_SIE to be used to correctly translate levels > 0
        static HEX_SIZE: f64 = Y_OFFSET * 2.0;
        
        // Draw a parallel line and dispatch a spoke draw call at the current level
        // for each intercardinal direction.
        for (_dir, theta) in HEX_SIDES.iter() {
            // Calculate parallel line endpoints
            let mut x = center.x + GRID_CELL_SIZE * (theta - PI/6.0).cos();
            let mut y = center.y - GRID_CELL_SIZE * (theta - PI/6.0).sin();
            let mut endpt_a: Point = Point::from(x, y);

            x = center.x + GRID_CELL_SIZE * (theta + PI/6.0).cos();
            y = center.y - GRID_CELL_SIZE * (theta + PI/6.0).sin();
            let mut endpt_b: Point = Point::from(x, y);

            // Translate lines based on level
            endpt_a.x = endpt_a.x + level as f64 * (HEX_SIZE * theta.cos());
            endpt_a.y = endpt_a.y - level as f64 * (HEX_SIZE * theta.sin());
            endpt_b.x = endpt_b.x + level as f64 * (HEX_SIZE * theta.cos());
            endpt_b.y = endpt_b.y - level as f64 * (HEX_SIZE * theta.sin());

            // Draw the line
            line(color, 0.5, [endpt_a.x, endpt_a.y, endpt_b.x, endpt_b.y], transform, g);
        }
        
        // Make the recursive call
        self.recursive_hex_draw(color, center, level+1, transform, g);
    }

    /// Draws a spoke (i.e. -<) from a point in the given direction.
    /// Recursively spawns two more spoke draws at the endpoint
    fn recursive_spoke_draw<G>(self, color: Color, origin: Point, theta: f64, level: u32, transform: Matrix2d, g: &mut G)
    where G: Graphics {
        // Final level exit case
        if level == self.max_radial_distance {
            return;
        }

        let mut lines: [[f64; 4]; 3] = [[0.0; 4]; 3];
        let mut endpoints: [Point; 3] = [Point::new(); 3];

        // Calculate endpoint of stem
        endpoints[0] = Point::from(
            origin.x + (GRID_CELL_SIZE * theta.cos()),
            origin.y - (GRID_CELL_SIZE * theta.sin())
        );
        lines[0] = [origin.x, origin.y,
                    endpoints[0].x, endpoints[0].y];

        // Calculate branch endpoints
        endpoints[1] = Point::from(
            endpoints[0].x + (GRID_CELL_SIZE * (theta + PI/3.0).cos()),
            endpoints[0].y - (GRID_CELL_SIZE * (theta + PI/3.0).sin())
        );
        endpoints[2] = Point::from(
            endpoints[0].x + (GRID_CELL_SIZE * (theta - PI/3.0).cos()),
            endpoints[0].y - (GRID_CELL_SIZE * (theta - PI/3.0).sin())
        );
        lines[1] = [endpoints[0].x, endpoints[0].y,
                    endpoints[1].x, endpoints[1].y];
        lines[2] = [endpoints[0].x, endpoints[0].y,
                    endpoints[2].x, endpoints[2].y];

        // Draw lines
        for i in 0..=2 {
            line (color, 0.5, lines[i], transform, g);
        }

        // Make the recursive calls
        self.recursive_spoke_draw(color, endpoints[1], theta, level+1, transform, g);
        self.recursive_spoke_draw(color, endpoints[2], theta, level+1, transform, g);
    }
}
