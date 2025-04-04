use std::time::Duration;

use crate::app::Scene;
use crate::components::menu_bar::MenuBarComponent;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::message::Message;
use crate::models::scheduled_task::ScheduledTask;
use leptos::task::spawn_local;
use leptos::{logging, prelude::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn ScheduledTaskRegisterScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
) -> impl IntoView {
    let character = RwSignal::new("rena/hearing.png".to_string());
    let message = RwSignal::new(Message::new(
        "レーナ".to_string(),
        format!(
            "{}さん、定期業務の登録ですね。日時を指定してください。",
            hardworker.get().unwrap().name
        ),
    ));

    let title = RwSignal::new(String::new());
    let datetime = RwSignal::new(String::new());

    let submit_scheduled_task = move |_| {
        let task_data = serde_json::json!({"scheduled_task":  ScheduledTask {
            title: title.get(),
            scheduled_time: datetime.get()
        }});

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&task_data).unwrap();
            logging::log!("{:?}", task_data);
            let _ = invoke("register_scheduled_task", args).await;

            message.set(Message::new(
                "レーナ".to_string(),
                format!("定期依頼「{}」を登録しました！", title.get()),
            ));
            character.set("rena/register_success.png".to_string());

            set_timeout(
                move || {
                    message.set(Message::new(
                        "レーナ".to_string(),
                        "他に定期的な業務はありますか？".to_string(),
                    ));
                    character.set("rena/hearing.png".to_string());
                },
                Duration::from_secs(2),
            );

            title.set("".to_string());
            datetime.set("".to_string());
        });
    };

    view! {
        <div class="zentas-main">
            <MenuBarComponent scene=scene/>

            <img src="public/assets/backgrounds/guild_inside.png" class="zentas-bg" />
            <img src={move || format!("public/assets/characters/{}", character.get())} class="zentas-person" />
            <WindowMessage message={message}/>

            <div class="task-form">
                <div class="task-list-title">"定期依頼の登録"</div>
                <div class="task-form-input-list">
                    <div class="task-form-input">
                        <label>"依頼タイトル（必須）"</label>
                        <input
                            placeholder="例: 定例会議"
                            on:input=move |e| title.set(event_target_value(&e)) />
                    </div>
                    <div class="task-form-input">
                        <label>"通知日時（必須）"</label>
                        <input
                            type="datetime-local"
                            on:input=move |e| datetime.set(event_target_value(&e)) />
                    </div>
                </div>

                <div style="margin-top: 20px;">
                    <button style="background:rgba(90, 180, 120, 0.7);" on:click=submit_scheduled_task>
                        "定期依頼を登録"
                    </button>
                    <button style="background:rgba(220, 90, 90, 0.7); margin-left: 10px;" on:click=move |_| scene.set(Scene::Guild)>
                        "ギルドに戻る"
                    </button>
                </div>
            </div>
        </div>
    }
}
