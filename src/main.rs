#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;

use rocket_contrib::Template;
use rocket::request::FromParam;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct Empty {}

impl Empty {
    fn new() -> Self {
        Empty {}
    }
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &Empty::new())
}

#[get("/projects")]
fn projects_index() -> Template {
    Template::render("projects", &Empty::new())
}

pub struct ProjectName(PathBuf);

impl<'a> FromParam<'a> for ProjectName {
    type Error = &'a str;

    fn from_param(param: &'a rocket::http::RawStr) -> Result<Self, &'a str> {
        lazy_static! {
            static ref PROJECT_NAME_RE: Regex = Regex::new(r"^[A-z0-9\-]+$").unwrap();
        }
        let param: &str = param.as_ref();
        let path = PathBuf::from("projects").join(param);
        println!("path = {:?}", path);
        if PROJECT_NAME_RE.is_match(param) &&
           Path::new("templates")
               .join(&path)
               .with_extension("tera")
               .exists() {
            Ok(ProjectName(path))
        } else {
            Err(param)
        }
    }
}

#[get("/projects/<name>")]
fn projects(name: ProjectName) -> Template {
    Template::render(name.0.to_str().unwrap().to_owned(), &Empty::new())
}

#[get("/blog")]
fn blog_index() -> Template {
    Template::render("blog", &Empty::new())
}

#[get("/organizations")]
fn organizations_index() -> Template {
    Template::render("organizations", &Empty::new())
}


#[error(404)]
fn not_found() -> Template {
    Template::render("error404", &Empty::new())
}

#[error(500)]
fn internal() -> Template {
    Template::render("error500", &Empty::new())
}


fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/",
               routes![index,
                       projects_index,
                       projects,
                       blog_index,
                       organizations_index])
        .catch(errors![not_found, internal])
        .launch();
}
