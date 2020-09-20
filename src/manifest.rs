use lazy_static::*;
use serde::Serialize;
use std::default::Default;
use std::sync::Arc;
use tera::Tera;
use warp::http::StatusCode;

use crate::state;

lazy_static! {
    static ref TEMPLATE: Tera = {
        let mut template: Tera = Default::default();
        template
            .add_raw_template(
                "app.webmanifest",
                include_str!("../frontend/app.webmanifest"),
            )
            .unwrap();
        template
    };
}

#[derive(Serialize)]
struct Context<'a> {
    pub title: &'a str,
}

type BoxReply = Box<dyn warp::reply::Reply>;

pub async fn handle(
    state: Arc<state::State>,
) -> Result<Box<dyn warp::reply::Reply>, warp::reject::Rejection> {
    let error = || {
        Box::new(warp::reply::with_status(
            warp::reply::reply(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )) as BoxReply
    };

    let title = state.title.replace(r#"""#, r#"\""#);
    let state = match tera::Context::from_serialize(&Context { title: &title }) {
        Ok(state) => state,
        Err(_) => return Ok(error()),
    };

    TEMPLATE.render("app.webmanifest", &state).map_or_else(
        |_| Ok(error()),
        |manifest| {
            Ok(Box::new(warp::reply::with_header(
                manifest,
                "Content-Type",
                "application/manifest+json",
            )) as BoxReply)
        },
    )
}
