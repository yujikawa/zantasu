use std::time::Duration;

use crate::app::Scene;
use crate::components::menu_bar::MenuBarComponent;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::message::Message;
use crate::models::task::Task;
use leptos::task::spawn_local;
use leptos::{logging, prelude::*};
use wasm_bindgen::prelude::*;

use crate::models::scheduled_task::{PatternDTO, ScheduledTaskDTO};

#[derive(Clone, Debug, PartialEq)]
enum RepeatType {
    Monthly,
    Weekly,
    Daily,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn ScheduledTaskRegisterScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
    tasks: RwSignal<Option<Vec<Task>>>,
) -> impl IntoView {
    let character = RwSignal::new("rena/hearing.png".to_string());
    let message = RwSignal::new(Message::new(
        "レーナ".to_string(),
        format!(
            "{}さん、定期業務の登録ですね。繰り返しタイプと時刻を指定してください。",
            hardworker.get().unwrap().name
        ),
    ));

    let title = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let repeat_type = RwSignal::new(RepeatType::Monthly);
    let datetime = RwSignal::new(String::new());
    let day_of_month = RwSignal::new(1u32);
    let weekday = RwSignal::new(1u32); // 月曜
    let time = RwSignal::new("09:00".to_string());

    let submit_scheduled_task = move |_| {
        // パターンごとのDTO生成
        let pattern = match repeat_type.get() {
            RepeatType::Monthly => PatternDTO::Monthly {
                day: day_of_month.get(),
                time: time.get(),
            },
            RepeatType::Weekly => PatternDTO::Weekly {
                weekday: weekday.get(),
                time: time.get(),
            },
            RepeatType::Daily => PatternDTO::Daily { time: time.get() },
        };

        let dto = ScheduledTaskDTO {
            title: title.get(),
            description: description.get(),
            pattern,
        };

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({ "dto": dto })).unwrap();
            let result = invoke("save_scheduled_task", args).await;
            if let Ok(task) = serde_wasm_bindgen::from_value::<Task>(result) {
                // タスクの新規登録
                tasks.update(|opt| {
                    if let Some(list) = opt {
                        list.push(task.clone());
                    }
                });
            }
        });

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
        description.set("".to_string());
        datetime.set("".to_string());
        time.set("09:00".to_string());
    };

    view! {
        <div class="zentas-main">
            <MenuBarComponent scene=scene />
            <img src="public/assets/backgrounds/guild_inside.png" class="zentas-bg" />
            <img src={move || format!("public/assets/characters/{}", character.get())} class="zentas-person" />
            <WindowMessage message={message} />

            <div class="scheduled-task-form">

                <div class="task-list-title">"定期依頼の登録"</div>
                <div class="scheduled-task-form-scroll">
                <div class="scheduled-task-form-input-list">
                    <div class="scheduled-task-form-input">
                        <label>"依頼タイトル（必須）"</label>
                        <input
                        prop:value=move || title.get()
                        placeholder="例: OO王の謁見" on:input=move |e| title.set(event_target_value(&e)) />
                    </div>
                    <div class="scheduled-task-form-input">
                        <label>"依頼詳細（任意）"</label>
                        <textarea
                            prop:value=move || description.get()
                            placeholder="例: 西の森で発生中" on:input=move |e| description.set(event_target_value(&e))>
                        </textarea>
                    </div>

                    <div class="scheduled-task-form-input">
                        <label>"繰り返しタイプ"</label>
                        <select
                        on:change=move |e| {
                            match event_target_value(&e).as_str() {
                                "Monthly" => repeat_type.set(RepeatType::Monthly),
                                "Weekly" => repeat_type.set(RepeatType::Weekly),
                                "Daily" => repeat_type.set(RepeatType::Daily),
                                _ => {},
                            }
                        }>
                            <option value="Monthly">"毎月"</option>
                            <option value="Weekly">"毎週"</option>
                            <option value="Daily">"毎日"</option>
                        </select>
                    </div>

                    // === パターン別の入力 ===

                    <Show when=move || repeat_type.get() == RepeatType::Monthly>
                        <div class="scheduled-task-form-input">
                            <label>"毎月何日？（1〜31）"</label>
                            <input type="number" min="1" max="31"
                            prop:value=move || day_of_month.get()
                            on:input=move |e| {
                                if let Ok(val) = event_target_value(&e).parse::<u32>() {
                                    day_of_month.set(val);
                                }
                            } />
                            <label>"何時？（例: 10:00）"</label>
                            <input type="time"
                            prop:value=move || time.get()
                            on:input=move |e| time.set(event_target_value(&e)) />
                        </div>
                    </Show>

                    <Show when=move || repeat_type.get() == RepeatType::Weekly>
                        <div class="scheduled-task-form-input">
                            <label>"曜日（0=日〜6=土）"</label>
                            <input type="number" min="0" max="6"
                            prop:value=move || weekday.get()
                            on:input=move |e| {
                                if let Ok(val) = event_target_value(&e).parse::<u32>() {
                                    weekday.set(val);
                                }
                            } />
                            <label>"何時？"</label>
                            <input type="time"
                            prop:value=move || time.get()
                            on:input=move |e| time.set(event_target_value(&e)) />
                        </div>
                    </Show>

                    <Show when=move || repeat_type.get() == RepeatType::Daily>
                        <div class="scheduled-task-form-input">
                            <label>"何時？"</label>
                            <input type="time"
                            prop:value=move || time.get()
                            on:input=move |e| time.set(event_target_value(&e)) />
                        </div>
                    </Show>
                </div>


                <div style="margin-top: 20px;">
                    <button style="background:rgba(220, 90, 90, 0.7);margin-right:10px;" on:click=move |_| scene.set(Scene::TaskRegister)>"依頼選択に戻る"</button>
                    <button style="background:rgba(90, 180, 120, 0.7);" on:click=submit_scheduled_task>
                        "定期依頼を登録"
                    </button>
                </div>
                </div>
            </div>
        </div>
    }
}
