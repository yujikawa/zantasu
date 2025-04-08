use crate::models::task::{Task, TaskCreateDTO, TaskFormState};
use leptos::prelude::*;
use leptos::task::{self, spawn_local};
use shared::dto::task::{CreateTaskRequest, TaskResponse};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn OneShotTaskFormComponent<F, C>(
    // hardworker: RwSignal<Option<HardWorker>>,
    task: RwSignal<TaskFormState>,
    on_submit: F,
    submit_label: &'static str,
    on_cancel: C,
) -> impl IntoView
where
    F: Fn() + Clone + 'static,
    C: Fn() + Clone + 'static,
{
    view! {

        <div class="modal-backdrop">
            <div class="modal-content">
                <h2>"依頼の登録"</h2>
                <div class="task-form-input">
                    <label style="color: black">"依頼タイトル（必須）"</label>
                    <input
                        prop:value=move || task.get().title
                        placeholder="例: ゴブリン退治"
                        on:input=move |e| {
                            let mut t = task.get();
                            t.title = event_target_value(&e);
                            task.set(t);
                        }
                    />
                </div>
                <div class="task-form-input">
                    <label style="color: black">"依頼詳細（任意）"</label>
                    <textarea
                        prop:value=move || task.get().description
                        placeholder="例: 西の森で発生中"
                        on:input=move |e| {
                            let mut t = task.get();
                            t.description = event_target_value(&e);
                            task.set(t);
                        }
                    >{move || task.get().description}</textarea>
                </div>
                <div class="task-form-input">
                    <label style="color: black">"締切日（任意）"</label>
                    <input
                        type="date"
                        prop:value=move || task.get().due_date
                        placeholder="例: 2025-05-01"
                        on:input=move |e| {
                            let mut t = task.get();
                            t.due_date = event_target_value(&e);
                            task.set(t);
                        }
                    />
                </div>

                <div class="modal-buttons">
                    <button on:click=move |_| on_cancel()>
                        "キャンセル"
                    </button>

                    <button on:click=move |_| on_submit()>
                    {submit_label}
                    </button>
                </div>

            </div>


        </div>

    }
}
