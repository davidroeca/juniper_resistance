table! {
    player_abilities (id) {
        id -> Int4,
        player_id -> Int4,
        special_ability_id -> Int4,
    }
}

table! {
    players (id) {
        id -> Int4,
        name -> Varchar,
        team_id -> Int4,
    }
}

table! {
    special_abilities (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    teams (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(player_abilities -> players (player_id));
joinable!(player_abilities -> special_abilities (special_ability_id));
joinable!(players -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    player_abilities,
    players,
    special_abilities,
    teams,
);
