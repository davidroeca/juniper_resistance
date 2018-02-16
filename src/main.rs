#[macro_use] extern crate juniper;

use juniper::{FieldResult};

#struct DatabasePool;
#impl DatabasePool {};

use juniper::{FieldResult};
#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

struct Context {
    pool: DatabasePool,
}

impl juniper::Context for Context {}

struct Query;

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field human(&executor, id: String) -> FieldResult<Human> {
        let context = executor.context();
        let connection = context.pool.get_connection()?;
        let human = connection.find_human(&id)?;
        Ok(human)
    }
});

struct Mutation;

graphql_object!(Mutation: Context |&self| {
    field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
        let db = executor.context().pool.get_connection()?;
        let human: Human = db.insert_human(&new_human)?;
        Ok(human)
    }
});

type Schema = juniper::RootNode<'static, Query, Mutation>;

fn main() {
    println!("Hello, world!");
}
