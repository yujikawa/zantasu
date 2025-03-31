use crate::app::Scene;
use crate::components::window_message::WindowMessage;
use crate::models::task::Task;
use leptos::prelude::*;

#[component]
pub fn TaskRegisterScene(scene: RwSignal<Scene>) -> impl IntoView {
    let message = RwSignal::new("依頼内容を記載してくださいね！".to_string());

    let title = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let due_date = RwSignal::new(String::new());

    let submit_task = move |_| {
        let task = Task {
            title: title.get(),
            description: if description.get().is_empty() {
                None
            } else {
                Some(description.get())
            },
            rank: "C".to_string(), // とりあえずランクは仮
            due_date: if due_date.get().is_empty() {
                None
            } else {
                Some(due_date.get())
            },
        };

        // on_submit.run(task);
        // 入力リセット
        title.set("".to_string());
        description.set("".to_string());
        due_date.set("".to_string());
    };

    view! {

        <div style="position: fixed; top:0; left:0; right:0; bottom:0; overflow: hidden;">

        // === 背景 ===
        <img src="public/assets/backgrounds/guild_inside.png"
            style="position: absolute; width: 100%; height: 100%; object-fit: cover;" />

        // === アリナ（立ち絵） ===
        <img src="public/assets/characters/happy.png"
            style="position: absolute; right: 50px; bottom: 0; height: 600px;" />
        // === セリフウィンドウ ===
        <WindowMessage message={ message }/>

        //  依頼書
        <div class="task-form">
            <h3>"新しい依頼の登録"</h3>
            <div>
                <label>"依頼タイトル（必須）"</label>
                <input placeholder="例: ゴブリン退治" on:input=move |e| title.set(event_target_value(&e)) />
            </div>
            <div>
                <label>"依頼詳細（任意）"</label>
                <textarea placeholder="例: 西の森で発生中" on:input=move |e| description.set(event_target_value(&e))></textarea>
            </div>
            <div>
                <label>"締切日（任意）"</label>
                <input placeholder="例: 2025-05-01" on:input=move |e| due_date.set(event_target_value(&e)) />
            </div>
            // <button on:click=submit_task>"認可"</button>
            <button style="background:rgba(220, 90, 90, 0.7);" on:click=move |_| scene.set(Scene::Guild)>"閉じる"</button>
            <button style="background:rgba(90, 116, 220, 0.7);margin-left:10px;" on:click=submit_task>"登録"</button>
    
        </div>

    </div>


    }
}
