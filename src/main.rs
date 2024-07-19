pub mod cached;
pub mod handles;
pub mod projects;
use handles::*;
use rocket::routes;
use rocket_dyn_templates::Template;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, assets]);

    Ok(rocket.into())
}
