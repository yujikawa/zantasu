use crate::app::Scene;
use crate::components::window_message::WindowMessage;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    title: String,
    description: String,
    rank: String,
    due_date: String,
}

impl Task {
    fn new(title: String, description: String, rank: String, due_date: String) -> Self {
        return Task {
            title,
            description,
            rank,
            due_date,
        };
    }
}

#[component]
pub fn TaskListScene(scene: RwSignal<Scene>) -> impl IntoView {
    let message = RwSignal::new("あなたへの依頼が確認できます。依頼が完了したら忘れずに報告してくださいね！".to_string());

    let tasks: RwSignal<Vec<String>> =
        RwSignal::new(vec!["薬草採取".to_string(), "ゴブリン退治".to_string()]);

    // let close_task_list = move |_ev: leptos::ev::MouseEvent| {
    //     scene.set(Scene::Guild);
    // };

    view! {


        <div style="position: fixed; top:0; left:0; right:0; bottom:0; overflow: hidden;">

        // === 背景 ===
        <img src="public/assets/backgrounds/guild_inside.png"
            style="position: absolute; width: 100%; height: 100%; object-fit: cover;" />

        // === アリナ（立ち絵） ===
        <img src="public/assets/characters/smile.png"
            style="position: absolute; right: 50px; bottom: 0; height: 600px;" />
        // === セリフウィンドウ ===
        <WindowMessage message={ message }/>

    </div>
        <div style="position: absolute; top:100px; left:10px; padding:10px; width:500px; height:500px; background:rgba(31, 29, 29, 0.7);  border: 2px solid #ffffff;
  border-radius: 12px;">
        <h2>"依頼一覧"</h2>
        <ul>
            <For
                each=move || tasks.get()
                key=|task| task.clone() // task自体がキー
                children=move |task| view! {
                    <li>{task}</li>
                }
            />
        </ul>
        <button style="background:rgba(220, 90, 90, 0.7);" on:click=move |_| scene.set(Scene::Guild)>"閉じる"</button>

        </div>
    }
}
