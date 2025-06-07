use bevy::prelude::*;

#[derive(Component)]
pub struct Faction {
    this_faction: Factions,
    relationships: Vec<RelationshipState>,
}

// System to initialize faction states by creating a new faction for each (except player)
pub fn initialize_all_factions() {
    let technocrat_faction = Faction {
        this_faction: Factions::Technocrat,
        relationships: generate_initial_faction(),
    };
    let nomad_faction = Faction {
        this_faction: Factions::Nomad,
        relationships: generate_initial_faction(),
    };
    let scavenger_faction = Faction {
        this_faction: Factions::Scavenger,
        relationships: generate_initial_faction(),
    };
    let dominion_faction = Faction {
        this_faction: Factions::Dominion,
        relationships: generate_initial_faction(),
    };

    //TODO: Need to make these entities. Most likely this will be a spawn script later
}

//TODO: create a system to get/set these values for any entity

// Generates blank faction relationship data
fn generate_initial_faction() -> Vec<RelationshipState> {
    vec![
        RelationshipState {
            faction: Factions::Technocrat,
            political: 0,
            social: 0,
        },
        RelationshipState {
            faction: Factions::Nomad,
            political: 0,
            social: 0,
        },
        RelationshipState {
            faction: Factions::Scavenger,
            political: 0,
            social: 0,
        },
        RelationshipState {
            faction: Factions::Dominion,
            political: 0,
            social: 0,
        },
        RelationshipState {
            faction: Factions::Player,
            political: 0,
            social: 0,
        },
    ]
}

#[derive(Debug)]
struct RelationshipState {
    faction: Factions,
    political: i32,
    social: i32,
}

#[derive(Debug, Reflect)]
pub enum Factions {
    Technocrat,
    Nomad,
    Scavenger,
    Dominion,
    Player,
}
