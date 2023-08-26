use surrealdb::{Surreal, engine::local::File, opt::auth::Root};

#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    let db = Surreal::new::<File>("test.db").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("namespace").use_db("pile").await?;

    println!("worked");

    Ok(())
}
