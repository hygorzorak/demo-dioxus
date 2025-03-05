use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TodoProps {
    pub id: usize,
    pub text: String,
    pub completed: bool,
    pub on_toggle: EventHandler<usize>,
    pub on_delete: EventHandler<usize>,
}

#[component]
pub fn Todo(props: TodoProps) -> Element {
    let TodoProps {
        id,
        text,
        completed,
        on_toggle,
        on_delete,
    } = props;

    rsx! {
        div {
            class: "todo-item",
            
            // Checkbox
            input {
                class: "todo-checkbox",
                r#type: "checkbox",
                checked: "{completed}",
                onclick: move |_| on_toggle.call(id),
            }
            
            // Todo text
            p {
                class: if completed { "todo-text completed" } else { "todo-text" },
                "{text}"
            }
            
            // Delete button
            button {
                class: "todo-delete",
                onclick: move |_| on_delete.call(id),
                "×"
            }
        }
    }
} 