use std::time::Duration;

use crate::app::Scene;
use crate::components::menu_bar::MenuBarComponent;
use crate::components::scheduled_task_form::{self, ScheduledTaskModalComponent};
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::message::Message;
use crate::models::task::{ScheduledTask, ScheduledTaskFormState, TaskCreateDTO};
use leptos::task::spawn_local;
use leptos::{logging, prelude::*};
use wasm_bindgen::prelude::*;

// use crate::models::scheduled_task::{PatternDTO, ScheduledTaskDTO};

use crate::models::task::SchedulePattern;
use crate::models::task::ScheduledTaskCreateDTO;

#[derive(Clone, Debug, PartialEq)]
enum RepeatType {
    Monthly,
    Weekly,
    Daily,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn ScheduledTaskRegisterScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
    tasks: RwSignal<Option<Vec<ScheduledTask>>>,
) -> impl IntoView {
    let scheduled_task_form_state = RwSignal::new(ScheduledTaskFormState::new());
    let on_cancel = move || scene.set(Scene::TaskRegister);

    let on_submit = move || {
        let new_task = TaskCreateDTO::new(
            scheduled_task_form_state.get().task.title,
            if scheduled_task_form_state.get().task.description.is_empty() {
                None
            } else {
                Some(scheduled_task_form_state.get().task.description)
            },
            None,
        );

        let pattern = scheduled_task_form_state.get().pattern;

        let dto = ScheduledTaskCreateDTO::new(new_task, pattern);

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({ "dto": dto })).unwrap();
            let result = invoke("save_scheduled_task", args).await;
            if let Ok(task) = serde_wasm_bindgen::from_value::<ScheduledTask>(result) {
                // タスクの新規登録
                tasks.update(|opt| {
                    if let Some(list) = opt {
                        list.push(task.clone());
                    }
                });
            }
        });
        scheduled_task_form_state.set(ScheduledTaskFormState::new());
    };

    view! {
    <ScheduledTaskModalComponent form=scheduled_task_form_state on_submit=on_submit submit_label="定期依頼の登録" on_cancel=on_cancel/>
    }
}
