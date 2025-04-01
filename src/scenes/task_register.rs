use crate::app::Scene;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::task::Task;
use leptos::prelude::*;
use leptos::task::{self, spawn_local};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn TaskRegisterScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
    tasks: RwSignal<Option<Vec<Task>>>,
) -> impl IntoView {
    let message = RwSignal::new(format!(
        "{}さん、依頼の内容を教えてください。私のほうで依頼書を書きますので。",
        hardworker.get().unwrap().name
    ));

    let title = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let due_date = RwSignal::new(String::new());
    let title_ref = NodeRef::<leptos::html::Input>::new();
    let description_ref = NodeRef::<leptos::html::Textarea>::new();
    let due_date_ref = NodeRef::<leptos::html::Input>::new();
    Effect::new(move |_| {
        if let Some(input) = title_ref.get() {
            input.set_value(&title.get());
        }
        if let Some(textarea) = description_ref.get() {
            textarea.set_value(&description.get());
        }
        if let Some(input) = due_date_ref.get() {
            input.set_value(&due_date.get());
        }
    });
    let submit_task = move |_| {
        let new_task = Task {
            title: title.get(),
            description: if description.get().is_empty() {
                None
            } else {
                Some(description.get())
            },
            rank: "C".to_string(), // とりあえずランクは仮
            due_date: if due_date.get().is_empty() {
                None
            } else {
                Some(due_date.get())
            },
        };
        // タスクの新規登録
        tasks.update(|opt| {
            if let Some(list) = opt {
                list.push(new_task);
            }
        });
        spawn_local(async move {
            if let Some(ts) = tasks.get() {
                let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                    "tasks": ts
                }))
                .unwrap();
                let _ = invoke("save_tasks", args).await;
            }
        });

        message.set(format!("「{}」を登録しました！", title.get()));

        // 入力リセット
        title.set("".to_string());
        description.set("".to_string());
        due_date.set("".to_string());
    };

    view! {

        <div class="zentas-main">

        // === 背景 ===
        <img src="public/assets/backgrounds/guild_inside.png"
            class="zentas-bg" />

        // === 受付嬢（立ち絵） ===
        <img src="public/assets/characters/memo.png"
            class="zentas-person" />
        // === セリフウィンドウ ===
        <WindowMessage message={ message }/>

        //  依頼書
        <div class="task-form">
        <div class="task-list-title">"新しい依頼の登録"</div>
            <div class="task-form-input-list">
                <div class="task-form-input">
                    <label>"依頼タイトル（必須）"</label>
                    <input
                    node_ref=title_ref
                    placeholder="例: ゴブリン退治" on:input=move |e| title.set(event_target_value(&e)) />
                </div>
                <div class="task-form-input">
                    <label>"依頼詳細（任意）"</label>
                    <textarea
                    node_ref=description_ref

                    placeholder="例: 西の森で発生中" on:input=move |e| description.set(event_target_value(&e))>
                    {move || description.get()}
                    </textarea>
                </div>
                <div class="task-form-input">
                    <label>"締切日（任意）"</label>
                    <input
                    type="date"
                    node_ref=due_date_ref

                    placeholder="例: 2025-05-01" on:input=move |e| due_date.set(event_target_value(&e)) />
                </div>
            </div>

            <div style="margin-top: 20px;">
                <button style="background:rgba(220, 90, 90, 0.7);" on:click=move |_| scene.set(Scene::Guild)>"閉じる"</button>
                <button style="background:rgba(90, 116, 220, 0.7);margin-left:10px;" on:click=submit_task>"登録"</button>
            </div>
        </div>

    </div>


    }
}
