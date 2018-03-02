use super::schema::{
    player_abilities,
    players,
    special_abilities,
    teams,
};

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "special_abilities"]
pub struct SpecialAbility {
    pub id: i32,
    pub name: String,
}


#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "teams"]
pub struct Team {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Team)]
#[table_name = "players"]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub team_id: i32,
}


#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Player)]
#[belongs_to(SpecialAbility)]
#[table_name = "player_abilities"]
pub struct PlayerAbility {
    pub id: i32,
    pub player_id: i32,
    pub special_ability_id: i32,
}
