/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : game_managers/actor_manager.rs

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
    This module manages all active actors (both PCs and NPCs) in the game, as
    well as providing utility methods for drawing, moving, etc.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use cast_iron::{
    actor::Actor,
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

pub struct ActorManager {
    actors:     Vec<Actor>,
    actor_mesh: ggez_gfx::Mesh,
}

#[derive(Debug)]
pub struct ActorError;


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl ActorManager {
    /// Generic Constructor - creates an empty instance
    pub fn new(ggez_ctx: &mut GgEzContext) -> Self {
        ActorManager {
            actors:     Vec::new(),
            actor_mesh: ggez_gfx::Mesh::new_line(ggez_ctx,
                                                 &[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                                                 ::DEFAULT_LINE_WIDTH,
                                                 ::DEFAULT_LINE_COLOR)
                                                 .unwrap(),
        }
    }
}



///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl DrawableMechanic for ActorManager {
    type Instance = Actor;
    type ErrorType = ActorError;

    fn instances(&self) -> &Vec<Self::Instance> {
        &self.actors
    }

    fn push_instance(&mut self, instance: Self::Instance) {
        mt_log!(Level::Debug,
            "Adding actor: {} at {} to mesh.",
            instance.name(),
            instance.origin());

        self.actors.push(instance);
    }

    fn mesh(&self) -> &ggez_gfx::Mesh {
        &self.actor_mesh
    }

    fn set_mesh(&mut self, mesh: ggez_gfx::Mesh) {
        self.actor_mesh = mesh;
    }

    fn add_instance_to_mesh_builder(instance: &Self::Instance,
                                    mesh_builder: &mut ggez_gfx::MeshBuilder,
                                    ggez_ctx: &mut GgEzContext) -> Result<(),Self::ErrorType> {
        // Create a HexGridCell object and add it to the mesh builder
        let actor_hex = HexGridCell::new_from_hex_coords(instance.origin(), ::HEX_RADIUS_VERTEX, ggez_ctx);
        
        // Draw green circle to represent the actor
        mesh_builder.circle(ggez_gfx::DrawMode::fill(), actor_hex.center(), ::HEX_RADIUS_VERTEX/2.0, 1.0, colors::GREEN);

        Ok(())
    }
}
