use diesel::insert_into;
use diesel::r2d2::{
    Pool,
    ConnectionManager,
};
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use diesel::query_dsl::{
    RunQueryDsl,
    QueryDsl,
};

use models::{
    Player,
    PlayerAbility,
    SpecialAbility,
    Team,
};

use schema::{
    teams,
    special_abilities,
    players,
    player_abilities,
};

pub fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool")
}

pub fn find_player<'a>(
    conn: &'a PgConnection,
    name: &'a str,
) -> Result<Player, &'a str> {
    let output = players::table
        .filter(players::name.eq(name))
        .first::<Player>(conn);
    match output {
        Ok(player) => Ok(player),
        _ => Err("Player not found"),
    }
}

pub fn player_from_id<'a>(
    conn: &'a PgConnection,
    id: i32,
) -> Result<Player, &'a str> {
    let output = players::table
        .find(id)
        .first::<Player>(conn);
    match output {
        Ok(player) => Ok(player),
        _ => Err("Player not found"),
    }
}

pub fn team_from_id<'a>(
    conn: &'a PgConnection,
    id: i32,
) -> Result<Team, &'a str> {
    let output = teams::table
        .find(id)
        .first::<Team>(conn);
    match output {
        Ok(team) => Ok(team),
        _ => Err("Team not found"),
    }

}

pub fn player_abilities<'a>(
    conn: &'a PgConnection,
    player_id: i32,
) -> Result<Vec<SpecialAbility>, &'a str> {
    /// Returns a player's vec of special abilities
    ///
    /// Currently uses a closure to propagate diesel-specific errors first
    ///
    let get_result = || -> Result<Vec<SpecialAbility>, diesel::result::Error> {
        let player_abilities = player_abilities::table
            .filter(player_abilities::player_id.eq(player_id))
            .load::<PlayerAbility>(conn)?;
        let output: Result<Vec<SpecialAbility>, diesel::result::Error> = {
            player_abilities
                .iter()
                .map(|player_ability| {
                    special_abilities::table
                        .find(player_ability.special_ability_id)
                        .first::<SpecialAbility>(conn)
                })
                .collect()
        };
        output
    };
    match get_result() {
        Ok(output) => Ok(output),
        _ => Err("Error finding player abilities"),
    }
}

pub fn create_player<'a>(
    conn: &'a PgConnection,
    name: &'a str,
    team_name: &'a str,
    abilities: &'a [String]
) -> i32 {
    // Returns the player id
    let special_ability_ids: Vec<i32> = abilities
        .into_iter()
        .map(|ability_name| {
            let ability = special_abilities::table
                .filter(special_abilities::name.eq(ability_name))
                .first::<SpecialAbility>(conn);
            match ability {
                Err(_) => {
                    insert_into(special_abilities::table)
                        .values(special_abilities::name.eq(ability_name))
                        .returning(special_abilities::id)
                        .get_result(conn)
                        .expect("Special ability could not be inserted")
                },
                Ok(ability) => ability.id,
            }
        })
        .collect();

    let found_team = teams::table
        .filter(teams::name.eq(team_name))
        .first::<Team>(conn);
    let team_id = match found_team {
        Err(_) => {
            insert_into(teams::table)
                .values(teams::name.eq(team_name))
                .returning(teams::id)
                .get_result(conn)
                .expect("Special ability could not be inserted")
        },
        Ok(team) => team.id,
    };
    let values = (players::name.eq(name), players::team_id.eq(team_id));
    let player_id = insert_into(players::table)
        .values(values)
        .returning(players::id)
        .get_result(conn)
        .expect("issue creating player");
    for special_ability_id in &special_ability_ids {
        let special_vals = (
            player_abilities::player_id.eq(player_id),
            player_abilities::special_ability_id.eq(special_ability_id),
        );
        insert_into(player_abilities::table)
            .values(special_vals)
            .execute(conn)
            .expect("issue creating special ability");
    }
    player_id
}
