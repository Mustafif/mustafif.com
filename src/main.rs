pub mod cached;
pub mod handles;
use handles::*;
use rocket::{routes, Build, Rocket};
use rocket_dyn_templates::Template;

#[rocket::launch]
async fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, assets])
        .attach(Template::fairing())
}
