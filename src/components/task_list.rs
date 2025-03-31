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
pub fn TaskList(
    is_show_task_list: RwSignal<bool>,
    receptionist: RwSignal<String>,
) -> impl IntoView {
    let tasks: RwSignal<Vec<String>> =
        RwSignal::new(vec!["薬草採取".to_string(), "ゴブリン退治".to_string()]);

    let close_task_list = move |_ev: leptos::ev::MouseEvent| {
        is_show_task_list.set(false);
        receptionist.set("normal_stand.png".to_string());

    };

    view! {
        <div style="position: absolute; top:100px; left:10px; padding:10px; width:500px; height:500px; background:rgba(31, 29, 29, 0.7);">
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
        <button style="background:rgba(220, 90, 90, 0.7);" on:click=close_task_list>"閉じる"</button>
        <button style="background:rgba(90, 116, 220, 0.7);margin-left:10px;" on:click=close_task_list>"クエスト登録"</button>

        </div>
    }
}
