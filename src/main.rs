mod schema;
use diesel::{
    debug_query, insert_into,
    pg::{Pg, PgConnection},
    prelude::Insertable,
    Connection, ExpressionMethods, QueryDsl, Queryable, RunQueryDsl,
};
use fake::{Dummy, Fake, Faker};
use length_aware_paginator::{Paginate, Response};
use schema::users;
use std::env;
use uuid::Uuid;

fn get_connection() -> PgConnection {
    let database_url =
        env::var("DATABASE_URL").expect("You have to provide DATABASE_URL to run tests");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url))
}

#[derive(Queryable, Debug, Dummy, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    id: Uuid,
    username: String,
    email: String,
}

#[allow(unused)]
fn seed() {
    let mut conn = get_connection();
    for _ in 0..100 {
        let user: User = Faker.fake();
        insert_into(users::table)
            .values(user)
            .execute(&mut conn)
            .expect("Error inserting user");
    }
}

fn main() {
    // seed();

    let mut connection = get_connection();
    // let query = users::table.count();

    let query = users::table.filter(users::username.eq("nesto")).count();

    println!("{}", debug_query::<Pg, _>(&query));

    let response= users::table
        .into_boxed()
        .page(Some(20))
        // .per_page(Some(10))
        // .paginate::<User>(&mut connection)
        // .unwrap();
        ;

    println!("{}", debug_query(&response));
    // println!("Items: {}", response.data.len());
    // println!("Page: {}", response.page);
    // println!("Per page: {}", response.per_page);
    // println!("Total: {}", response.total);
    // println!("Last page: {}", response.last_page);
}
