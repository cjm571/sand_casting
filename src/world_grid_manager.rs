/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : world_grid_manager.rs

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

use cast_iron::debug_exec;

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

pub struct WorldGridManager {
    max_radial_distance: usize,       // Maximum value for an axis of the hex grid
    base_grid_mesh: ggez_gfx::Mesh  // Mesh for the base hex grid
}

//TODO: Proper implementation of an error type
#[derive(Debug)]
pub struct WorldGridError;


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl WorldGridManager {
    /// Returns a new instance of WorldGridManager, with a base grid mesh initialized based on
    /// the GGEZ context's current window dimensions.
    pub fn new(max_radial_distance: usize, ctx: &mut GgEzContext) -> Self {
        let mut world_manager = WorldGridManager {
            max_radial_distance: max_radial_distance,
            base_grid_mesh: ggez_gfx::MeshBuilder::new()
                                .line(
                                    &[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                                    ::DEFAULT_LINE_WIDTH,
                                    ::DEFAULT_LINE_COLOR
                                )
                                .unwrap()
                                .build(ctx)
                                .unwrap()
        };

        // Get window dimensions
        let (window_x, window_y) = ggez_gfx::size(ctx);
        let center = ggez_mint::Point2 {x: window_x / 2.0, y: window_y / 2.0};

        // Create a mesh builder for the base hex grid
        let mut base_grid_mesh_builder = ggez_gfx::MeshBuilder::new();
        world_manager.build_base_grid(center, &mut base_grid_mesh_builder);

        // Build the base hex grid mesh and draw it
        world_manager.base_grid_mesh = base_grid_mesh_builder.build(ctx).unwrap();

        world_manager
    }    


    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////
     
    pub fn get_grid_size(&self) -> usize {
        self.max_radial_distance
    }

    pub fn get_base_grid_mesh(&self) -> &ggez_gfx::Mesh {
        &self.base_grid_mesh
    }


    ///////////////////////////////////////////////////////////////////////////
    //  Helper Functions
    ///////////////////////////////////////////////////////////////////////////

    /// Builds a baseline hex grid to the graphics window.
    fn build_base_grid(
        &self,
        center: ggez_mint::Point2<f32>,
        mesh_builder: &mut ggez_gfx::MeshBuilder
    ) {
        // Construct the central hex cell
        let central_hex_cell = HexGridCell::new(center, ::GRID_CELL_SIZE);
    
        // Add it, and its radials to the mesh
        central_hex_cell.add_to_mesh(colors::TRANSPARENT, ::DEFAULT_LINE_COLOR, mesh_builder);
        central_hex_cell.add_radials_to_mesh(
            colors::TRANSPARENT,
            ::DEFAULT_LINE_COLOR,
            ::DEFAULT_HEX_GRID_MAX_RADIAL_DISTANCE,
            false,
            mesh_builder);

        debug_exec!{
            // Add central hex with a green outline for visibility
            central_hex_cell.add_to_mesh(colors::TRANSPARENT, colors::GREEN, mesh_builder)
        };
    }

    //FIXME: Would be nice to have a global tracker of all occupied hexes in the grid, to simplify collision checking
}
