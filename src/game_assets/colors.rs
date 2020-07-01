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

Changelog:
    CJ McAllister   01 Jul 2020     File created
\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use ggez::graphics;

///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

pub const BLACK:    graphics::Color = graphics::BLACK;
pub const WHITE:    graphics::Color = graphics::WHITE;

pub const RED:      graphics::Color = graphics::Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0
};

pub const GREEN:    graphics::Color = graphics::Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 1.0
};

pub const BLUE:     graphics::Color = graphics::Color {
    r: 0.0,
    g: 0.0,
    b: 1.0,
    a: 1.0
};

pub const YELLOW:   graphics::Color = graphics::Color {
    r: 1.0,
    g: 1.0,
    b: 0.0,
    a: 1.0
};

pub const CYAN:     graphics::Color = graphics::Color {
    r: 0.0,
    g: 1.0,
    b: 1.0,
    a: 1.0
};

pub const PURPLE:   graphics::Color = graphics::Color {
    r: 1.0,
    g: 0.0,
    b: 1.0,
    a: 1.0
};

pub const GREY:     graphics::Color = graphics::Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0
};
