#![feature(plugin)]
#![plugin(rocket_codegen)]

use std::collections::HashMap;
use std::sync::RwLock;

extern crate rocket;
use rocket::response::content;
use rocket::State;

//#[macro_use] extern crate rocket_contrib;

#[macro_use] extern crate juniper;
extern crate juniper_rocket;

use juniper::{FieldResult};

//------------------------------------------------------------
// Define base graphql object types
//------------------------------------------------------------
#[derive(Clone)]
#[derive(GraphQLEnum)]
enum Team {
    Spy,
    Resistance,
}

#[derive(Clone)]
#[derive(GraphQLEnum)]
enum SpecialAbility {
    CanSeeSpies,
    KnowsMerlin,
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
type ID = i32;

struct Database {
    players: HashMap<ID, Player>,
    max_player_id: ID,
}

impl Database {

    pub fn new() -> Database {
        Database {
            players: HashMap::<ID, Player>::new(),
            max_player_id: 0,
        }
    }

    pub fn find_player(&self, id: &ID) -> Result<Player, &'static str> {
        match self.players.get(&id) {
            None => Err("No Player found"),
            Some(player) => Ok(player.clone()),
        }
    }

    pub fn insert_player(&mut self, new_player: NewPlayer) -> Player {
        let next_player_id = self.max_player_id + 1;
        let player = Player {
            id: next_player_id,
            name: new_player.name,
            special_abilities: new_player.special_abilities,
            team: new_player.team,
        };
        self.players.insert(next_player_id, player.clone());
        self.max_player_id = next_player_id;
        player
    }
}

struct Context {
    database: RwLock<Database>,
}

impl Context {
    fn new() -> Context {
        Context {
            database: RwLock::new(Database::new()),
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

    field player(&executor, id: ID) -> FieldResult<Player> {
        let context = executor.context();
        let database = context.database.try_read()?;
        let player = database.find_player(&id)?;
        Ok(player)
    }

});

struct Mutation;

graphql_object!(Mutation: Context |&self| {

    field createPlayer(&executor, new_player: NewPlayer) -> FieldResult<Player> {
        let context = executor.context();
        let mut database = context.database.try_write()?;
        let player: Player = database.insert_player(new_player);
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

