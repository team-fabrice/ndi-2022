use askama::Template;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        index,
    ]
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    message: &'static str,
}

#[get("/")]
fn index() -> IndexTemplate {
    IndexTemplate {
        message: "It works!"
    }
}
