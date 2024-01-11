pub mod cached;
pub mod handles;
pub mod mailer;
use handles::*;
use rocket::{routes, Build, Rocket};
use rocket_dyn_templates::Template;

#[rocket::launch]
async fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, assets ,contact])
        .attach(Template::fairing())
}
