use crate::models::hard_worker::HardWorker;
use crate::models::task::Task;
use crate::scenes::finish::FinishScene;
use crate::scenes::guild::GuildScene;
use crate::scenes::register::RegisterScene;
use crate::scenes::start::StartScene;
use crate::scenes::status::StatusScene;
use crate::scenes::task_list::TaskListScene;
use crate::scenes::task_register::TaskRegisterScene;

use leptos::task::{self, spawn_local};
use leptos::{logging, prelude::*};

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
    Status,
}

#[component]
pub fn App() -> impl IntoView {
    // scene切り替え用
    let scene = RwSignal::new(Scene::Start);
    // JSONの情報を取得する
    // let hardworker = RwSignal::new(HardWorker::new("".to_string()));

    let hardworker = RwSignal::new(None::<HardWorker>);
    let tasks = RwSignal::new(None::<Vec<Task>>);
    spawn_local({
        let hardworker = hardworker.clone();
        
        
        async move {
            let result = invoke("load_hardworker", JsValue::NULL).await;
            logging::log!("{:?}", result);
            if let Ok(hw) = serde_wasm_bindgen::from_value::<HardWorker>(result) {
                logging::log!("ハードワーカーをLOADしました");
                hardworker.set(Some(hw));
            }
        }
    });

    spawn_local({
        let tasks = tasks.clone();
        async move {
            let result = invoke("load_tasks", JsValue::NULL).await;
            if let Ok(loaded) = serde_wasm_bindgen::from_value::<Vec<Task>>(result) {
                logging::log!("TASKをLOADしました");
                tasks.set(Some(loaded));
            } else {
                logging::log!("TASKを空でLOADしました");
                tasks.set(Some(vec![]));
            }
        }
    });

    view! {
        <Show
            when=move || hardworker.get().is_some() && tasks.get().is_some()
            fallback=|| view! { <div>"読み込み中..."</div> }
        >

                <Show
                when=move || scene.get() == Scene::Start
                fallback=|| ()>
                    <StartScene scene=scene hardworker=hardworker/>
                </Show>

                <Show
                when=move || scene.get() == Scene::Guild && !hardworker.get().unwrap().name.is_empty()
                fallback=|| ()>
                    <GuildScene scene=scene hardworker=hardworker tasks=tasks/>
                </Show>

                <Show
                when=move || scene.get() == Scene::Register && hardworker.get().unwrap().name.is_empty()
                fallback=|| ()>
                    <RegisterScene scene=scene hardworker=hardworker/>
                </Show>

                <Show
                when=move || scene.get() == Scene::Finish
                fallback=|| ()>
                    <FinishScene scene=scene hardworker=hardworker/>
                </Show>

                <Show
                when=move || scene.get() == Scene::TaskRegister
                fallback=|| ()>
                    <TaskRegisterScene hardworker=hardworker scene=scene tasks=tasks/>
                </Show>

                <Show
                when=move || scene.get() == Scene::TaskList
                fallback=|| ()>
                    <TaskListScene scene=scene hardworker=hardworker tasks=tasks/>
                </Show>


                <Show
                when=move || scene.get() == Scene::Status
                fallback=|| ()>
                    <StatusScene scene=scene hardworker=hardworker tasks=tasks/>
                </Show>
        </Show>

    }
}
