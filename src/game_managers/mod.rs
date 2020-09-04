/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : game_managers/mod.rs

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
    //TODO: Fill in purpose statement

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use ggez::{
    Context as GgEzContext,
    graphics as ggez_gfx,
};


///////////////////////////////////////////////////////////////////////////////
//  Module Declarations
///////////////////////////////////////////////////////////////////////////////

pub mod actor_manager;
pub mod obstacle_manager;
pub mod resource_manager;
pub mod weather_manager;
pub mod world_grid_manager;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////


//OPT: *STYLE* Better named VisualMechanic/VisibleMechanic?
pub trait DrawableMechanic {

    //OPT: *DESIGN* Many of these can probably be removed via Instance type constraints
    /*  *  *  *  *  *  *  *  *\
     *  Implementor-Defined  *
    \*  *  *  *  *  *  *  *  */

    /// Implementor-defined type representing an instance of its drawable mechanic
    type Instance;

    /// Implementor-defined type indicating an error
    type ErrorType: std::fmt::Debug;

    /// Implementor-defined function to return a reference to its mesh
    fn instances(&self) -> &Vec<Self::Instance>;

    /// Implementor-defined function to return a reference to its mesh
    fn mesh(&self) -> &ggez_gfx::Mesh;

    /// Implementor-defined function to set its mesh
    fn set_mesh(&mut self, mesh: ggez_gfx::Mesh);

    /// Implementor-defined function to an instance of itself to a mesh
    fn add_instance_to_mesh(
        instance: &Self::Instance,
        mesh_builder: &mut ggez_gfx::MeshBuilder,
        ggez_ctx: &mut GgEzContext) -> Result<(),Self::ErrorType>;

    
    /*  *  *  *  *  *  *  *  *\
     *  Defined by Default   *
    \*  *  *  *  *  *  *  *  */

    /// Draws the mesh for the mechanic in the given context
    fn draw(&self, ggez_ctx: &mut GgEzContext) {
        ggez_gfx::draw(ggez_ctx, self.mesh(), ggez_gfx::DrawParam::default()).unwrap();
    }

    /// Updates the mechanic mesh with current instances
    fn update_mesh(&mut self, ggez_ctx: &mut GgEzContext) {        
        // Short-circuit if there are no instances
        if self.instances().is_empty() {
            return;
        }

        //OPT: *PERFORMANCE* is this necesary? could be faster if mesh is updated in-place
        // Create a mesh builder for the update
        let mut mesh_builder = ggez_gfx::MeshBuilder::new();

        // Iterate through instances, adding to the mesh builder along the way
        for instance in self.instances() {
            Self::add_instance_to_mesh(instance, &mut mesh_builder, ggez_ctx).unwrap();
        }

        self.set_mesh(mesh_builder.build(ggez_ctx).unwrap());
    }
}
