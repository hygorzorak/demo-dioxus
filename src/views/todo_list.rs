use dioxus::prelude::*;
use crate::components::Todo;

#[derive(Clone, PartialEq)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

#[component]
pub fn TodoList() -> Element {
    let mut next_id = use_signal(|| 0usize);
    let mut todos = use_signal(|| Vec::<TodoItem>::new());
    let mut new_todo = use_signal(|| String::new());

    let mut add_todo = move |_| {
        let text = new_todo.read().clone();
        let trimmed = text.trim();
        
        if !trimmed.is_empty() {
            todos.write().push(TodoItem {
                id: *next_id.read(),
                text: trimmed.to_string(),
                completed: false,
            });
            *next_id.write() += 1;
            new_todo.set(String::new());
        }
    };

    let toggle_todo = move |id: usize| {
        if let Some(todo) = todos.write().iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
    };

    let delete_todo = move |id: usize| {
        todos.write().retain(|todo| todo.id != id);
    };

    let clear_completed = move |_| {
        todos.write().retain(|todo| !todo.completed);
    };

    let active_count = todos.read().iter().filter(|todo| !todo.completed).count();
    let completed_count = todos.read().len() - active_count;

    rsx! {
        main {
            class: "container",
            
            h1 {
                class: "app-title",
                "Tasks"
            }
            
            div {
                class: "card",
                
                form {
                    class: "todo-form",
                    prevent_default: "onsubmit",
                    onsubmit: move |_| {
                        add_todo(());
                    },
                    
                    input {
                        class: "todo-input",
                        r#type: "text",
                        placeholder: "What needs to be done?",
                        value: "{new_todo.read()}",
                        oninput: move |event| {
                            new_todo.set(event.value().to_string());
                        }
                    }
                    
                    button {
                        class: "todo-button",
                        r#type: "submit",
                        "ADD"
                    }
                }
                
                div {
                    if todos.read().is_empty() {
                        div {
                            class: "empty-state",
                            "No tasks yet"
                        }
                    } else {
                        for todo in todos.read().iter() {
                            Todo {
                                key: "{todo.id}",
                                id: todo.id,
                                text: todo.text.clone(),
                                completed: todo.completed,
                                on_toggle: toggle_todo,
                                on_delete: delete_todo,
                            }
                        }
                    }
                }
                
                if !todos.read().is_empty() {
                    div {
                        class: "todo-footer",
                        
                        span {
                            class: "todo-count",
                            if active_count == 1 {
                                "{active_count} item left"
                            } else {
                                "{active_count} items left"
                            }
                        }
                        
                        if completed_count > 0 {
                            button {
                                class: "clear-button",
                                onclick: clear_completed,
                                "CLEAR COMPLETED"
                            }
                        }
                    }
                }
            }
            
            footer {
                class: "app-footer",
                "Todo App • Dioxus + Rust"
            }
        }
    }
} 