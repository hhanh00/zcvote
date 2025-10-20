use anyhow::Result;
use lfrb::api::app::App;
use rocket::{routes, Rocket};
use rocket::response::Debug;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, State};

#[get("/test")]
pub async fn get_test(
    app: &State<App>,
) -> Result<String, Debug<anyhow::Error>> {
    let c = app.test().await?;
    Ok(c.to_string())
}

#[rocket::main]
pub async fn main() -> Result<()> {
    let _ = dotenv::dotenv();

    let rocket = Rocket::build();
    let app = App::new(&dotenv::var("DB").unwrap_or("vote-client.db".to_string())).await?;
    rocket.manage(app).mount("/", routes![get_test]).launch().await?;
    Ok(())
}
