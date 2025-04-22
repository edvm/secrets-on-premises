#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use tera::{Context, Tera};
use std::collections::HashMap;
use loco_rs::prelude::*;
use axum::debug_handler;

fn render_template(template_name: &str, context: &HashMap<&str, &str>) -> tera::Result<String> {
    let template_path = "templates/**/*";
    let tera = Tera::new(template_path)?;
    let mut tera_context = Context::new();
    for (key, value) in context {
        tera_context.insert(*key, *value);
    }
    tera.render(template_name, &tera_context)
}

#[debug_handler]
pub async fn encrypt() -> Result<Response> {
    let context = HashMap::new();
    let rendered = render_template("encrypt.html", &context)?;
    Ok(Response::new(rendered.into()))
}

#[debug_handler]
pub async fn decrypt() -> Result<Response> {
    let context = HashMap::new();
    let rendered = render_template("decrypt.html", &context)?;
    Ok(Response::new(rendered.into()))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/")
        .add("/", get(encrypt))
        .add("/decrypt", get(decrypt))
}