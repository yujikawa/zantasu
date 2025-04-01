use leptos::prelude::*;

use crate::app::Scene;
use crate::models::hard_worker::HardWorker;

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

            <img src="public/assets/characters/byebye.png"
            style="position: absolute;right: 50px;bottom: 0;height: 500px;" />

            <img src="public/assets/characters/guild_master.png"
                style="position: absolute;left: 50px;bottom: 0;height: 500px;" />

            // === スタートボタン ===
            <button
            style="
                position: absolute; 
                left: 50%;
                bottom: 100px;
                transform: translateX(-50%);
                padding: 16px 32px;
                font-size: 24px;
                border-radius: 8px;
            "
            on:click=move |_| if hardworker.get().unwrap().name.is_empty() { scene.set(Scene::Register) } else {scene.set(Scene::Guild)}
            >
            "業務スタート"
            </button>
        </div>
    }
}
