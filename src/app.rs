use crate::scenes::finish::FinishScene;
use crate::scenes::guild::GuildScene;
use crate::scenes::register::RegisterScene;
use crate::scenes::start::StartScene;
use leptos::prelude::*;
use leptos::task::{self, spawn_local};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Copy, PartialEq)]
pub enum Scene {
    Start,
    Register,
    Guild,
    Finish,
}

#[component]
pub fn App() -> impl IntoView {
    // scene切り替え用
    let scene = RwSignal::new(Scene::Start);

    view! {
        <div>
        <Show
        when=move || scene.get() == Scene::Start
        fallback=|| ()>
        <StartScene scene=scene/>

        </Show>

        <Show
        when=move || scene.get() == Scene::Guild
        fallback=|| ()>
            <GuildScene scene=scene/>

        </Show>

        <Show
        when=move || scene.get() == Scene::Register
        fallback=|| ()>
            <RegisterScene scene=scene/>

        </Show>

        <Show
        when=move || scene.get() == Scene::Finish
        fallback=|| ()>
            <FinishScene scene=scene/>

        </Show>


        </div>
    }
}
