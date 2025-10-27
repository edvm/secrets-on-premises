use axum::{extract::State, response::{Html, IntoResponse, Response}};
use std::collections::HashMap;
use tera::{Context, Tera};

use crate::error::Result;
use crate::state::AppState;

fn render_template(template_name: &str, context: &HashMap<&str, &str>) -> Result<String> {
    let template_path = "templates/**/*";
    let tera = Tera::new(template_path)?;
    let mut tera_context = Context::new();
    for (key, value) in context {
        tera_context.insert(*key, *value);
    }
    Ok(tera.render(template_name, &tera_context)?)
}

pub async fn encrypt(State(state): State<AppState>) -> Response {
    let server_host = &state.config.server_url();
    let mut context = HashMap::new();
    context.insert("server_host", server_host.as_str());
    let rendered = match render_template("encrypt.html", &context) {
        Ok(r) => r,
        Err(e) => return e.into_response(),
    };
    Html(rendered).into_response()
}

pub async fn decrypt(State(state): State<AppState>) -> Response {
    let server_host = &state.config.server_url();
    let mut context = HashMap::new();
    context.insert("server_host", server_host.as_str());
    let rendered = match render_template("decrypt.html", &context) {
        Ok(r) => r,
        Err(e) => return e.into_response(),
    };
    Html(rendered).into_response()
}
