use dioxus::prelude::*;
use serde_json::json;

use crate::models::task::{NewTask, Task};
use crate::components::task::task as TaskComponent;

pub fn taskfrom(cx: Scope) -> Element {
    let task_name = use_state(cx, || String::new());

    let tasks = use_ref(cx, Vec::<Task>::new);
    let tasks_lock = tasks.read();
    let tasks_rendered = tasks_lock.iter().rev().map(|task| {
        rsx!( TaskComponent {
          key: "{task.id}",
          task: task.clone()
      })
    });

    cx.render(rsx! (
      div {
        class: "border-b-2 border-red-700 gap-4",

        div {
          class: "flex flex-col justify-center p-2 bg-red-100 gap-4",
    
          div {
            label {
              r#for: "task_name",
              class: "block mb-2 text-sm font-medium text-gray-900",
              "Task"
            }
    
            div {
              class: "flex gap-4",
    
              input {
                id: "task_name",
                r#type: "text",
                class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5",
                placeholder: "ex. go shopping",
                required: true,
                value: "{ task_name }",
                oninput: move |e| task_name.set(e.value.clone()),
              }
      
              button {
                r#type: "button",
                class: "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-24 sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                onclick: move |_| cx.spawn({
                  let task_name = task_name.clone();
                  let tasks = tasks.clone();
                  async move {
                    let res = post_task(task_name.to_string()).await;
        
                    if let Ok(task) = res {
                      tasks.write().push(Task {
                        id: task.id,
                        name: task.name,
                        completed: task.completed
                      });
                    };
        
                    task_name.set(String::new());
                }}),
                "Submit"
              }
            }
        }
      }
      ul {
        tasks_rendered
      }
    }
  ))
}


async fn post_task(name: String) -> reqwest::Result<Task> {
  let post_body = json!(NewTask { name: name });
  let client = reqwest::Client::new();
  let task = client.post("http://127.0.0.1:8080/task")
    .json(&post_body)
    .send()
    .await?
    .json::<Task>()
    .await?;

  Ok(task)
}