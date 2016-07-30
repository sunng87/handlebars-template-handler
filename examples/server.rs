extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate handlebars_template_handler as hbsh;

use std::error::Error;

use iron::prelude::*;
use router::Router;

use hbs::{HandlebarsEngine, DirectorySource};
use hbsh::TemplateHandler;

fn main() {
    let mut hbse = HandlebarsEngine::new();
    // add a directory source, all files with .hbs suffix will be loaded as template
    hbse.add(Box::new(DirectorySource::new("./examples/templates/", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }


    let mut router = Router::new();
    router.get("/", TemplateHandler::new("index"));
    let mut chain = Chain::new(router);
    chain.link_after(hbse);
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
