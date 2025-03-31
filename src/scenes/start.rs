use leptos::prelude::*;

use crate::app::Scene;

#[component]
pub fn StartScene(scene: RwSignal<Scene>) -> impl IntoView {
    view! {
        <div style="position: fixed; top:0; left:0; right:0; bottom:0; overflow: hidden;">
            // === 背景 ===
            <img src="public/assets/backgrounds/guild_outside.png"
                style="position: absolute; width: 100%; height: 100%; object-fit: cover;" />

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
            on:click=move |_| scene.set(Scene::Register)
            >
            "業務スタート"
            </button>
        </div>
    }
}
