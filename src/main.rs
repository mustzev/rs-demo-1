mod cornucopia;
mod diesel;
mod linfa;
mod prompt;

#[tokio::main]
async fn main() {
    dotenvy::from_filename("env/.env").expect("Cannot load env variables");

    // cornucopia::run::run().await;

    // linfa::run::run();

    // prompt::run::run();

    diesel::run::run();
}
