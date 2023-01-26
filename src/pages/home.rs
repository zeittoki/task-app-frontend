use dioxus::prelude::*;
use dioxus_router::Link;
use crate::components::container::container as Container;

pub fn home(cx: Scope) -> Element {
  cx.render(rsx! {
      Container {
        div {
          class: "flex flex-col h-full items-center justify-center",

          "Home"

          Link {
            to: "/project"
            class: "block text-sm text-blue-700 underline font-semibold rounded transition duration-200",
            "Task Page"
          }
        }
      }
  })
}