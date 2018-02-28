use diesel::insert_into;
use diesel::r2d2::{
    Pool,
    ConnectionManager,
};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use super::models::{
    Player,
    PlayerAbility,
    SpecialAbility,
    Team,
};

pub fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool")
}

//pub fn establish_connection() -> PgConnection {
    //dotenv().ok();

    //let database_url = env::var("DATABASE_URL")
        //.expect("DATABASE_URL must be set");
    //PGConnection::establish(&database_url)
        //.expect(&format!("Error connecting to {}", database_url);
//}

pub fn find_player<'a>(
    conn: &'a PgConnection,
    name: &'a str,
) -> Player {
    use schema::players::dsl::*;
    players.filter(name.eq(name)).first(conn).expect("Player not found")
}

pub fn player_from_id<'a>(
    conn: &'a PgConnection,
    id: &'a str,
) -> Player {
    use schema::players::dsl::*;
    players.find(id).first(conn).expect("Player not found")
}

pub fn team_from_id<'a>(
    conn: &'a PgConnection,
    id: i32,
) -> Team {
    use schema::teams::dsl::*;
    teams.find(id).first(conn).expect("Error finding team")
}

pub fn player_abilities<'a>(
    conn: &'a PgConnection,
    player_id_in: i32,
) -> Vec<SpecialAbility> {
    let player_abilities = {
        use schema::player_abilities::dsl::*;
        player_abilities
            .filter(player_id.eq(player_id_in))
            .load::<PlayerAbility>(conn)
            .expect("Player abilities not found")
    };
    let special_abilities: Vec<SpecialAbility> = {
        use schema::special_abilities::dsl::*;
        player_abilities
            .iter()
            .map(|player_ability| {
                special_abilities
                    .find(player_ability.special_ability_id)
                    .first(conn)
                    .expect("Ability not found")
            }).collect()
    };
    special_abilities
}

pub fn create_player<'a>(
    conn: &'a PgConnection,
    name: &'a str,
    team_name: &'a str,
    abilities: &'a [String]
) -> i32 {
    // Returns the player id
    let special_ability_ids = {
        use schema::special_abilities::dsl::*;
        abilities
            .into_iter()
            .map(|ability_name| {
                let ability = special_abilities
                    .filter(name.eq(ability_name))
                    .first(conn)
                    .expect("ability not found");
                match ability {
                    None => {
                        insert_into(special_abilities)
                            .values(name.eq(ability_name))
                            .returning(id)
                            .get_result(conn)
                    },
                    Some(ability) => ability.id,
                }
            })
            .collect()
    };
    let team_id;
    let values = {
        use schema::teams::dsl::*;
        let found_team = teams
            .filter(name.eq(team_name))
            .first(conn)
            .expect("team not found");
        team_id = match found_team {
            None => {
                insert_into(teams)
                    .values(name.eq(team_name))
                    .returning(id)
                    .get_result(conn)
            },
            Some(team) => team.id,
        };
        (name.eq(name), team_id.eq(team_id))
    };

    let player_id_in = {
        use schema::players::dsl::*;
        insert_into(players)
            .values(values)
            .returning(id)
            .get_result(conn)
            .expect("issue creating player")
    };
    use schema::player_abilities::dsl::*;
    for special_ability_id in &special_ability_ids {
        let special_vals = (
            player_id.eq(player_id_in),
            team_id.eq(team_id),
        );
        insert_into(player_abilities)
            .values(special_vals)
            .execute(conn)
            .expect("issue creating special ability");
    }
    player_id_in
}
