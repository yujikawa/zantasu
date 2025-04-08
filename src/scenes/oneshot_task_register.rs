use std::time::Duration;

use crate::app::Scene;
use crate::components::menu_bar::MenuBarComponent;
use crate::components::oneshot_task_form::OneShotTaskFormComponent;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::message::Message;
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
pub fn OneShotTaskRegisterScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
    tasks: RwSignal<Option<Vec<Task>>>,
) -> impl IntoView {
    let character = RwSignal::new("receptionist/hearing.png".to_string());
    let message = RwSignal::new(Message::new(
        "ギルド受付嬢".to_string(),
        format!(
            "{}さん、依頼の内容を教えてください。私のほうで依頼書を書きますので。",
            hardworker.get().unwrap().name
        ),
    ));
    let task_form_state = RwSignal::new(TaskFormState::new());
    // let title = RwSignal::new(String::new());
    // let description = RwSignal::new(String::new());
    // let due_date = RwSignal::new(String::new());
    let on_cancel = move || scene.set(Scene::TaskRegister);
    let on_submit = move || {
        let new_task = TaskCreateDTO::new(
            task_form_state.get().title,
            if task_form_state.get().description.is_empty() {
                None
            } else {
                Some(task_form_state.get().description)
            },
            if task_form_state.get().due_date.is_empty() {
                None
            } else {
                Some(task_form_state.get().due_date)
            },
        );

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "task": new_task
            }))
            .unwrap();
            let task = invoke("save_task", args).await;
            if let Ok(task) = serde_wasm_bindgen::from_value::<Task>(task) {
                // タスクの新規登録
                tasks.update(|opt| {
                    if let Some(list) = opt {
                        list.push(task.clone());
                    }
                });
            }
        });

        message.set(Message::new(
            "ギルド受付嬢".to_string(),
            format!("「{}」を登録しました！", task_form_state.get().title),
        ));
        character.set("receptionist/register_success.png".to_string());

        set_timeout(
            move || {
                message.set(Message::new(
                    "ギルド受付嬢".to_string(),
                    format!("ほかに依頼があれば引き続き伺います!"),
                ));
                character.set("receptionist/hearing.png".to_string());
            },
            Duration::from_secs(2),
        );
        // 入力リセット
        task_form_state.set(TaskFormState::new());
    };

    view! {
        <div class="zantas-main">
        // === 背景 ===
        <img src="public/assets/backgrounds/table.png"
            class="zantas-bg" />
        </div>
        <OneShotTaskFormComponent task=task_form_state on_submit=on_submit submit_label="依頼登録する" on_cancel=on_cancel />

    }
}
