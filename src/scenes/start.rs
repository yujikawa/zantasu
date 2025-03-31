use leptos::prelude::*;

use crate::app::Scene;

#[component]
pub fn StartScene(scene: RwSignal<Scene>, hardworker_name: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="zentas-main">
            // === 背景 ===
            <img src="public/assets/backgrounds/guild_outside.png"
                class="zentas-bg"/>

            // === タイトルロゴ ===
            <img src="public/assets/logo/title.png"
                style="position: absolute; left: 50%; top: -80px; transform: translateX(-50%); width: 400px;" />

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
            on:click=move |_| if hardworker_name.get()== "" { scene.set(Scene::Register) } else {scene.set(Scene::Guild)}
            >
            "業務スタート"
            </button>
        </div>
    }
}
