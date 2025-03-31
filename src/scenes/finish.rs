use leptos::prelude::*;

use crate::app::Scene;

#[component]
pub fn FinishScene(scene: RwSignal<Scene>) -> impl IntoView {
    view! {
        <div style="position: fixed; top:0; left:0; right:0; bottom:0; overflow: hidden;">
            // === 背景 ===
            <img src="public/assets/backgrounds/guild_outside2.png"
                style="position: absolute; width: 100%; height: 100%; object-fit: cover;" />

                <img src="public/assets/characters/byebye.png"
                    style="position: absolute; right: 50px; bottom: 0; height: 600px;" />


                    <div style="
                    position: absolute;
                    left: 50%;
                    bottom: 20px;
                    transform: translateX(-50%);
                    width: 80%;
                    background: rgba(31, 29, 29, 0.7);
                    border: 2px solid #000;
                    border-radius: 10px;
                    padding: 16px;
                    font-size: 20px;
                    ">
        "今日もお疲れさまでした！"
            </div>

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
