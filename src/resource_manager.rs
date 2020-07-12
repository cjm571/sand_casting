/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : resource_manager.rs

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
    This module manages all active resources in the game, as well as providing
    utility functions for resource drawing, moving, etc.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use cast_iron::environment::resource::Resource;

use ggez::{
    Context as GgEzContext,
    graphics as ggez_gfx,
    mint as ggez_mint
};

use ::game_assets::{
    colors,
    hex_grid_cell::HexGridCell
};


///////////////////////////////////////////////////////////////////////////////
// Data structures
///////////////////////////////////////////////////////////////////////////////

//TODO: Proper implementation of an error type
#[derive(Debug)]
pub struct ResourceError;

pub struct ResourceManager {
    resources: Vec<Resource>,
    resource_mesh: ggez_gfx::Mesh
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl ResourceManager {
    /// Generic Constructor - creates an empty instance
    pub fn new(ctx: &mut GgEzContext) -> ResourceManager {
        ResourceManager {
            resources: Vec::new(),
            resource_mesh: ggez_gfx::Mesh::new_line(
                ctx,
                &[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                ::DEFAULT_LINE_WIDTH,
                ::DEFAULT_LINE_COLOR)
                .unwrap()
        }
    }
    
    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    pub fn get_resource_mesh(&self) -> &ggez_gfx::Mesh {
        &self.resource_mesh
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Utility Functions
    ///////////////////////////////////////////////////////////////////////////

    pub fn add_resource(&mut self, new_res: Resource, ctx: &mut GgEzContext) -> Result<(), ResourceError>
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
            Err(ResourceError)
        }
    }    

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
            let x_offset = (res_coords.get_x() - res_coords.get_y()) as f32 * (::X_OFFSET * 3.0);
            let y_offset = res_coords.get_z() as f32 * ::Y_OFFSET;
            let res_center = ggez_mint::Point2 {
                x: window_center.x + x_offset,
                y: window_center.y + y_offset
            };

            // Create a HexGridCell object and add it to the mesh builder
            let cur_hex = HexGridCell::new(res_center, ::GRID_CELL_SIZE);
            cur_hex.add_to_mesh(colors::from_resource(&res), &mut resource_mesh_builder);

            // Create radial HexGridCells as necessary
            cur_hex.add_radials_to_mesh(
                colors::from_resource(res),
                res.get_radius(),
                true,
                &mut resource_mesh_builder);
        }

        self.resource_mesh = resource_mesh_builder.build(ctx).unwrap();
    }
}