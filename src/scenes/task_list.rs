use crate::app::Scene;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::task::Task;
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

    let message = RwSignal::new(format!(
        "{}さんへの依頼が確認できます。依頼が完了したら忘れずに報告してくださいね！",
        hardworker.get().unwrap().name
    ));

    fn select_task(character: RwSignal<String>, message: RwSignal<String>, selected_task: Task) {
        let new_message = match selected_task.description {
            Some(description) => {
                character.set("rena/explain_task.png".to_string());
                format!("依頼の詳細は..{}ということみたいです！", description)
            }
            None => {
                character.set("rena/explain_task_confused.png".to_string());
                format!("依頼の詳細は..無いみたいですね..")
            }
        };
        message.set(new_message);
    }

    view! {


            <div class="zentas-main">

            // === 背景 ===
            <img src="public/assets/backgrounds/guild_inside.png"
                class="zentas-bg" />

    // === 掲示板 ===
    <Show
    when=move || task_count.get() !=0
    fallback=|| ()>

    <img src="public/assets/objects/board_with_paper.png"
        style="position: absolute; left: 50px; top: 140px; width: 500px;"
        />

    </Show>

    <Show
    when=move || task_count.get() == 0
    fallback=|| ()>

    <img src="public/assets/objects/board.png"
        style="position: absolute; left: 50px; top: 140px; width: 500px;"
        />

    </Show>

            // === 受付嬢（立ち絵） ===
            <img src={move || format!("public/assets/characters/{}", character.get())}
                class="zentas-person" />
            // === セリフウィンドウ ===
            <WindowMessage message={ message }/>

        </div>
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
                                        tasks.update(|opt| {
                                            if let Some(list) = opt {
                                                list.retain(|t| t != &task_delete);
                                            }
                                        });
                                        spawn_local({
                                            let tasks = tasks.clone();
                                            async move {
                                                let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                                                    "tasks": tasks.get().clone().unwrap_or(vec![])
                                                })).unwrap();
                                                let _ = invoke("save_tasks", args).await;
                                                logging::log!("タスク完了で削除 & 保存しました");
                                            }
                                        });
                                        message.set(format!("「{}」の依頼をやらないんですね...わかりました。", task_delete.title));
                                        character.set("rena/delete.png".to_string());
                                    }
                                >
                                    "依頼削除"
                                </button>
                                <button class="task-complete"
                                    on:click=move |_| {
                                        tasks.update(|opt| {
                                            if let Some(list) = opt {
                                                list.retain(|t| t != &task_complete);
                                            }
                                        });
                                        spawn_local({
                                            let tasks = tasks.clone();
                                            async move {
                                                let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                                                    "tasks": tasks.get().clone().unwrap_or(vec![])
                                                })).unwrap();
                                                let _ = invoke("save_tasks", args).await;
                                                logging::log!("タスク完了で削除 & 保存しました");
                                            }
                                        });
                                        message.set(format!("「{}」の依頼を完了しましたね！おめでとうございます！", task_complete.title));
                                        character.set("rena/congrats.png".to_string());
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
            <button
            style="
            position: absolute; 
            right: 10px;
            top: 10px;
            transform: translateX(-50%);
            padding: 16px 32px;
            font-size: 24px;
            border-radius: 8px;
            background:rgba(220, 90, 90, 0.7);
        "
            on:click=move |_| scene.set(Scene::Guild)
            >
            "戻る"
            </button>
        }
}
