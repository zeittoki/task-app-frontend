use dioxus::prelude::*;
// use dioxus_router::Link;
use crate::components::{taskform::taskfrom as TaskForm, tasklist::tasklist as TaskList};

pub fn project(cx: Scope) -> Element {
  cx.render(rsx! {
    div {
      class: "flex flex-col p-4 max-w-5xl mx-auto",

      TaskForm {}

      TaskList {}
    }
  })
}