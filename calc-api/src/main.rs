use std::env;

use calc_api::parser::expr_parser;
use actix_web::{web, App, HttpServer, Result, error};
use derive_more::{Display, Error};
use serde::Deserialize;

pub mod ast;

#[derive(Deserialize)]
struct Expr {
    expr: String
}

#[derive(Debug, Display, Error)]
#[display(fmt = "parse error: {}", message)]
struct ParseError {
    message: &'static str,
}

impl error::ResponseError for ParseError {}

async fn index(json: web::Json<Expr>) -> Result<String, ParseError> {
    let result = expr_parser(&json.expr);

    if result.is_ok() {
        let (_, expr) = result.unwrap();
        Ok(expr.eval().to_string())
    } else {
        Err(ParseError { message: "式が正しくありません" })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: i32 = env::var("PORT")
        .unwrap_or_else(|_| "8088".to_string())
        .parse()
        .expect("PORT must be a number");

    let bind = format!("0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new()
            .route("/expr", web::get().to(index))
    })
    .bind(bind)?
    .run()
    .await
}