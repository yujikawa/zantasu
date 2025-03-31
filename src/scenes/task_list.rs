use crate::app::Scene;
use crate::components::window_message::WindowMessage;
use crate::models::task::Task;
use leptos::{logging, prelude::*};

#[component]
pub fn TaskListScene(
    scene: RwSignal<Scene>,
    hardworker_name: RwSignal<String>,
    tasks: RwSignal<Vec<Task>>,
) -> impl IntoView {
    let character = RwSignal::new("smile.png".to_string());

    let message = RwSignal::new(format!(
        "{}さんへの依頼が確認できます。依頼が完了したら忘れずに報告してくださいね！",
        hardworker_name.get()
    ));

    fn select_task(character: RwSignal<String>, message: RwSignal<String>, selected_task: Task) {
        let new_message = match selected_task.description {
            Some(description) => {
                character.set("normal.png".to_string());
                format!("依頼の詳細は..{}ということみたいです！", description)
            }
            None => {
                character.set("worry2.png".to_string());
                format!("依頼の詳細は..無いみたいですね..")
            }
        };
        message.set(new_message);
    }

    view! {


        <div class="zentas-main">

        // === 背景 ===
        <img src="public/assets/backgrounds/guild_inside.png"
            class="zentas-bg" />

        // === 受付嬢（立ち絵） ===
        <img src={move || format!("public/assets/characters/{}", character.get())}
            class="zentas-person" />
        // === セリフウィンドウ ===
        <WindowMessage message={ message }/>

    </div>
        <div class="task-list-window">
            <div class="task-list-title">"依頼一覧"</div>
            <div class="task-list-scroll">
                <For
                    each=move || tasks.get()
                    key=|task| task.clone() // task自体がキー
                    children=move |task| view! {
                    <div class="task-item"
                    on:click=move |_| select_task(character, message, task.clone())
                    >
                        <div>{task.title.clone()}</div>
                        <div>{task.due_date.clone().unwrap_or("締切未定".into())}</div>

                    </div>
                    }
                />
            </div>
        </div>
        <button
        style="
            position: absolute; 
            right: 10px;
            top: 10px;
            transform: translateX(-50%);
            padding: 16px 32px;
            font-size: 24px;
            border-radius: 8px;
            background:rgba(220, 90, 90, 0.7);
        "
        on:click=move |_| scene.set(Scene::Guild)
        >
        "戻る"
        </button>
    }
}
