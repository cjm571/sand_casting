/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : obstacle_manager.rs

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
    This module manages all active obstacles in the game, as well as providing
    Utility Methods for obstacle drawing, moving, etc.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use cast_iron::{
    context::Context as CastIronContext,
    environment::{
        element::Elemental,
        obstacle::Obstacle
    },
    hex_direction_provider::*,
    logger::{
        LoggerInstance,
        LogLevel
    },
    ci_log
};

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
// Constants
///////////////////////////////////////////////////////////////////////////////

const MAX_RAND_OBSTACLE_ATTEMPTS: usize = 10;


///////////////////////////////////////////////////////////////////////////////
// Data structures
///////////////////////////////////////////////////////////////////////////////

//TODO: Proper implementation of an error type
#[derive(Debug)]
pub struct ObstacleError;

pub struct ObstacleManager {
    logger:         LoggerInstance,
    obstacles:      Vec<Obstacle>,
    obstacle_mesh:  ggez_gfx::Mesh,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl ObstacleManager {
    /// Generic Constructor - creates an empty instance
    pub fn new(logger: LoggerInstance, ctx: &mut GgEzContext) -> Self {
        ObstacleManager {
            logger:         logger,
            obstacles:      Vec::new(),
            obstacle_mesh:  ggez_gfx::Mesh::new_line(
                                ctx,
                                &[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                                ::DEFAULT_LINE_WIDTH,
                                ::DEFAULT_LINE_COLOR)
                                .unwrap(),
        }
    }


    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    pub fn get_obstacle_mesh(&self) -> &ggez_gfx::Mesh {
        &self.obstacle_mesh
    }
    

    //OPT: *STYLE* These are very similar to the Resource Manager's utility methods...
    ///////////////////////////////////////////////////////////////////////////
    //  Utility Methods
    ///////////////////////////////////////////////////////////////////////////
 
    pub fn update_obstacle_mesh(&mut self, ggez_ctx: &mut GgEzContext) {
        ci_log!(self.logger, LogLevel::DEBUG, "Updating obstacle mesh...");

        // Do not attempt to update the mesh if we have no obstacles
        if self.obstacles.len() == 0 {
            ci_log!(self.logger, LogLevel::DEBUG, "No obstacles! Abandoning update.");
            return;
        }

        let mut obstacle_mesh_builder = ggez_gfx::MeshBuilder::new();

        // Get window dimensions
        let (window_x, window_y) = ggez_gfx::size(ggez_ctx);
        let window_center = ggez_mint::Point2 {
            x: window_x / 2.0,
            y: window_y / 2.0
        };

        // Iterate through obstacles, adding to mesh builder along the way
        for obstacle in &self.obstacles {
            ci_log!(self.logger, LogLevel::DEBUG, "Updating with {:?}", obstacle);
            let all_obstacle_coords = obstacle.get_all_coords();

            // Iterate through current obstacle's coords, adding hexes to the mesh for each
            for obstacle_coords in all_obstacle_coords {
                ci_log!(self.logger, LogLevel::DEBUG, "Adding hex at {:?}", obstacle_coords);
                //OPT: *PERFORMANCE* Not a great spot for this conversion logic...
                // Calculate x, y offsets to determine (x,y) centerpoint from hex grid coords
                let x_offset = obstacle_coords.get_x() as f32 * (::CENTER_TO_VERTEX_DIST * 3.0);
                let y_offset = -1.0 * (obstacle_coords.get_y() as f32 * f32::from(HexSides::NORTHWEST).sin() * (::CENTER_TO_SIDE_DIST * 2.0)) + 
                               -1.0 * (obstacle_coords.get_z() as f32 * f32::from(HexSides::SOUTHWEST).sin() * (::CENTER_TO_SIDE_DIST * 2.0));

                let obstacle_center = ggez_mint::Point2 {
                    x: window_center.x + x_offset,
                    y: window_center.y + y_offset
                };

                // Create a HexGridCell object and add it to the mesh builder
                let cur_hex = HexGridCell::new(obstacle_center, ::GRID_CELL_SIZE);
                cur_hex.add_to_mesh(colors::from_element(obstacle.get_element()), colors::RED, &mut obstacle_mesh_builder);

                //FIXME: Need to match the outline shared with previous hex to element color to create a continuous obstacle
            }

        }

        self.obstacle_mesh = obstacle_mesh_builder.build(ggez_ctx).unwrap();
    }

    pub fn add_obstacle(&mut self, new_obstacle: Obstacle, ggez_ctx: &mut GgEzContext) -> Result<(), ObstacleError> {
        //TODO: Should this function also sanity check against a CastIron context?
        
        // OPT: *PERFORMANCE* oof, this is probably super slow
        // Verify that no obstacle already exists along the new obstacle's path
        let mut coords_occupied = false;
        for existing_obstacle in &self.obstacles {
            for existing_obstacle_coords in existing_obstacle.get_all_coords() {
                if new_obstacle.get_all_coords().contains(existing_obstacle_coords) {
                    coords_occupied = true;
                    break;
                }
            }
        }

        // If the new obstacle's coordinates are unoccupied, add it
        if coords_occupied == false {
            self.obstacles.push(new_obstacle);

            // Update obstacle mesh
            self.update_obstacle_mesh(ggez_ctx);
            ci_log!(self.logger, LogLevel::DEBUG, "Added obstacle: {:?}", self.obstacles.last().unwrap());

            Ok(())
        }
        else { // Otherwise, return an error
            Err(ObstacleError)
        }
    }

    pub fn add_rand_obstacle(&mut self, ci_ctx: &CastIronContext, ggez_ctx: &mut GgEzContext) -> Result<(), ObstacleError> {
        // Create a random obstacle and attempt to add them until we succeed (or fail too many times)
        let mut attempts = 0;
        while attempts < MAX_RAND_OBSTACLE_ATTEMPTS {
            let rand_obstacle = Obstacle::rand(ci_ctx);
            match self.add_obstacle(rand_obstacle, ggez_ctx) {
                Ok(())              => {
                    break;                  // Successfully added obstacle, break loop
                },
                Err(ObstacleError)  => ()   // Failed to add obstacle, continue
            }

            attempts = attempts + 1;
        }

        // If attempts maxed out - return error, otherwise Ok()
        if attempts == MAX_RAND_OBSTACLE_ATTEMPTS {
            Err(ObstacleError)
        }
        else
        {
            Ok(())
        }
    }
}
