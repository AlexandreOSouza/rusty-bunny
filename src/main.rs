#![feature(proc_macro_hygiene, decl_macro)]

use rocket::response::Redirect;
mod utils;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let command = utils::get_command_from_query_string(&cmd);
    let redirect_url = match command {
        "gh" => utils::github::construct_github_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd),
    };
    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}
