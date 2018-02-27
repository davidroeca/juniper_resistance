use diesel::r2d2;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use super::models::{
    Player,
};

pub fn create_pool() -> r2d2::Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool")
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
) -> Option<Player> {
    use schema::players;
    use schema::players::dsl;
    players.filter(dsl::name.eq(name)).first(conn)
}

pub fn create_player<'a>(
    conn: &'a PgConnection,
    name: &'a str,
    team: &'a str,
    abilities: &'a [String]
) -> Player {
    use schema::players;
    use schema::special_abilities;
    use schema::teams;

    let special_ability_ids = {
        use schema::special_abilities::dsl::*;
        abilities
            .into_iter()
            .map(|ability_name| {
                let ability = special_abilities
                    .filter(name.eq(ability_name))
                    .first(conn);
                match ability {
                    None => {
                        insert_into(special_abilities)
                            .values(name.eq(ability_name))
                            .execute(conn)
                    },
                    Some(output) => output,
                }
            })
            .collect()
    };
    let values = {
        use schema::teams::dsl::*;
        let found_team = teams
            .filter(name.eq(team_name))
            .first(conn);
        let team_id = match found_team {
            None => {
                let output = insert_into(teams)
                    .values(teams::dsl::name.eq(team_name))
                    .execute(conn);
                output.id
            },
            Some(output) => output.id,
        };
        (dsl::name.eq(name), dsl::team_id.eq(team_id))
    };

    let player = {
        use schema::players::dsl::*;
        insert_into(players)
            .values(values)
            .execute(conn)
    };
    use schema::special_abilities::dsl::*;
    for special_ability_id in &special_ability_ids {
        let special_vals = (
            player_id(player.id),
            team_id(team_id),
        );
        insert_into(special_abilities)
            .values(special_vals)
            .execute(conn);
    }
    player
}
