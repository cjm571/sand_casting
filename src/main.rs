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
    println!("Position: ({}, {}, {})", pos[0], pos[1], pos[2]);

    println!("Fatigue: Current: {}", actor.get_cur_fatigue());

    println!("Abilities:");
    let mut i = 1;
    for abil in actor.get_abilities() {
        println!("{}: Name:    {}", i, abil.get_name());
        println!("   Potency: {}", abil.get_potency());
        println!("   {:?}", abil.get_aspects());
        i = i + 1;
    }

    println!("END Stats for actor {}", actor.get_name());
}

fn main() {
    let null_abil: Ability = Ability::new("Null");

    let mut lightning_bolt: Ability = Ability::new("Lightning Bolt");
    lightning_bolt.set_potency(20);
    lightning_bolt.set_aesthetics(Aesthetics::Impressive);
    lightning_bolt.set_element(Element::Electric);
    lightning_bolt.set_method(Method::Wand);
    lightning_bolt.set_morality(Morality::Neutral);
    lightning_bolt.set_school(School::Destruction);

    let mut blood_drain: Ability = Ability::new("Blood Drain");
    blood_drain.set_potency(50);
    blood_drain.set_aesthetics(Aesthetics::Ugly);
    blood_drain.set_element(Element::Dark);
    blood_drain.set_method(Method::Manual);
    blood_drain.set_morality(Morality::Evil);
    blood_drain.set_school(School::Destruction);

    let mut player_one: Actor = Actor::new("CJ McAllister");
    player_one.add_ability(lightning_bolt);
    player_one.add_ability(blood_drain);
    player_one.add_ability(null_abil);

    print_stats(player_one);
}

