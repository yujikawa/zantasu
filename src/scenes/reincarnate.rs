use crate::app::Scene;
use crate::components::board::BoardComponent;
use crate::components::menu_bar::MenuBarComponent;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::message::Message;
use crate::models::task::Task;
use leptos::prelude::*;
use leptos::task::{self, spawn_local};
use std::time::Duration;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn ReincarnateScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
    tasks: RwSignal<Option<Vec<Task>>>,
) -> impl IntoView {
    let show_overlay = RwSignal::new(false);
    let is_shot = RwSignal::new(false);

    let receptionist = RwSignal::new("receptionist/reincarnate0.png".to_string());
    let message = RwSignal::new(Message::new(
        "ギルド受付嬢".to_string(),
        format!(
            "{}さん、実は私は禁忌魔術である転生という魔術を使えます。それを使えば転生できます。ただしすべてがリセットになりますが..それでも転生しますか？もし...システム的にバグかな？って思ったら転生しちゃってください",
            hardworker.get().unwrap().name
        ),
    ));

    let reset = move |_| {
        spawn_local(async move {
            let result = invoke("reset", JsValue::NULL).await;
            if let Ok(_) = serde_wasm_bindgen::from_value::<()>(result) {
                is_shot.set(true);
                receptionist.set("receptionist/reincarnate1.png".to_string());
                message.set(Message::new(
                    "ギルド受付嬢".to_string(),
                    format!("リンカーネイション！！はあぁぁぁぁあああああああ！！"),
                ));
                show_overlay.set(true);

                set_timeout(
                    move || {
                        leptos::web_sys::window()
                            .unwrap()
                            .location()
                            .reload()
                            .unwrap();
                    },
                    Duration::from_secs(3),
                );
            }
        });
    };

    view! {
        <main>

        <div class="zantas-main">
        <MenuBarComponent scene=scene/>

                // === 背景 ===
                <img src="public/assets/backgrounds/guild_inside.png" class="zantas-bg" />

                // === 掲示板 ===
                <BoardComponent tasks=tasks/>

                // === 受付嬢（立ち絵） ===
                <img
                src={move || format!("public/assets/characters/{}", receptionist.get())}
                class={move || if is_shot.get() { "zantas-person magic-blast" } else { "zantas-person"} } />
        // === セリフウィンドウ ===
        <WindowMessage message={ message }/>


        <div class="select-reincarnate-type">
            <button class="select-reincarnate-no-button"
                on:click=move |_|  { scene.set(Scene::Guild) }

                >"やめる"</button>
            <button class="select-reincarnate-yes-button"
                on:click=reset

                >"転生する!!"</button>
        </div>

        <div class=move || {
            if show_overlay.get() {
                "fade-overlay show"
            } else {
                "fade-overlay"
            }
        } />
    </div>

    </main>
    }
}
