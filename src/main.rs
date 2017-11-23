// Filename : actor.rs
// Author   : CJ McAllister
// Created  : 22-11-2017
// License  : TODO: Add license info

extern crate cast_iron;

use cast_iron::actor::Actor;
use cast_iron::ability::Ability;
use cast_iron::ability::aspect;

fn main() {
    let mut lightning_bolt: Ability = Ability::new();
    lightning_bolt.set_aesthetics(aspect::Aesthetics::Impressive);
    lightning_bolt.set_element(aspect::Element::Electric);
    lightning_bolt.set_method(aspect::Method::Wand);
    lightning_bolt.set_morality(aspect::Morality::Neutral);
    lightning_bolt.set_school(aspect::School::Destruction);

    let mut player_one: Actor = Actor::new();
    player_one.add_ability(lightning_bolt);
}
