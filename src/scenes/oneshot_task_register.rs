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

        <OneShotTaskFormComponent task=task_form_state on_submit=on_submit submit_label="依頼登録する" on_cancel=on_cancel />
    //     <div class="zantas-main">
    //     <MenuBarComponent scene=scene/>

    //     // === 背景 ===
    //     <img src="public/assets/backgrounds/guild_inside.png"
    //         class="zantas-bg" />

    //     // === 受付嬢（立ち絵） ===
    //     <img src={move || format!("public/assets/characters/{}", character.get())}
    //         class="zantas-person" />
    //     // === セリフウィンドウ ===
    //     <WindowMessage message={ message }/>

    //     //  依頼書
    //     <div class="task-form">
    //     <div class="task-list-title">"新しい依頼の登録"</div>
    //         <div class="task-form-input-list">
    //             <div class="task-form-input">
    //                 <label>"依頼タイトル（必須）"</label>
    //                 <input
    //                 prop:value=move || title.get()
    //                 placeholder="例: ゴブリン退治" on:input=move |e| title.set(event_target_value(&e)) />
    //             </div>
    //             <div class="task-form-input">
    //                 <label>"依頼詳細（任意）"</label>
    //                 <textarea
    //                 prop:value=move || description.get()
    //                 placeholder="例: 西の森で発生中" on:input=move |e| description.set(event_target_value(&e))>
    //                 {move || description.get()}
    //                 </textarea>
    //             </div>
    //             <div class="task-form-input">
    //                 <label>"締切日（任意）"</label>
    //                 <input
    //                 type="date"
    //                 prop:value=move || due_date.get()
    //                 placeholder="例: 2025-05-01" on:input=move |e| due_date.set(event_target_value(&e)) />
    //             </div>
    //         </div>

    //         <div style="margin-top: 20px;">
    //             <button style="background:rgba(220, 90, 90, 0.7);margin-right:10px;" on:click=move |_| scene.set(Scene::TaskRegister)>"依頼選択に戻る"</button>
    //             <button style="background:rgba(90, 180, 120, 0.7);" on:click=submit_task>"単発依頼を登録"</button>
    //         </div>
    //     </div>

    // </div>


    }
}
