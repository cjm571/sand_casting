/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : game_managers/world_grid_manager.rs

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
    This module provides functions to determine interactions between various objects
    in the world grid.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::collections::HashMap;

use cast_iron::{
    context::Context as CastIronContext,
    coords,
    hex_directions,
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

use crate::game_assets::{
    colors,
    hex_grid_cell::HexGridCell,
};


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

/// Number of additional hex cells per level in a hex grid.
const NUM_ADDITIONAL_CELLS_PER_LEVEL: usize = 6;

/// Starting direction of a new hex ring
const NEW_RING_START_DIRECTION: hex_directions::Side    = hex_directions::Side::NorthEast;

/// First intra-ring direction in new hex ring
const FIRST_INTRARING_DIRECTION: hex_directions::Side   = hex_directions::Side::North;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

pub struct WorldGridManager {
    radial_size:            usize,          // Maximum value for an axis of the hex grid
    base_grid_mesh:         ggez_gfx::Mesh, // Mesh for the base hex grid
    hex_map:                HashMap::<coords::Position, HexGridCell>
}

#[derive(Debug)]
pub struct WorldGridError;


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl WorldGridManager {
    /// Returns a new instance of WorldGridManager, with a base grid mesh initialized based on
    /// the GGEZ context's current window dimensions.
    pub fn new(
        //OPT: *DESIGN* Get grid radius from context
        radial_size: usize,
        ci_ctx: &CastIronContext,
        ggez_ctx: &mut GgEzContext ) -> Self {
        // Create manager and update mesh for initialization
        let mut world_grid_manager = Self {
            radial_size,
            base_grid_mesh: ggez_gfx::Mesh::new_line(
                                ggez_ctx,
                                &[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                               crate::DEFAULT_LINE_WIDTH,
                               crate::DEFAULT_LINE_COLOR)
                                .unwrap(),
            hex_map:        Self::build_default_hex_cell_map(radial_size, ci_ctx, ggez_ctx),
        };
        world_grid_manager.update_base_mesh(ggez_ctx);

        world_grid_manager
    }    


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */
     
    pub fn radial_size(&self) -> usize {
        self.radial_size
    }

    pub fn base_grid_mesh(&self) -> &ggez_gfx::Mesh {
        &self.base_grid_mesh
    }

    pub fn hex_map(&self) -> &HashMap::<coords::Position, HexGridCell> {
        &self.hex_map
    }
    

    /*  *  *  *  *  *  *  *\
     *  Mutator Methods   *
    \*  *  *  *  *  *  *  */

    pub fn toggle_cell_highlight(&mut self, cell_position: &coords::Position, ggez_ctx: &mut GgEzContext) -> Result<(), WorldGridError> {
        // Look up cell by position
        match self.hex_map.get_mut(cell_position) {
            Some(hex_cell) => {
                // Update highlight property of the found cell
                hex_cell.toggle_highlight();

                // Update the mesh
                self.update_base_mesh(ggez_ctx);
                Ok(())
            },
            _ => Err(WorldGridError)
        }
    }


    /*  *  *  *  *  *  *  *\
     *  Utility Methods   *
    \*  *  *  *  *  *  *  */

    pub fn draw(&self, ggez_ctx: &mut GgEzContext) {
        // Draw world grid mesh
        ggez_gfx::draw(ggez_ctx, &self.base_grid_mesh, ggez_gfx::DrawParam::default()).unwrap();
    }


    /*  *  *  *  *  *  *  *\
     *  Helper Methods    *
    \*  *  *  *  *  *  *  */
    
    fn update_base_mesh(&mut self, ggez_ctx: &mut GgEzContext) {
        let mut mesh_builder = ggez_gfx::MeshBuilder::new();

        for (_position, hex_cell) in self.hex_map.iter() {
            hex_cell.add_to_mesh(colors::TRANSPARENT,crate::DEFAULT_LINE_COLOR, &mut mesh_builder);
        }

        self.base_grid_mesh = mesh_builder.build(ggez_ctx).unwrap();

        mt_log!(Level::Debug, "Base mesh updated");
    }


    /*  *  *  *  *  *  *  *\
     *  Helper Functions  *
    \*  *  *  *  *  *  *  */

    /// Builds representation of all hex grid cells
    fn build_default_hex_cell_map(radial_size: usize, ci_ctx: &CastIronContext, ggez_ctx: &GgEzContext) -> HashMap<coords::Position, HexGridCell> {
        // There are 6*(n-1) cells for a given (1-based) level n of a hex grid, so size map according to arithmetic sum
        let map_size = 1 + ((radial_size as f32/2.0) * ((2.0*NUM_ADDITIONAL_CELLS_PER_LEVEL as f32) + ((radial_size as f32 - 1.0)*NUM_ADDITIONAL_CELLS_PER_LEVEL as f32))) as usize;

        // Create a hashmap of hex grid cells with the appropriate capacity (avoids expensive re-allocations)
        let mut hex_map: HashMap<coords::Position, HexGridCell> = HashMap::with_capacity(map_size);

        /* Populate Map */
        // Add central hex
        let central_hex_position = coords::Position::default();
        let mut cur_hex_position = central_hex_position;
        let mut cur_hex_cell_instance = HexGridCell::new_from_hex_coords(&cur_hex_position,crate::HEX_RADIUS_VERTEX, ggez_ctx);
        hex_map.insert(cur_hex_position, cur_hex_cell_instance);

        // Add the remainder of the hexes in a spiral pattern
        for radial_level in 1 ..= radial_size {
            // Translate to the starting hex of the next ring, but don't add it to the map (will be done by the innermost loop)
            cur_hex_position.translate(&coords::Translation::from(NEW_RING_START_DIRECTION), ci_ctx).expect("Could not translate to the starting hex of the next ring.");

            let directions: hex_directions::Provider<hex_directions::Side> = hex_directions::Provider::new(FIRST_INTRARING_DIRECTION);
            for direction in directions {
                for _intradirection_step in 0..radial_level {
                    // Add the hex at the current step
                    cur_hex_position.translate(&coords::Translation::from(direction), ci_ctx).expect("Could not translate to next intrastep hex.");

                    cur_hex_cell_instance = HexGridCell::new_from_hex_coords(&cur_hex_position,crate::HEX_RADIUS_VERTEX, ggez_ctx);
                    hex_map.insert(cur_hex_position, cur_hex_cell_instance);
                }
            }
        }

        hex_map
    }
}
