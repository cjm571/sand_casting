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
    logger,
    ci_log,
};

use ggez::{
    Context as GgEzContext,
    graphics as ggez_gfx,
    mint as ggez_mint,
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


