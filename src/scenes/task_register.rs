use std::time::Duration;

use crate::app::Scene;
use crate::components::board::BoardComponent;
use crate::components::menu_bar::MenuBarComponent;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::message::Message;
use crate::models::task::{Task, TaskCreateDTO};

use leptos::prelude::*;
use leptos::task::{self, spawn_local};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn TaskRegisterScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
    tasks: RwSignal<Option<Vec<Task>>>,
) -> impl IntoView {
    let character = RwSignal::new("receptionist/explain.png".to_string());
    let message = RwSignal::new(Message::new(
        "ギルド受付嬢".to_string(),
        format!(
            "{}さん、依頼は２種類あります。単発の依頼と定期的に発生する依頼とありますのでまずは選択してください。",
            hardworker.get().unwrap().name
        ),
    ));

    view! {

        <div class="zantas-main">
        <MenuBarComponent scene=scene/>

        // === 背景 ===
        <img src="public/assets/backgrounds/guild_inside.png"
            class="zantas-bg" />

            // === 掲示板 ===
            <BoardComponent tasks=tasks/>

            <div class="select-register-type">
                <button class="select-register-type-button"
                on:click=move |_|  { scene.set(Scene::OneShotTaskRegister) }

                >"単発依頼"</button>
                <button class="select-register-type-button"
                on:click=move |_| { scene.set(Scene::ScheduleTaskRegister) }

                >"定期依頼"</button>
            </div>

        // === 受付嬢（立ち絵） ===
        <img src={move || format!("public/assets/characters/{}", character.get())}
            class="zantas-person" />
        // === セリフウィンドウ ===
        <WindowMessage message={ message }/>

    </div>


    }
}
