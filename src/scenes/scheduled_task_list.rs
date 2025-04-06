use std::time::Duration;

use crate::app::Scene;
use crate::components::board::BoardComponent;
use crate::components::menu_bar::MenuBarComponent;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::message::Message;
use crate::models::task::ScheduledTask;
use crate::models::task::{DeleteTaskRequest, Task};
use leptos::task::{self, spawn_local};
use leptos::{logging, prelude::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn ScheduledTaskListScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
    tasks: RwSignal<Option<Vec<ScheduledTask>>>,
) -> impl IntoView {
    let character = RwSignal::new("rena/watching.png".to_string());
    let selected_task_id = RwSignal::new(None::<String>);

    let message = RwSignal::new(Message::new(
        "レーナ".to_string(),
        format!(
            "{}さんへの定期依頼の設定が確認できます。定期依頼が不要な場合は設定を削除してくださいね",
            hardworker.get().unwrap().name
        ),
    ));

    fn select_task(
        character: RwSignal<String>,
        message: RwSignal<Message>,
        selected_task: ScheduledTask,
        selected_task_id: RwSignal<Option<String>>,
    ) {
        selected_task_id.set(Some(selected_task.id.clone()));

        let new_text = match selected_task.task.description {
            Some(description) => {
                character.set("rena/explain_task.png".to_string());
                format!("依頼の詳細は..{}ということみたいです！", description)
            }
            None => {
                character.set("rena/explain_task_confused.png".to_string());
                format!("依頼の詳細は..無いみたいですね..")
            }
        };

        let new_message = Message::new("レーナ".to_string(), new_text);

        message.set(new_message);
    }

    view! {


        <div class="zentas-main">
        <MenuBarComponent scene=scene/>

        // === 背景 ===
        <img src="public/assets/backgrounds/guild_inside.png"
            class="zentas-bg" />

        // // === 掲示板 ===
        // <BoardComponent tasks=tasks/>


        // === 受付嬢（立ち絵） ===
        <img src={move || format!("public/assets/characters/{}", character.get())}
            class="zentas-person" />
        // === セリフウィンドウ ===
        <WindowMessage message={ message }/>

        // タスク一覧
        <div class="task-list-main">
            <div class="task-list-window">
                <div class="task-list-title">"定期依頼設定一覧"</div>
                <div class="task-list-scroll">
                    <For
                        each=move || tasks.get().unwrap()
                        key=|task| task.clone() // task自体がキー
                        children=move |task| {
                            let task_select = task.clone();
                            let task_delete = task.clone();

                            view! {
                                <div class=move || {
                                    if selected_task_id.get() == Some(task.id.clone()) {
                                        "task-item task-selected"
                                    } else {
                                        "task-item"
                                    }
                                }
                                on:click=move |_| select_task(character, message, task_select.clone(), selected_task_id)>
                                    <div class="task-item-basic">
                                    <div>{ task.task.title.clone() }</div>
                                    <div>{ task.pattern.to_pattern_string().clone()}</div>
                                    </div>

                                    <div class="task-operation-buttons">
                                    <button class="task-delete"
                                    on:click=move |e| {
                                        e.stop_propagation();
                                        spawn_local({
                                            let task_id = task_delete.id.clone();
                                            async move {
                                                let request = DeleteTaskRequest {task_id};
                                                let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                                                    "dto": &request
                                                })).unwrap();
                                                let result = invoke("delete_scheduled_task", args).await;
                                                match serde_wasm_bindgen::from_value::<Vec<ScheduledTask>>(result) {
                                                    Ok(current_tasks) => tasks.set(Some(current_tasks)),
                                                    Err(e) => logging::log!("{:?}", e)
                                                }
                                            }
                                        });

                                        // Reaction
                                        message.set(
                                            Message::new("レーナ".to_string(),  format!("「{}」の定期依頼を削除しました！", task_delete.task.title)
                                        ));
                                        character.set("rena/delete.png".to_string());
                                        set_timeout(
                                            move || {
                                                message.set(Message::new(
                                                    "レーナ".to_string(),
                                                    format!("ほかに調整したい定期依頼があれば引き続き伺います!"),
                                                ));
                                                character.set("rena/watching.png".to_string());
                                            },
                                            Duration::from_secs(3),
                                        );
                                    }
                                >
                                    "設定削除"
                                </button>

                                </div>
                                </div>
                        }
                    }
                    />
                </div>
            </div>

        </div>
        // タスク一覧終了
    </div>


    }
}
