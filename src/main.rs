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

use diesel::r2d2;

pub mod models;
pub mod schema;
pub mod database;

use database::{
    find_player,
    create_player,
};

//------------------------------------------------------------
// Define base graphql object types
//------------------------------------------------------------

static RESISTANCE_STR: &'static str = "resistance";
static SPIES_STR: &'static str = "spies";
static KNOWS_MERLIN_STR: &'static str = "knows_merlin";
static CAN_SEE_SPIES_STR: &'static str = "can_see_spies";

type ID = i32;

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
    id: ID,
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
    pool: Option<r2d2::Pool>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            pool: None,
        }
    }

    pub fn get_pool(&mut self) -> r2d2::Pool {
        match self.pool {
            None => {self.pool = create_pool();},
            Some(pool) => (),
        };
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

    field player(&executor, name: &str) -> FieldResult<Player> {
        let context = executor.context();
        let pool = context.database.get_pool();
        let connection = pool.get();

        let player = find_player(name)?;
        Ok(player)
    }

});

struct Mutation;

graphql_object!(Mutation: Context |&self| {

    field createPlayer(&executor, new_player: NewPlayer) -> FieldResult<Player> {
        let context = executor.context();
        let pool = context.database.get_pool();
        let connection = pool.get();
        let team_name = match new_player.team {
            Spy => "spy",
            Resistance => "resistance",
        };
        let abilities: Vec<String> = abilities.into_iter().map(|ability| {
            match ability {
                CanSeeSpies => "can_see_spies".to_string(),
                KnowsMerlin => "knows_merlin".to_string(),
            }
        }).collect();
        let player: Player = create_player(
           connection,
           new_player.name,
           team_name,
           abilities.as_slice(),
        );
        Ok(player)
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
