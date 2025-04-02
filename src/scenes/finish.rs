use leptos::prelude::*;

use crate::app::Scene;
use crate::components::window_message::WindowMessage;
use crate::models::message::Message;

#[component]
pub fn FinishScene(scene: RwSignal<Scene>) -> impl IntoView {
    let message = RwSignal::new(Message::new(
        "レーナ".to_string(),
        "今日もお疲れさまでした！".to_string(),
    ));

    view! {

        <div class="zentas-main">
            // === 背景 ===
            <img src="public/assets/backgrounds/guild_outside2.png"
                class="zentas-bg" />

                <img src="public/assets/characters/rena/bye.png"
                    class="zentas-person" />

            <WindowMessage message=message />


            <button
            style="
                position: absolute; 
                right: -80px;
                top: 10px;
                transform: translateX(-50%);
                padding: 16px 32px;
                font-size: 24px;
                border-radius: 8px;
                background:rgba(220, 90, 90, 0.7);
            "
            on:click=move |_| scene.set(Scene::Start)
            >
            "最初に戻る"
            </button>
        </div>
    }
}
