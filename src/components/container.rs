use dioxus::prelude::*;

#[derive(Props)]
pub struct ContainerProps<'a> {
  children: Element<'a>
}

pub fn container<'a>(cx: Scope<'a, ContainerProps<'a>>) -> Element {
  cx.render(rsx! {
    div {
      class: "min-h-screen p-4 font-semibold",

      &cx.props.children
    }
  })
}