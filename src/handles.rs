use crate::cached::CachedNameFile;
use crate::projects::projects;
use mkblogs_rss::{feed, Source};
use rocket::fs::NamedFile;
use rocket::get;
use rocket_dyn_templates::{context, Template};
use std::path::{Path, PathBuf};
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[get("/")]
pub async fn index() -> Template {
    let feed = feed().await.unwrap_or_default();
    let devto = feed.get(&Source::DevTo).unwrap();
    let mokareads = feed.get(&Source::MoKa).unwrap();
    let mut projects = projects().await.unwrap_or_default();
    let _ = projects.projects.iter_mut().map(|x|x.change_end()).collect::<()>();
    Template::render(
        "index",
        context! {
            version: VERSION,
            moka_articles: mokareads,
            devto_articles: devto, 
            projects: projects.projects
        },
    )
}

#[get("/assets/<file..>")]
pub async fn assets(file: PathBuf) -> Option<CachedNameFile> {
    NamedFile::open(Path::new("assets/").join(file))
        .await
        .ok()
        .map(|file| CachedNameFile::max_age(file, 31536000))
}
