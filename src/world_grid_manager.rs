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

use std::f32::consts::PI;
use std::collections::HashMap;

use cast_iron::environment::resource::Resource;

use ggez::{
    Context as GgEzContext,
    graphics as ggez_gfx,
    mint as ggez_mint
};

use ::game_assets::{
    colors,
    hexagon::Hexagon
};


///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

const GRID_CELL_SIZE: f32 = 30.0;

// Y_OFFSET = GRID_CELL_SIZE * sin(pi/3) * 2
// Distance from centerpoint of hex to center of a side 
static Y_OFFSET: f32 = GRID_CELL_SIZE * 0.86602540378;

// Y_OFFSET = GRID_CELL_SIZE * cos(pi/3) * 2
// Distance from centerpoint of hex to center of a side 
static X_OFFSET: f32 = GRID_CELL_SIZE * 0.5;


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
    pub static ref HEX_SIDES: HashMap<Direction, f32> = {
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
    pub static ref HEX_VERTICES: HashMap<Direction, f32> = {
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

pub struct WorldGridManager {
    max_radial_distance: u32,       // Maximum value for an axis of the hex grid
    base_grid_mesh: ggez_gfx::Mesh, // Mesh for the base hex grid
    resources: Vec<Resource>,       // Collection of active resources
    resource_mesh: ggez_gfx::Mesh   // Mesh for the resources on the grid
}


#[derive(Debug)]
pub struct WorldGridError;

///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl WorldGridManager {
    /// Returns a new instance of WorldGridManager, with a base grid mesh initialized based on
    /// the GGEZ context's current window dimensions.
    pub fn new(max_radial_distance: u32, ctx: &mut GgEzContext) -> WorldGridManager {
        let mut world_manager = WorldGridManager {
            max_radial_distance: max_radial_distance,
            base_grid_mesh: ggez_gfx::MeshBuilder::new()
                                .line(
                                    &[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                                    1.0,
                                    colors::WHITE
                                )
                                .unwrap()
                                .build(ctx)
                                .unwrap(),
            resources: Vec::new(),
            resource_mesh: ggez_gfx::MeshBuilder::new()
                                .line(
                                    &[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                                    1.0,
                                    colors::WHITE
                                )
                                .unwrap()
                                .build(ctx)
                                .unwrap(),
        };

        // Get window dimensions
        let (window_x, window_y) = ggez_gfx::size(ctx);
        let center = ggez_mint::Point2 {x: window_x / 2.0, y: window_y / 2.0};

        // Create a mesh builder for the base hex grid
        let mut base_grid_mesh_builder = ggez_gfx::MeshBuilder::new();
        world_manager.build_grid(center, &mut base_grid_mesh_builder);

        // Build the base hex grid mesh and draw it
        world_manager.base_grid_mesh = base_grid_mesh_builder.build(ctx).unwrap();

        world_manager
    }    


    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////
     
    pub fn get_grid_size(&self) -> u32 {
        self.max_radial_distance
    }

    pub fn get_base_grid_mesh(&self) -> &ggez_gfx::Mesh {
        &self.base_grid_mesh
    }

    pub fn get_resource_mesh(&self) -> &ggez_gfx::Mesh {
        &self.resource_mesh
    }


    ///////////////////////////////////////////////////////////////////////////
    //  Mutator Methods
    ///////////////////////////////////////////////////////////////////////////

    pub fn add_resource(&mut self, new_res: Resource, ctx: &mut GgEzContext) -> Result<(), WorldGridError>
    {
        // Verify that no resource already exists in the same location
        let mut coords_occupied = false;
        for existing_res in &self.resources {
            if existing_res.get_coords() == new_res.get_coords() {
                coords_occupied = true;
                break;
            }
        }

        // If the new resource's coordinates are unoccupied, add it
        if coords_occupied == false {
            self.resources.push(new_res);

            // Update resource mesh
            self.update_resource_mesh(ctx);
            Ok(())
        }
        else { // Otherwise, return an error
            Err(WorldGridError)
        }
    }    

    //FIXME: Still needs to handle radius of resources
    pub fn update_resource_mesh(&mut self, ctx: &mut GgEzContext) {
        let mut resource_mesh_builder = ggez_gfx::MeshBuilder::new();

        // Get window dimensions
        let (window_x, window_y) = ggez_gfx::size(ctx);
        let window_center = ggez_mint::Point2 {
            x: window_x / 2.0,
            y: window_y / 2.0
        };
        
        // Iterate through resources, adding to mesh builder along the way
        for res in &self.resources {
            let res_coords = res.get_coords();

            // Calculate x, y offsets to determine (x,y) centerpoint from hex grid coords
            let x_offset = (res_coords.get_x() - res_coords.get_y()) as f32 * (X_OFFSET * 3.0);
            let y_offset = res_coords.get_z() as f32 * Y_OFFSET;
            let res_center = ggez_mint::Point2 {
                x: window_center.x + x_offset,
                y: window_center.y + y_offset
            };

            // Create a hexagon object and add it to the mesh builder
            let cur_hex = Hexagon::from(res_center, GRID_CELL_SIZE);
            cur_hex.add_to_mesh(colors::from_element(res.get_kind()), &mut resource_mesh_builder);
        }

        self.resource_mesh = resource_mesh_builder.build(ctx).unwrap();
    }


    ///////////////////////////////////////////////////////////////////////////
    //  Helper Functions
    ///////////////////////////////////////////////////////////////////////////

    /// Builds a baseline hex grid to the graphics window.
    fn build_grid(
        &self,
        center: ggez_mint::Point2<f32>,
        mesh_builder: &mut ggez_gfx::MeshBuilder
    ) {
        // Build GRID_CELL_SIZE-width hexagon sides recursively
        self.recursive_hex_build(colors::WHITE, center, 0, mesh_builder);

        let spoke_color = colors::GREEN;
        // Build spokes recursively in all directions
        for (_dir, theta) in HEX_VERTICES.iter() {
            // Determine origin point for current direction
            let origin = ggez_mint::Point2 {
                x: center.x + (GRID_CELL_SIZE * theta.cos()),
                y: center.y - (GRID_CELL_SIZE * theta.sin())
            };
            self.recursive_spoke_build(spoke_color, origin, *theta, 0, mesh_builder);
        }
    }

    /// Builds a hex grid at the given level using recursive calls radiating out
    /// from the given center.
    fn recursive_hex_build(
        &self,
        color: ggez_gfx::Color,
        center: ggez_mint::Point2<f32>,
        level: u32,
        mesh_builder: &mut ggez_gfx::MeshBuilder
    ) {
        // Final level exit case
        if level == self.max_radial_distance {
            return;
        }

        // HEX_SIZE to be used to correctly translate levels > 0
        static HEX_SIZE: f32 = Y_OFFSET * 2.0;
        
        // Build a parallel line and dispatch a spoke build call at the current level
        // for each intercardinal direction.
        for (_dir, theta) in HEX_SIDES.iter() {
            // Calculate parallel line endpoints
            let mut endpt_x = center.x + GRID_CELL_SIZE * (theta - PI/6.0).cos();
            let mut endpt_y = center.y - GRID_CELL_SIZE * (theta - PI/6.0).sin();
            let mut endpt_a = ggez_mint::Point2 {
                x: endpt_x,
                y: endpt_y
            };

            endpt_x = center.x + GRID_CELL_SIZE * (theta + PI/6.0).cos();
            endpt_y = center.y - GRID_CELL_SIZE * (theta + PI/6.0).sin();
            let mut endpt_b = ggez_mint::Point2 {
                x: endpt_x,
                y: endpt_y
            };

            // Translate lines based on level
            endpt_a.x = endpt_a.x + level as f32 * (HEX_SIZE * theta.cos());
            endpt_a.y = endpt_a.y - level as f32 * (HEX_SIZE * theta.sin());
            endpt_b.x = endpt_b.x + level as f32 * (HEX_SIZE * theta.cos());
            endpt_b.y = endpt_b.y - level as f32 * (HEX_SIZE * theta.sin());

            // Add the line to the GGEZ mesh builder
            match mesh_builder.line(&[endpt_a, endpt_b], 1.0, color) {
                Ok(_mb) => (),
                _       => panic!("Failed to add line to mesh_builder")
            }
        }
        
        // Make the recursive call
        self.recursive_hex_build(color, center, level+1, mesh_builder);
    }

    /// Builds a spoke (i.e. -<) from a point in the given direction.
    /// Recursively spawns two more spoke builds at the endpoint
    fn recursive_spoke_build(
        &self,
        mut color: ggez_gfx::Color,
        origin: ggez_mint::Point2<f32>,
        theta: f32,
        level: u32,
        mesh_builder: &mut ggez_gfx::MeshBuilder
    ) {
        // Final level exit case
        if level == self.max_radial_distance {
            return;
        }

        let mut lines: [[ggez_mint::Point2<f32>; 2]; 3] = [[ggez_mint::Point2 {x: 0.0, y: 0.0}; 2]; 3];
        let mut endpoints: [ggez_mint::Point2<f32>; 3] = [ggez_mint::Point2 {x: 0.0, y: 0.0}; 3];

        // Calculate endpoint of stem
        endpoints[0] = ggez_mint::Point2 {
            x: origin.x + (GRID_CELL_SIZE * theta.cos()),
            y: origin.y - (GRID_CELL_SIZE * theta.sin())
        };
        lines[0] = [origin, endpoints[0]];

        // Calculate branch endpoints
        endpoints[1] = ggez_mint::Point2 {
            x: endpoints[0].x + (GRID_CELL_SIZE * (theta + PI/3.0).cos()),
            y: endpoints[0].y - (GRID_CELL_SIZE * (theta + PI/3.0).sin())
        };
        endpoints[2] = ggez_mint::Point2 {
            x: endpoints[0].x + (GRID_CELL_SIZE * (theta - PI/3.0).cos()),
            y: endpoints[0].y - (GRID_CELL_SIZE * (theta - PI/3.0).sin())
        };
        lines[1] = [endpoints[0], endpoints[1]];
        lines[2] = [endpoints[0], endpoints[2]];

        // Build lines
        for i in 0..=2 {
            match mesh_builder.line(&lines[i], 1.0, color) {
                Ok(_mb) => (),
                _       => panic!("Failed to add line to mesh_builder")
            }
        }

        // Make the recursive calls
        color.g = color.g - 0.1;

        color.r = color.r + 0.1;
        self.recursive_spoke_build(color, endpoints[1], theta, level+1, mesh_builder);
        color.r = color.b + 0.1;
        self.recursive_spoke_build(color, endpoints[2], theta, level+1, mesh_builder);
    }
}
