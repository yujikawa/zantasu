use crate::models::task::Task;
use crate::scenes::finish::FinishScene;
use crate::scenes::guild::GuildScene;
use crate::scenes::register::RegisterScene;
use crate::scenes::start::StartScene;
use crate::scenes::task_list::TaskListScene;
use crate::scenes::task_register::TaskRegisterScene;

use leptos::prelude::*;
// use leptos::task::{self, spawn_local};

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
    TaskRegister,
    TaskList,
}

#[component]
pub fn App() -> impl IntoView {
    // scene切り替え用
    let scene = RwSignal::new(Scene::Start);
    // JSONの情報を取得する
    let hardworker_name = RwSignal::new("ゆじ".to_string());
    let tasks: RwSignal<Vec<Task>> = RwSignal::new(vec![
        Task::new(
            "薬草採取".into(),
            Some("西の森で薬草を5個採取".into()),
            "C".into(),
            Some("2025-05-01".into()),
        ),
        Task::new(
            "ゴブリン退治".into(),
            None, // 詳細なし
            "B".into(),
            None, // 締切なし
        ),
    ]);

    view! {

        <div>
            <Show
            when=move || scene.get() == Scene::Start
            fallback=|| ()>
                <StartScene scene=scene hardworker_name=hardworker_name/>
            </Show>

            <Show
            when=move || scene.get() == Scene::Guild && hardworker_name.get() != ""
            fallback=|| ()>
                <GuildScene scene=scene hardworker_name=hardworker_name tasks=tasks/>
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

            <Show
            when=move || scene.get() == Scene::TaskRegister
            fallback=|| ()>
                <TaskRegisterScene hardworker_name=hardworker_name scene=scene tasks=tasks/>
            </Show>

            <Show
            when=move || scene.get() == Scene::TaskList
            fallback=|| ()>
                <TaskListScene scene=scene hardworker_name=hardworker_name tasks=tasks/>
            </Show>

        </div>
    }
}
