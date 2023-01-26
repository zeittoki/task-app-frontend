mod components;
mod models;
mod pages;
use crate::pages::{
  project::project as Project,
  home::home as Home,
  not_found::not_found as NotFound,
};

use dioxus::prelude::*;
use dioxus_router::{Route, Router};

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  log::debug!("Hello, world!");
  dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
  cx.render(rsx! {
    Router {
      Route { to: "/", Home {} }
      Route { to: "/index.html", Home {} }
      Route { to: "/project", Project {} }
      Route { to: "", NotFound {} }
    }
  })
}