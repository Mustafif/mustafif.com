use crate::cached::CachedNameFile;
use mkblogs_rss::{feed, Source};
use rocket::fs::NamedFile;
use rocket::get;
use rocket_dyn_templates::{context, Template};
use std::path::{Path, PathBuf};

#[get("/")]
pub async fn index() -> Template {
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_default();
    let feed = feed().await.unwrap_or_default();
    let devto = feed.get(&Source::DevTo).unwrap();
    let mokareads = feed.get(&Source::MoKa).unwrap();

    Template::render(
        "index",
        context! {
            version: version, 
            moka_articles: mokareads, 
            devto_articles: devto
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
