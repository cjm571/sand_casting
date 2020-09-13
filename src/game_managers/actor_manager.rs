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
    //TODO: Fill in purpose statement

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use cast_iron::{
    actor::Actor,
    hex_directions,
    logger,
    Locatable,
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
    ci_log,
};


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

//TODO: Proper implementation of an error type
#[derive(Debug)]
pub struct ActorError;

pub struct ActorManager {
    logger:     logger::Instance,
    actors:     Vec<Actor>,
    actor_mesh: ggez_gfx::Mesh,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl ActorManager {
    /// Generic Constructor - creates an empty instance
    pub fn new(logger_original: &logger::Instance, ctx: &mut GgEzContext) -> Self {
        // Clone the logger instance so this module has its own sender to use
        let logger_clone = logger_original.clone();

        ActorManager {
            logger:     logger_clone,
            actors:     Vec::new(),
            actor_mesh: ggez_gfx::Mesh::new_line(ctx,
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
        ci_log!(self.logger, logger::FilterLevel::Debug,
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
        // Get actor position
        let actor_pos = instance.origin();

        //OPT: *PERFORMANCE* Do this in advance and pass in
        // Get window dimensions
        let (window_x, window_y) = ggez_gfx::size(ggez_ctx);
        let window_center = ggez_mint::Point2 {
            x: window_x / 2.0,
            y: window_y / 2.0
        };
        
        //OPT: *PERFORMANCE* Not a great spot for this conversion logic...
        // Calculate x, y offsets to determine (x,y) centerpoint from hex grid coords
        let x_offset = actor_pos.x() as f32 * (::CENTER_TO_VERTEX_DIST * 3.0);
        let y_offset = (-actor_pos.y() as f32 * f32::from(hex_directions::Side::NORTHWEST).sin() * (::CENTER_TO_SIDE_DIST * 2.0)) +
                       (-actor_pos.z() as f32 * f32::from(hex_directions::Side::SOUTHWEST).sin() * (::CENTER_TO_SIDE_DIST * 2.0));

        let actor_center = ggez_mint::Point2 {
            x: window_center.x + x_offset,
            y: window_center.y + y_offset
        };
        
        // Create a HexGridCell object and add it to the mesh builder
        let actor_hex = HexGridCell::new(actor_center, ::GRID_CELL_SIZE);
        
        //FEAT: Actual sprites (or images or something) for actors
        mesh_builder.circle(ggez_gfx::DrawMode::fill(), actor_hex.center(), ::GRID_CELL_SIZE/2.0, 1.0, colors::GREEN);

        Ok(())
    }
}
