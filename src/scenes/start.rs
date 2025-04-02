use crate::app::Scene;
use crate::models::hard_worker::HardWorker;
use leptos::prelude::*;
use leptos::task::{self, spawn_local};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn StartScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
) -> impl IntoView {
    view! {
        <div class="zentas-main">
            // === 背景 ===
            <img src="public/assets/backgrounds/guild_outside.png"
                class="zentas-bg"/>

            // === タイトルロゴ ===
            <img src="public/assets/logo/title.png"
                style="position: absolute; left: 50%; top: -80px; transform: translateX(-50%); width: 400px;" />

            <img src="public/assets/characters/rena/start.png"
            style="position: absolute;right: 50px;bottom: 0;height: 500px;" />

            <img src="public/assets/characters/guild_master.png"
                style="position: absolute;left: 50px;bottom: 0;height: 500px;" />

            // === スタートボタン ===
            <div class="start-menu">
            <button class="start-button"
            on:click=move |_| if hardworker.get().unwrap().name.is_empty() { scene.set(Scene::Register) } else {scene.set(Scene::Guild)}
            >
            "業務スタート"
            </button>

            // === 終了ボタン ===
            <button
                class="start-button"
                on:click=move |_| {
                    spawn_local(async move {
                        let _ = invoke("close_app", JsValue::NULL ).await;
                    });
                }
            >
            "終了する"
            </button>
            </div>

        </div>
    }
}
