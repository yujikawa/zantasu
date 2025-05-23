use std::time::Duration;

use crate::app::Scene;
use crate::components::board::BoardComponent;
use crate::components::menu_bar::MenuBarComponent;
use crate::components::oneshot_task_form::OneShotTaskFormComponent;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::message::Message;
use crate::models::task::{DeleteTaskRequest, Task, TaskFormState};
use leptos::task::{self, spawn_local};
use leptos::{logging, prelude::*};
use shared::dto::task::TaskResponse;
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
    let character = RwSignal::new("receptionist/explain1.png".to_string());
    let selected_task_id = RwSignal::new(None::<String>);
    let selected_edit_task = RwSignal::new(None::<Task>);
    let task_form_state = RwSignal::new(TaskFormState::new());
    let on_cancel = move || selected_edit_task.set(None);
    let on_submit = move || {
        let new_task = Task {
            id: selected_edit_task.get().unwrap().id,
            title: task_form_state.get().title,
            description: if task_form_state.get().description.is_empty() {
                None
            } else {
                Some(task_form_state.get().description)
            },
            due_date: if task_form_state.get().due_date.is_empty() {
                None
            } else {
                Some(task_form_state.get().due_date)
            },
        };
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "task": new_task
            }))
            .unwrap();
            let result = invoke("update_task_command", args).await;
            if let Ok(new_tasks) = serde_wasm_bindgen::from_value::<Vec<Task>>(result) {
                // タスクの新規登録
                tasks.set(Some(new_tasks));
                // selected_edit_task.set(None);
            }
        });
    };
    let message = RwSignal::new(Message::new(
        "ギルド受付嬢".to_string(),
        format!(
            "{}さんへの依頼が確認できます。依頼が完了したら忘れずに報告してくださいね！",
            hardworker.get().unwrap().name
        ),
    ));

    fn select_task(
        character: RwSignal<String>,
        message: RwSignal<Message>,
        selected_task: Task,
        selected_task_id: RwSignal<Option<String>>,
    ) {
        selected_task_id.set(Some(selected_task.id.clone()));

        let new_text = match selected_task.description {
            Some(description) => {
                character.set("receptionist/explain_task.png".to_string());
                format!(
                    "依頼タイトルの「{}」の詳細は..{}ということみたいです！",
                    selected_task.title, description
                )
            }
            None => {
                character.set("receptionist/explain_task_confused.png".to_string());
                format!(
                    "依頼タイトルの「{}」の詳細は..無いみたいですね..",
                    selected_task.title
                )
            }
        };

        let new_message = Message::new("ギルド受付嬢".to_string(), new_text);

        message.set(new_message);
    }

    view! {


        <div class="zantas-main">
        <MenuBarComponent scene=scene/>

        // === 背景 ===
        <img src="public/assets/backgrounds/guild_inside.png"
            class="zantas-bg" />

        // === 掲示板 ===
        <BoardComponent tasks=tasks/>


        // === 受付嬢（立ち絵） ===
        <img src={move || format!("public/assets/characters/{}", character.get())}
            class="zantas-person" />
        // === セリフウィンドウ ===
        <WindowMessage message={ message }/>

        // タスク一覧
        <div class="task-list-main">
            <div class="task-list-window">
                <div class="task-list-title">"依頼一覧"</div>
                <div class="task-list-scroll">
                    <For
                        each=move || tasks.get().unwrap()
                        key=|task| task.clone() // task自体がキー
                        children=move |task| {
                            let task_select = task.clone();
                            let task_edit = task.clone();
                            let task_delete = task.clone();
                            let task_complete = task.clone();

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
                                    <div class="task-item-title">{ task.title.clone() }</div>
                                    <div>{task.due_date.clone().unwrap_or("締切未定".into())}</div>
                                    </div>

                                    <div class="task-operation-buttons">
                                    <button class="task-edit" on:click=move |_| {

                                        selected_edit_task.set(Some(task_edit.clone()));
                                        let t = task_edit.clone();
                                        task_form_state.set(TaskFormState {
                                            title: t.title,
                                            description: t.description.unwrap_or("".to_string()),
                                            due_date: t.due_date.unwrap_or("".to_string())
                                        });
                                    }>
                                        "編集"
                                    </button>

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
                                                let result = invoke("delete_task", args).await;
                                                match serde_wasm_bindgen::from_value::<Vec<Task>>(result) {
                                                    Ok(current_tasks) => tasks.set(Some(current_tasks)),
                                                    Err(e) => logging::log!("{:?}", e)
                                                }
                                            }
                                        });

                                        // Reaction
                                        message.set(
                                            Message::new("ギルド受付嬢".to_string(),  format!("「{}」の依頼をやらないんですね...わかりました。", task_delete.title)
                                        ));
                                        character.set("receptionist/delete.png".to_string());
                                        set_timeout(
                                            move || {
                                                message.set(Message::new(
                                                    "ギルド受付嬢".to_string(),
                                                    format!("ほかに報告したい依頼があれば引き続き伺います!"),
                                                ));
                                                character.set("receptionist/explain1.png".to_string());
                                            },
                                            Duration::from_secs(3),
                                        );
                                    }
                                >
                                    "依頼削除"
                                </button>
                                <button class="task-complete"
                                    on:click=move |e| {
                                        e.stop_propagation();

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
                                            Message::new("ギルド受付嬢".to_string(),  format!("「{}」の依頼を完了しましたね！おめでとうございます！", task_complete.title)
                                        ));
                                        character.set("receptionist/congrats.png".to_string());
                                        set_timeout(
                                            move || {
                                                message.set(Message::new(
                                                    "ギルド受付嬢".to_string(),
                                                    format!("ほかに報告したい依頼があれば引き続き伺います!"),
                                                ));
                                                character.set("receptionist/explain1.png".to_string());
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
        // タスク一覧終了
    </div>
    // 編集モーダルの描画

    <Show
    when=move || selected_edit_task.get().is_some()
    fallback=|| ()>
        <OneShotTaskFormComponent on_submit=on_submit submit_label="依頼を更新する" on_cancel=on_cancel task=task_form_state/>
    </Show>

    }
}
