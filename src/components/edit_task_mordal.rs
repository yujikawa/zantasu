use leptos::prelude::*;

use crate::models::task::Task;
use leptos::task::{self, spawn_local};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn EditTaslMordalComponent(
    tasks: RwSignal<Option<Vec<Task>>>,
    task: RwSignal<Option<Task>>,
) -> impl IntoView {
    let update_task_command = move || {
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "task": task.get()
            }))
            .unwrap();
            let result = invoke("update_task_command", args).await;
            if let Ok(new_tasks) = serde_wasm_bindgen::from_value::<Vec<Task>>(result) {
                // タスクの新規登録
                tasks.set(Some(new_tasks));
                task.set(None);
            }
        });
    };

    view! {
        <div class="modal-backdrop">
        <div class="modal-content">
            <h2>"依頼の編集"</h2>
            <input
                type="text"
                prop:value=task.get().unwrap().title.clone()
                on:input=move |ev| {
                    let mut new = task.get().unwrap().clone();
                    new.title = event_target_value(&ev);
                    task.set(Some(new));
                }
            />
            <textarea
                prop:value=task.get().unwrap().description.clone()
                on:input=move |ev| {
                    let mut new = task.get().unwrap().clone();
                    new.description = Some(event_target_value(&ev));
                    task.set(Some(new));
                }
            />
            <input
                type="date"
                prop:value=task.get().unwrap().due_date.clone().unwrap_or_default()
                on:input=move |ev| {
                    let mut new = task.get().unwrap().clone();
                    new.due_date = Some(event_target_value(&ev));
                    task.set(Some(new));
                }
            />

            <div class="modal-buttons">
                <button on:click=move |_| task.set(None)>
                "キャンセル"
                </button>
                <button on:click=move |_| update_task_command()>"保存"</button>
            </div>
        </div>
    </div>
    }
}
