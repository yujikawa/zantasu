use leptos::prelude::*;

use crate::app::Scene;
use crate::components::window_message::WindowMessage;

#[component]
pub fn RegisterScene(scene: RwSignal<Scene>) -> impl IntoView {
    let message = RwSignal::new(
        "初めての方ですね？まずはハードワーカー登録が必要なのでこちらに名前を記入してください。お手元の魔道具、打鍵石で入力できますよ"
            .to_string(),
    );
    let name = RwSignal::new("".to_string());

    view! {
        <div style="position: fixed; top:0; left:0; right:0; bottom:0; overflow: hidden;">
            // === 背景 ===
            <img src="public/assets/backgrounds/guild_day.png"
                style="position: absolute; width: 100%; height: 100%; object-fit: cover;" />

            <img src="public/assets/characters/register.png"
                style="position: absolute; right: 50px; bottom: 0; height: 600px;" />

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

                <button style="background:rgba(90, 116, 220, 0.7);margin-left:10px;" on:click=move |_| scene.set(Scene::Guild)
            >"登録"</button>

            </div>

        <WindowMessage message=message/>

        </div>
    }
}
