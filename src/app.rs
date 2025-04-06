use std::time::Duration;

use crate::models::hard_worker::HardWorker;
use crate::models::task::{ScheduledTask, Task};
use crate::scenes::finish::FinishScene;
use crate::scenes::guild::GuildScene;
use crate::scenes::oneshot_task_register::OneShotTaskRegisterScene;
use crate::scenes::register::RegisterScene;
use crate::scenes::scheduled_task_list::ScheduledTaskListScene;
use crate::scenes::scheduled_task_register::ScheduledTaskRegisterScene;
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
    ScheduleTaskRegister,
    OneShotTaskRegister,
    TaskList,
    ScheduledTaskList,
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
    let schduled_tasks = RwSignal::new(None::<Vec<ScheduledTask>>);
    set_interval(
        move || {
            spawn_local(async move {
                let result = invoke("check_scheduled_tasks", JsValue::NULL).await;
                if let Ok(add_tasks) = serde_wasm_bindgen::from_value::<Vec<Task>>(result) {
                    tasks.update(|opt| {
                        if let Some(list) = opt {
                            list.extend(add_tasks.clone());
                        }
                    });
                }
                logging::log!("定期依頼の確認を行いました！");
            });
        },
        Duration::from_secs(60),
    );

    spawn_local({
        // let hardworker = hardworker.clone();
        async move {
            let result = invoke("get_hardworker", JsValue::NULL).await;
            if let Ok(hw) = serde_wasm_bindgen::from_value::<HardWorker>(result) {
                hardworker.set(Some(hw));
            }
        }
    });

    spawn_local({
        // let tasks = tasks.clone();
        async move {
            let result = invoke("get_tasks", JsValue::NULL).await;

            if let Ok(loaded) = serde_wasm_bindgen::from_value::<Vec<Task>>(result) {
                logging::log!("get_tasksLOADしました");

                logging::log!("{:?}", loaded);

                tasks.set(Some(loaded));
            } else {
                logging::log!("get_tasksLOADしてない");

                tasks.set(Some(vec![]));
            }
        }
    });

    spawn_local({
        async move {
            let result = invoke("get_scheduled_tasks", JsValue::NULL).await;

            if let Ok(loaded) = serde_wasm_bindgen::from_value::<Vec<ScheduledTask>>(result) {
                logging::log!("get_scheduled_tasksLOADしました");

                logging::log!("{:?}", loaded);

                schduled_tasks.set(Some(loaded));
            } else {
                logging::log!("get_scheduled_tasksLOADしてない");

                schduled_tasks.set(Some(vec![]));
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
                when=move || scene.get() == Scene::OneShotTaskRegister
                fallback=|| ()>
                    <OneShotTaskRegisterScene hardworker=hardworker scene=scene tasks=tasks/>
                </Show>

                <Show
                when=move || scene.get() == Scene::ScheduleTaskRegister
                fallback=|| ()>
                    <ScheduledTaskRegisterScene hardworker=hardworker scene=scene tasks=schduled_tasks/>
                </Show>

                <Show
                when=move || scene.get() == Scene::TaskList
                fallback=|| ()>
                    <TaskListScene scene=scene hardworker=hardworker tasks=tasks/>
                </Show>

                <Show
                when=move || scene.get() == Scene::ScheduledTaskList
                fallback=|| ()>
                    <ScheduledTaskListScene scene=scene hardworker=hardworker tasks=schduled_tasks/>
                </Show>


                <Show
                when=move || scene.get() == Scene::Status
                fallback=|| ()>
                    <StatusScene scene=scene hardworker=hardworker tasks=tasks/>
                </Show>
        </Show>

    }
}
