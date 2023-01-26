use dioxus::prelude::*;

use crate::models::task::Task;

#[derive(Props, PartialEq)]
pub struct TaskProps {
    task: Task,
}

pub fn task(cx: Scope<TaskProps>) -> Element {
    let is_open = use_state(cx, || false);

    cx.render(rsx! {
      li {
        class: "bg-white my-2 shadow-lg",
        onclick: move |_| {
          is_open.set(!is_open);
        },

        h2 {
          class: "flex flex-row items-center gap-4 font-semibold p-3 cursor-pointer",

          input {
            r#type: "checkbox",
            checked: "{ cx.props.task.completed }"
          }

          span {
            class: if cx.props.task.completed { "line-through overflow-x-auto" } else { "overflow-x-auto" },
            "{ cx.props.task.name }"
          }
        }
        div {
          class: if **is_open {"border-l-2 border-purple-600 overflow-hidden max-h-full"} else { "overflow-hidden max-h-0" },

          p {
            class: "p-3 text-gray-900",
            "Shipping time is set by our delivery partners, according to the delivery method chosen by you. Additional details can be found in the order confirmation"
          }
        }
      }
    })
}

