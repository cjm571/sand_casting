/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : game_managers/obstacle_manager.rs

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
    element::Elemental,
    hex_directions,
    mechanics::obstacle::Obstacle,
    Plottable,
};

use ggez::{
    Context as GgEzContext,
    graphics as ggez_gfx,
    mint as ggez_mint,
};

use mt_logger::{
    mt_log,
    Level,
};

use crate::{
    game_assets::{
        colors,
        hex_grid_cell::HexGridCell,
    },
    game_managers::DrawableMechanic,
};


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

pub struct ObstacleManager {
    obstacles:      Vec<Obstacle>,
    obstacle_mesh:  ggez_gfx::Mesh,
}

#[derive(Debug)]
pub struct ObstacleError;


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl ObstacleManager {
    /// Generic Constructor - creates an empty instance
    pub fn new(ctx: &mut GgEzContext) -> Self {
        ObstacleManager {
            obstacles:      Vec::new(),
            obstacle_mesh:  ggez_gfx::Mesh::new_line(
                                ctx,
                                &[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                               crate::DEFAULT_LINE_WIDTH,
                               crate::DEFAULT_LINE_COLOR)
                                .unwrap(),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl DrawableMechanic for ObstacleManager {
    type Instance = Obstacle;
    type ErrorType = ObstacleError;

    fn instances(&self) -> &Vec<Self::Instance> {
        &self.obstacles
    }

    fn push_instance(&mut self, instance: Self::Instance) {
        mt_log!(Level::Debug,
            "Adding {} obstacle starting at {} to mesh.",
            String::from(instance.element()),
            instance.origin());

        self.obstacles.push(instance);
    }

    fn mesh(&self) -> &ggez_gfx::Mesh {
        &self.obstacle_mesh
    }

    fn set_mesh(&mut self, mesh: ggez_gfx::Mesh) {
        self.obstacle_mesh = mesh;
    }

    fn add_instance_to_mesh_builder(instance: &Self::Instance,
                                    mesh_builder: &mut ggez_gfx::MeshBuilder,
                                    ggez_ctx: &mut GgEzContext) -> Result<(),Self::ErrorType> {
        // Get all positions for current obstacle instance
        let obstacle_positions = instance.positions();

        // Iterate through current obstacle's positions, adding hexes to the mesh for each
        for (i, obstacle_pos) in obstacle_positions.iter().enumerate() {
            //OPT: *PERFORMANCE* Not a great spot for this conversion logic...
            // Create a HexGridCell object and add it to the mesh builder
            let cur_hex = HexGridCell::new_from_hex_coords(obstacle_pos,crate::HEX_RADIUS_VERTEX, ggez_ctx);
            cur_hex.add_to_mesh(colors::from_element(instance.element()), colors::DARKGREY, mesh_builder);

            // Draw a line over the hex side between the new and previous obstacle cell for all but the first cell
            if i > 0 {
                // Determine direction of hex side that should be overwritten
                let prev_obstacle_pos = obstacle_positions.get(i-1).unwrap();
                let direction = hex_directions::Side::from(obstacle_pos.delta_to(prev_obstacle_pos));

                // Get the vertices for the direction's side                
                let shared_vertex_indices = vec![usize::from(hex_directions::Side::get_adjacent_vertices(direction).0),
                                                 usize::from(hex_directions::Side::get_adjacent_vertices(direction).1)];

                let shared_line = [cur_hex.vertices()[shared_vertex_indices[0]],
                                   cur_hex.vertices()[shared_vertex_indices[1]]];

                mesh_builder.line(&shared_line,crate::DEFAULT_LINE_WIDTH, colors::from_element(instance.element())).unwrap();
            }
        }

        Ok(())
    }
}




