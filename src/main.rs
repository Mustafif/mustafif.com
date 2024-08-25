pub mod cached;
pub mod handles;
pub mod projects;
use handles::*;
use rocket::routes;
use rocket_dyn_templates::Template;

#[rocket::launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, assets])
}
