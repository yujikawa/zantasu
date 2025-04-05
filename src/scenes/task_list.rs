use std::time::Duration;

use crate::app::Scene;
use crate::components::board::BoardComponent;
use crate::components::menu_bar::MenuBarComponent;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::message::Message;
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
pub fn TaskListScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
    tasks: RwSignal<Option<Vec<Task>>>,
) -> impl IntoView {
    let character = RwSignal::new("rena/watching.png".to_string());
    let task_count = RwSignal::new(tasks.get().unwrap().len());

    let message = RwSignal::new(Message::new(
        "レーナ".to_string(),
        format!(
            "{}さんへの依頼が確認できます。依頼が完了したら忘れずに報告してくださいね！",
            hardworker.get().unwrap().name
        ),
    ));

    fn select_task(character: RwSignal<String>, message: RwSignal<Message>, selected_task: Task) {
        let new_text = match selected_task.description {
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

        // === 掲示板 ===
        <BoardComponent tasks=tasks/>


        // === 受付嬢（立ち絵） ===
        <img src={move || format!("public/assets/characters/{}", character.get())}
            class="zentas-person" />
        // === セリフウィンドウ ===
        <WindowMessage message={ message }/>

        // タスク一覧
        <div class="task-list-window">
            <div class="task-list-title">"依頼一覧"</div>
            <div class="task-list-scroll">
                <For
                    each=move || tasks.get().unwrap()
                    key=|task| task.clone() // task自体がキー
                    children=move |task| {
                        let task_select = task.clone();
                        let task_delete = task.clone();
                        let task_complete = task.clone();

                        view! {
                            <div class="task-item"
                            on:click=move |_| select_task(character, message, task_select.clone())
                            >
                                <div class="task-item-basic">
                                <div>{task.title.clone()}</div>
                                <div>{task.due_date.clone().unwrap_or("締切未定".into())}</div>
                                </div>
                                <div class="task-operation-buttons">
                                <button class="task-delete"
                                on:click=move |_| {
                                    spawn_local({
                                        let task_id = task_delete.id.clone();
                                        async move {
                                            let request = DeleteTaskRequest {task_id};
                                            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                                                "dto": &request
                                            })).unwrap();
                                            let result = invoke("delete_task", args).await;
                                            match serde_wasm_bindgen::from_value::<Vec<Task>>(result) {
                                                Ok(current_tasks) => tasks.set(Some(current_tasks)),
                                                Err(e) => logging::log!("{:?}", e)
                                            }
                                        }
                                    });

                                    // Reaction
                                    message.set(
                                        Message::new("レーナ".to_string(),  format!("「{}」の依頼をやらないんですね...わかりました。", task_delete.title)
                                    ));
                                    character.set("rena/delete.png".to_string());
                                    set_timeout(
                                        move || {
                                            message.set(Message::new(
                                                "レーナ".to_string(),
                                                format!("ほかに報告したい依頼があれば引き続き伺います!"),
                                            ));
                                            character.set("rena/watching.png".to_string());
                                        },
                                        Duration::from_secs(3),
                                    );
                                }
                            >
                                "依頼削除"
                            </button>
                            <button class="task-complete"
                                on:click=move |_| {
                                    spawn_local({
                                        let task_id = task_complete.id.clone();
                                        async move {
                                            let request = DeleteTaskRequest {task_id};
                                            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                                                "dto": &request
                                            })).unwrap();
                                            let result = invoke("complete_task", args).await;
                                            match serde_wasm_bindgen::from_value::<Vec<Task>>(result) {
                                                Ok(current_tasks) => tasks.set(Some(current_tasks)),
                                                Err(e) => logging::log!("{:?}", e)
                                            }

                                            let result = invoke("get_hardworker", JsValue::NULL).await;
                                            if let Ok(hw) = serde_wasm_bindgen::from_value::<HardWorker>(result) {
                                                logging::log!("ハードワーカーをLOADしました");
                                                hardworker.set(Some(hw));
                                            }
                                        }
                                    });
                                    // reaction
                                    message.set(
                                        Message::new("レーナ".to_string(),  format!("「{}」の依頼を完了しましたね！おめでとうございます！", task_complete.title)
                                    ));
                                    character.set("rena/congrats.png".to_string());
                                    set_timeout(
                                        move || {
                                            message.set(Message::new(
                                                "レーナ".to_string(),
                                                format!("ほかに報告したい依頼があれば引き続き伺います!"),
                                            ));
                                            character.set("rena/watching.png".to_string());
                                        },
                                        Duration::from_secs(3),
                                    );
                                }
                            >
                                "達成報告"
                            </button>
                            </div>

                            </div>
                    }
                }
                />
            </div>
        </div>
    </div>


    }
}
