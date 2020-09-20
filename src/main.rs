mod index;
mod manifest;
mod press;
mod state;
mod static_;

use anyhow::Error;
use std::path::PathBuf;
use std::process;
use std::sync::Arc;
use structopt::StructOpt;
use uuid::Uuid;
use warp::Filter;

use crate::state::State;

#[derive(StructOpt)]
#[structopt(about = "button service")]
pub struct Args {
    #[structopt(name = "config", help = "Configuration file path")]
    pub config_path: PathBuf,
}

fn run() -> Result<(), Error> {
    let args = Args::from_args();
    let state = Arc::new(State::from_path(&args.config_path)?);

    let routes = {
        let state = state.clone();
        let state = warp::any().map(move || state.clone());

        let index = warp::filters::path::end()
            .and(warp::get())
            .and(state.clone())
            .and_then(index::handle);
        let manifest = warp::path("app.webmanifest")
            .and(warp::filters::path::end())
            .and(warp::get())
            .and(state.clone())
            .and_then(manifest::handle);
        let static_ = warp::filters::path::tail()
            .and(warp::get())
            .and_then(static_::handle);
        let press = warp::path("press")
            .and(warp::post())
            .and(state)
            .and(warp::path::param::<Uuid>())
            .and(warp::filters::path::end())
            .and_then(press::handle);
        index.or(manifest).or(static_).or(press)
    };

    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()?;

    rt.block_on(async {
        println!("listening on {}", state.address);
        warp::serve(routes).run(state.address).await
    });

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("buttons: {}", err);
        process::exit(1);
    }
}
