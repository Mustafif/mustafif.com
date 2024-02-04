use rocket::serde::{Deserialize, Serialize};
const PATH: &str = "projects.toml";
use rocket::tokio::fs::read_to_string;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub start: String,
    pub end: Option<String>,
    pub technologies: Vec<String>,
    pub description: String,
    pub gh: String,
    pub https: Option<String>,
    pub subprojects: Vec<SubProject>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SubProject {
    pub name: String,
    pub start: String,
    pub end: Option<String>,
    pub technologies: Vec<String>,
    pub description: String,
    pub gh: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Projects {
    pub projects: Vec<Project>,
}

impl Project {
    pub fn change_end(&mut self) {
        if self.end.is_none() {
            self.end = Some("Present".to_owned());
        }
        let _ = self
            .subprojects
            .iter_mut()
            .map(|x| x.change_end())
            .collect::<()>();
    }
}

impl SubProject {
    fn change_end(&mut self) {
        if self.end.is_none() {
            self.end = Some("Present".to_owned());
        }
    }
}

pub async fn projects() -> std::io::Result<Projects> {
    let str = read_to_string(PATH).await?;
    Ok(toml::from_str(&str).unwrap())
}
