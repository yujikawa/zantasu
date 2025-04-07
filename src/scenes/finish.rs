use leptos::prelude::*;

use crate::app::Scene;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::message::Message;

#[component]
pub fn FinishScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
) -> impl IntoView {
    let message = RwSignal::new(Message::new(
        "レーナ".to_string(),
        format!(
            "{}さん、今日もお疲れさまでした！またお仕事がんばってください！",
            hardworker.get().unwrap().name
        ),
    ));

    view! {

        <div class="zantas-main">
            // === 背景 ===
            <img src="public/assets/backgrounds/guild_outside2.png"
                class="zantas-bg" />

                <img src="public/assets/characters/rena/bye.png"
                    class="zantas-person" />

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
