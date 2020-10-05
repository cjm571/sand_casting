/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : game_assets\colors.rs

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
    This module defines various game assets to be used for drawing.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use cast_iron::{
    element::{
        Element,
        Elemental,
    },
    mechanics::{
        resource::{
            Resource,
            State,
        },
    },
};

use ggez::graphics as ggez_gfx;


///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

/* Greyscale */
pub const BLACK:    ggez_gfx::Color = ggez_gfx::BLACK;
pub const WHITE:    ggez_gfx::Color = ggez_gfx::WHITE;
pub const GREY:     ggez_gfx::Color = ggez_gfx::Color {
    r: 0.500,
    g: 0.500,
    b: 0.500,
    a: 1.000
};
pub const DARKGREY: ggez_gfx::Color = ggez_gfx::Color {
    r: 0.333,
    g: 0.333,
    b: 0.333,
    a: 1.000
};

/* ROYGBIV Rainbow (values from wikipedia) */
pub const RED:      ggez_gfx::Color = ggez_gfx::Color {
    r: 1.000,
    g: 0.000,
    b: 0.000,
    a: 1.000
};
pub const ORANGE:   ggez_gfx::Color = ggez_gfx::Color {
    r: 1.000,
    g: 0.496,
    b: 0.000,
    a: 1.000
};
pub const YELLOW:   ggez_gfx::Color = ggez_gfx::Color {
    r: 1.000,
    g: 1.000,
    b: 0.000,
    a: 1.000
};
pub const GREEN:    ggez_gfx::Color = ggez_gfx::Color {
    r: 0.000,
    g: 1.000,
    b: 0.000,
    a: 1.000
};
pub const BLUE:     ggez_gfx::Color = ggez_gfx::Color {
    r: 0.000,
    g: 0.000,
    b: 1.000,
    a: 1.000
};
pub const INDIGO:   ggez_gfx::Color = ggez_gfx::Color {
    r: 0.180,
    g: 0.168,
    b: 0.371,
    a: 1.000
};
pub const VIOLET:   ggez_gfx::Color = ggez_gfx::Color {
    r: 0.543,
    g: 0.000,
    b: 1.000,
    a: 1.000
};

/* Printer Ink (CMYK) */
pub const CYAN:     ggez_gfx::Color = ggez_gfx::Color {
    r: 0.000,
    g: 1.000,
    b: 1.000,
    a: 1.000
};
pub const MAGENTA:  ggez_gfx::Color = ggez_gfx::Color {
    r: 1.000,
    g: 0.000,
    b: 1.000,
    a: 1.000
};

/* Other (alphabetical) */
pub const BROWN:    ggez_gfx::Color = ggez_gfx::Color {
    r: 0.547,
    g: 0.273,
    b: 0.078,
    a: 1.000
};
pub const IVORY:    ggez_gfx::Color = ggez_gfx::Color {
    r: 1.000,
    g: 1.000,
    b: 0.941,
    a: 1.000
};
pub const TRANSPARENT: ggez_gfx::Color = ggez_gfx::Color {
    r: 0.000,
    g: 0.000,
    b: 0.000,
    a: 0.000
};

/* Highlighters (alphabetical) */
pub const HILITE_STD: ggez_gfx::Color = ggez_gfx::Color {
    r: 0.900,
    g: 1.000,
    b: 0.000,
    a: 0.333,
};

///////////////////////////////////////////////////////////////////////////////
//  Utility Functions
///////////////////////////////////////////////////////////////////////////////

pub fn from_element(elem: Element) -> ggez_gfx::Color {
    // Determine base color based on element of resource
    match elem {
        Element::Unset      => panic!("Requested color of Unset Element!"),
        Element::Fire       => RED,
        Element::Ice        => CYAN,
        Element::Wind       => GREEN,
        Element::Water      => BLUE,
        Element::Electric   => YELLOW,
        Element::Earth      => BROWN,
        Element::Light      => IVORY,
        Element::Dark       => INDIGO
    }
}

pub fn from_resource(res: &Resource) -> ggez_gfx::Color {
    // Determine base color based on element of resource
    let mut res_color = from_element(res.element());

    // Adjust alpha based on state
    match res.state() {
        State::Depleted => res_color.a = 0.000,
        State::Low      => res_color.a = 0.050,
        State::Partial  => res_color.a = 0.100,
        State::High     => res_color.a = 0.150,
        State::Full     => res_color.a = 0.200,
        State::Overflow => res_color.a = 1.000
    }

    res_color
}
