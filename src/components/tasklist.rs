use dioxus::prelude::*;

use crate::{components::task::task as TaskComponent, models::task::Task};

pub fn tasklist(cx: Scope) -> Element {
    let content = use_future(&cx, (), |_| async move {
        reqwest::get("http://127.0.0.1:8080/tasks")
            .await
            .unwrap()
            .json::<Vec<Task>>()
            .await
    });

    cx.render(match content.value() {
        Some(Ok(data)) => rsx!(
          ul {
            class: "flex flex-col",
            data.iter().rev().map(|task| rsx! (
               TaskComponent { key: "{task.id}", task: task.clone() }
            ))
          }
        ),
        Some(Err(_)) => rsx! { "An error loading the resource occured." },
        None => rsx! { pre { "Loading..." } },
    })
}
