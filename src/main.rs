// Filename : actor.rs
// Author   : CJ McAllister
// Created  : 22-11-2017
// License  : TODO: Add license info

extern crate cast_iron;

use cast_iron::actor::Actor;
use cast_iron::ability::Ability;
use cast_iron::ability::aspect::*;


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

// Outputs the stats of the given player to the terminal
fn print_stats(actor: Actor) {
    println!("BEGIN Stats for actor {}", actor.get_name());

    let pos = actor.get_pos();
    println!(" - Position: ({}, {}, {})", pos[0], pos[1], pos[2]);

    println!(" - Fatigue: Current: {}", actor.get_cur_fatigue());

    println!(" - Abilities:");
    let mut i = 1;
    for abil in actor.get_abilities() {
        println!("   {}: {} - Potency {}", i, abil.get_name(), abil.get_potency());
        for key in abil.get_aspects().keys() {
            println!("      {:?}: IDK", key);
            //TODO: Union was a poor choice... look into example at https://stackoverflow.com/a/29249230
        }
        i = i + 1
    }

    println!("END Stats for actor {}", actor.get_name());
}

fn main() {
    let mut lightning_bolt: Ability = Ability::new();
    lightning_bolt.set_name("Lightning Bolt");
    lightning_bolt.set_aspect(Tag::Aesthetics, Value {aesthetics: Aesthetics::Impressive});
    lightning_bolt.set_aspect(Tag::Element, Value {element: Element::Electric});
    lightning_bolt.set_aspect(Tag::Method, Value {method: Method::Wand});
    lightning_bolt.set_aspect(Tag::Morality, Value {morality: Morality::Neutral});
    lightning_bolt.set_aspect(Tag::School, Value {school: School::Destruction});

    let mut player_one: Actor = Actor::new();
    player_one.set_name("CJ McAllister");
    player_one.add_ability(lightning_bolt);

    print_stats(player_one);
}

