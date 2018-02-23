#[derive(Queryable)]
pub struct SpecialAbilities {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[table_name="special_abilities"]
pub struct NewSpecialAbility {
    pub name: String
}

#[derive(Queryable)]
pub struct Teams {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[table_name="teams"]
pub struct NewTeam {
    pub name: String
}

#[derive(Queryable)]
pub struct Players {
    pub id: i32,
    pub name: String,
    pub team_id: i32
}


#[derive(Insertable)]
#[table_name="players"]
pub struct NewPlayer {
    pub name: String,
    pub team_name: String,
    pub special_abilities: Vec<String>
}

#[derive(Queryable)]
pub struct PlayerAbilities {
    pub id: i32,
    player_id: i32,
    special_ability_id: i32
}
