use lazy_static::*;
use serde::Serialize;
use std::default::Default;
use std::sync::Arc;
use tera::Tera;
use uuid::Uuid;
use warp::http::StatusCode;

use crate::state;

lazy_static! {
    static ref TEMPLATE: Tera = {
        let mut template: Tera = Default::default();
        template
            .add_raw_template("app.html", include_str!("../frontend/app.html"))
            .unwrap();
        template
    };
}

#[derive(Serialize)]
struct Context<'a> {
    pub title: &'a str,
    pub buttons: Vec<Button<'a>>,
}

#[derive(Serialize)]
struct Button<'a> {
    pub uuid: &'a Uuid,
    pub label: &'a str,
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

    let buttons: Vec<Button<'_>> = state
        .buttons
        .iter()
        .map(|(id, button)| Button {
            label: &button.label,
            uuid: id,
        })
        .collect();

    let state = match tera::Context::from_serialize(&Context {
        title: &state.title,
        buttons,
    }) {
        Ok(state) => state,
        Err(_) => return Ok(error()),
    };

    TEMPLATE.render("app.html", &state).map_or_else(
        |_| Ok(error()),
        |page| Ok(Box::new(warp::reply::html(page)) as BoxReply),
    )
}
