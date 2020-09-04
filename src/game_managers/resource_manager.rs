/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : game_managers/resource_manager.rs

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
    Utility Methods for resource drawing, moving, etc.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use cast_iron::{
    context::Context as CastIronContext,
    environment::resource::Resource,
    hex_directions,
    logger,
    ci_log,
};

use ggez::{
    Context as GgEzContext,
    graphics as ggez_gfx,
    mint as ggez_mint,
};

use crate::{
    game_assets::{
    colors,
    hex_grid_cell::HexGridCell,
    },
    game_managers::DrawableMechanic,
};


///////////////////////////////////////////////////////////////////////////////
// Constants
///////////////////////////////////////////////////////////////////////////////

const MAX_RAND_RESOURCE_ATTEMPTS: usize = 10;


///////////////////////////////////////////////////////////////////////////////
// Data structures
///////////////////////////////////////////////////////////////////////////////

//TODO: Proper implementation of an error type
#[derive(Debug)]
pub struct ResourceError;

pub struct ResourceManager {
    logger:         logger::Instance,
    resources:      Vec<Resource>,
    resource_mesh:  ggez_gfx::Mesh,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl ResourceManager {
    /// Generic Constructor - creates an empty instance
    pub fn new(logger_original: &logger::Instance, ctx: &mut GgEzContext) -> Self {
        // Clone the logger instance so this module has its own sender to use
        let logger_clone = logger_original.clone();

        ResourceManager {
            logger:         logger_clone,
            resources:      Vec::new(),
            resource_mesh:  ggez_gfx::Mesh::new_line(
                            ctx,
                            &[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                            ::DEFAULT_LINE_WIDTH,
                            ::DEFAULT_LINE_COLOR)
                            .unwrap(),
        }
    }

    ///
    //  Utility Methods
    ///

    //OPT: *DESIGN* Should probably not have radial reach across (most) obstacles
    pub fn add_resource(&mut self, new_res: Resource, ggez_ctx: &mut GgEzContext) -> Result<(), ResourceError> {
        // Verify that no resource already exists in the same location
        let mut coords_occupied = false;
        for existing_res in &self.resources {
            if existing_res.coords() == new_res.coords() {
                coords_occupied = true;
                break;
            }
        }

        // If the new resource's coordinates are unoccupied, add it
        if !coords_occupied {
            self.resources.push(new_res);

            // Update resource mesh
            self.update_mesh(ggez_ctx);
            ci_log!(self.logger, logger::FilterLevel::Debug, "Added resource: {:?}", self.resources.last().unwrap());

            Ok(())
        }
        else { // Otherwise, return an error
            Err(ResourceError)
        }
    }

    pub fn add_rand_resource(&mut self, ci_ctx: &CastIronContext, ggez_ctx: &mut GgEzContext) -> Result<(), ResourceError> {
        // Create a random resource and attempt to add them until we succeed (or fail too many times)
        let mut attempts = 0;
        while attempts < MAX_RAND_RESOURCE_ATTEMPTS {
            let rand_resource = Resource::rand(ci_ctx);
            match self.add_resource(rand_resource, ggez_ctx) {
                Ok(())              => {
                    break;                  // Successfully added resource, break loop
                },
                Err(ResourceError)  => ()   // Failed to add resource, continue
            }

            attempts += 1;
        }

        // If attempts maxed out - return error, otherwise Ok()
        if attempts == MAX_RAND_RESOURCE_ATTEMPTS {
            Err(ResourceError)
        } else {
            Ok(())
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl DrawableMechanic for ResourceManager {
    type Instance = Resource;
    type ErrorType = ResourceError;

    fn instances(&self) -> &Vec<Self::Instance> {
        &self.resources
    }

    fn mesh(&self) -> &ggez_gfx::Mesh {
        &self.resource_mesh
    }

    fn set_mesh(&mut self, mesh: ggez_gfx::Mesh) {
        self.resource_mesh = mesh;
    }

    fn add_instance_to_mesh(instance: &Self::Instance,
                            mesh_builder: &mut ggez_gfx::MeshBuilder,
                            ggez_ctx: &mut GgEzContext) -> Result<(), Self::ErrorType> {
        // Get all coords for current resource instance
        let res_coords = instance.coords();

        //OPT: *PERFORMANCE* Do this in advance and pass in
        // Get window dimensions
        let (window_x, window_y) = ggez_gfx::size(ggez_ctx);
        let window_center = ggez_mint::Point2 {
            x: window_x / 2.0,
            y: window_y / 2.0
        };

        //OPT: *PERFORMANCE* Not a great spot for this conversion logic...
        // Calculate x, y offsets to determine (x,y) centerpoint from hex grid coords
        let x_offset = res_coords.x() as f32 * (::CENTER_TO_VERTEX_DIST * 3.0);
        let y_offset = (-res_coords.y() as f32 * f32::from(hex_directions::Side::NORTHWEST).sin() * (::CENTER_TO_SIDE_DIST * 2.0)) +
                        (-res_coords.z() as f32 * f32::from(hex_directions::Side::SOUTHWEST).sin() * (::CENTER_TO_SIDE_DIST * 2.0));

        let res_center = ggez_mint::Point2 {
            x: window_center.x + x_offset,
            y: window_center.y + y_offset
        };

        // Create a HexGridCell object and add it to the mesh builder
        let cur_hex = HexGridCell::new(res_center, ::DEFAULT_FILL_COLOR, ::GRID_CELL_SIZE);
        cur_hex.add_to_mesh(colors::from_resource(instance), colors::WHITE, mesh_builder);

        // Create radial HexGridCells as necessary
        cur_hex.add_radials_to_mesh(
            colors::from_resource(instance),
            colors::WHITE,
            instance.radius(),
            true,
            mesh_builder);

        Ok(())
    }
}