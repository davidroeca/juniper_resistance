#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate rocket;

#[macro_use] extern crate juniper;
extern crate juniper_rocket;

use rocket::response::content;
use rocket::State;


use juniper::{FieldResult};

use diesel::r2d2::{
    Pool,
    ConnectionManager,
};
use diesel::pg::PgConnection;

pub mod models;
pub mod schema;
pub mod database;

use database::{
    create_pool,
    find_player,
    team_from_id,
    player_abilities,
    create_player,
};
// Define base graphql object types
//------------------------------------------------------------

const RESISTANCE_STR: &str = "resistance";
const SPIES_STR: &str = "spies";
const KNOWS_MERLIN_STR: &str = "knows_merlin";
const CAN_SEE_SPIES_STR: &str = "can_see_spies";
const UNKNOWN_STR: &str = "unknown";

#[derive(Clone)]
#[derive(GraphQLEnum)]
enum Team {
    Spy,
    Resistance,
    Unknown,
}

#[derive(Clone)]
#[derive(GraphQLEnum)]
enum SpecialAbility {
    CanSeeSpies,
    KnowsMerlin,
    Unknown,
}

#[derive(Clone)]
#[derive(GraphQLObject)]
#[graphql(description="A player in the game of resistance")]
struct Player {
    id: i32,
    name: String,
    special_abilities: Vec<SpecialAbility>,
    team: Team,
}


#[derive(Clone)]
#[derive(GraphQLInputObject)]
#[graphql(description="A player in the game of resistance")]
struct NewPlayer {
    name: String,
    special_abilities: Vec<SpecialAbility>,
    team: Team,
}

//------------------------------------------------------------
// Define prerequisite database for the examples
//------------------------------------------------------------

struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            pool: create_pool(),
        }
    }

    pub fn get_pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        self.pool.clone()
    }
}

struct Context {
    database: Database,
}

impl Context {
    fn new() -> Context {
        Context {
            database: Database::new(),
        }
    }
}

impl juniper::Context for Context {}

struct Query;

//------------------------------------------------------------
// Define the broad graphql interface
//------------------------------------------------------------
graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field player(&executor, name: String) -> FieldResult<Player> {
        let context = executor.context();
        let pool = context.database.get_pool();
        let connection = pool.get()?;

        let player_db = find_player(&connection, name.as_str())?;
        let abilities_db = player_abilities(&connection, player_db.id)?;
        let team_db = team_from_id(&connection, player_db.team_id)?;
        let team = match team_db.name.as_str() {
            SPIES_STR => Team::Spy,
            RESISTANCE_STR => Team::Resistance,
            _ => Team::Unknown,
        };
        let special_abilities: Vec<SpecialAbility> = abilities_db
            .iter()
            .map(|ability| match ability.name.as_str() {
                CAN_SEE_SPIES_STR => SpecialAbility::CanSeeSpies,
                KNOWS_MERLIN_STR => SpecialAbility::KnowsMerlin,
                _ => SpecialAbility::Unknown,
            })
            .filter(|x| match x {
                &SpecialAbility::Unknown => false,
                _ => true,
            })
            .collect();
        Ok(Player {
            id: player_db.id,
            name: name,
            team: team,
            special_abilities: special_abilities,
        })
    }

});

struct Mutation;

graphql_object!(Mutation: Context |&self| {

    field createPlayer(&executor, new_player: NewPlayer) -> FieldResult<Player> {
        let context = executor.context();
        let pool = context.database.get_pool();
        let connection = pool.get()?;
        let team_name = match new_player.team {
            Team::Spy => SPIES_STR,
            Team::Resistance => RESISTANCE_STR,
            Team::Unknown => UNKNOWN_STR,
        };
        let abilities: Vec<String> = new_player.special_abilities
            .iter()
            .map(|ability| {
                match ability {
                    &SpecialAbility::CanSeeSpies => CAN_SEE_SPIES_STR.to_string(),
                    &SpecialAbility::KnowsMerlin => KNOWS_MERLIN_STR.to_string(),
                    _ => UNKNOWN_STR.to_string(),
                }
            })
            .collect();
        let player_id = create_player(
           &connection,
           new_player.name.as_str(),
           team_name,
           abilities.as_slice(),
        );
        Ok(Player{
            id: player_id,
            name: new_player.name,
            special_abilities: new_player.special_abilities,
            team: new_player.team,
        })
    }

});

type Schema = juniper::RootNode<'static, Query, Mutation>;

//------------------------------------------------------------
// Define the routes
//------------------------------------------------------------
#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn rocket() -> rocket::Rocket {
    let context = Context::new();
    rocket::ignite()
        .manage(context)
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler]
        )
}

fn main() {
    rocket().launch();
}
