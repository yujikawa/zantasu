use leptos::prelude::*;
use leptos::task::{self, spawn_local};
use wasm_bindgen::prelude::*;

use crate::components::window_message::WindowMessage;
use crate::models::message::Message;
use crate::{app::Scene, models::hard_worker::HardWorker};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn register(
    name: RwSignal<String>,
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
) {
    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
            "name": if name.get() == "" { "ななし".to_string() } else { name.get() }
        }))
        .unwrap();
        // let hw = HardWorker::new(name.get());
        let result = invoke("save_hardworker", args).await;
        if let Ok(hw) = serde_wasm_bindgen::from_value::<HardWorker>(result) {
            hardworker.set(Some(hw));
            leptos::logging::log!("登録しました");
            scene.set(Scene::Guild);
        }
    });
}

#[component]
pub fn RegisterScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
) -> impl IntoView {
    let message = RwSignal::new(
        Message::new("ギルド受付嬢".to_string(), "初めての方ですね？まずはハードワーカー登録が必要なのでこちらに名前を記入してください。お手元の魔道具、打鍵石で入力できますよ。登録後はFランクからのスタートとなります。タスクをこなせばランクがあがりますので頑張ってくださいね！"
            .to_string())
    );
    let name = RwSignal::new("".to_string());

    view! {
        <div class="zantas-main">
            // === 背景 ===
            <img src="public/assets/backgrounds/guild_day.png"
                class="zantas-bg" />

            <img src="public/assets/characters/receptionist/register.png"
                class="zantas-person" />

            <div style="position: absolute; top:200px; left:50px; padding:10px; width:400px; height:200px; background:rgba(31, 29, 29, 0.7);  border: 2px solid #ffffff;
    border-radius: 12px;">
                <h2>"ハードワーカー登録書"</h2>

                <input type="text"
                placeholder="ここに名前を入力してね"
                value="test"
                on:input:target=move |ev| {
                    name.set(ev.target().value());

                }
                prop:value=name.get()
            />

                <button style="background:rgba(90, 116, 220, 0.7);margin-left:10px;" on:click=move |_| register(name, scene, hardworker)
            >"登録"</button>

            </div>

        <WindowMessage message=message/>

        </div>
    }
}
