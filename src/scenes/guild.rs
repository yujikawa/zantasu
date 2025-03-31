use crate::app::Scene;
use crate::components::task_list::TaskList;
use crate::components::window_message::WindowMessage;
use leptos::prelude::*;

#[component]
pub fn GuildScene(scene: RwSignal<Scene>) -> impl IntoView {
    let welcome_message =
        "ようこそ、残業ギルドへ！掲示板をクリックするとクエストが見れます！".to_string();
    let is_show_task_list = RwSignal::new(false);
    let message = RwSignal::new(welcome_message);
    let receptionist = RwSignal::new("normal_stand.png".to_string());

    let show_task_list = move |_ev: leptos::ev::MouseEvent| {
        // let v = event_target_value(&ev);
        is_show_task_list.set(true);
        message
            .set("あなたの受注しているクエストの確認や新たにクエストを登録できますけど..まだ仕事するの..?".to_string());
        receptionist.set("normal.png".to_string());
    };

    view! {
        <main>

        <div style="position: fixed; top:0; left:0; right:0; bottom:0; overflow: hidden;">

                // === 背景 ===
                <img src="public/assets/backgrounds/guild_inside.png"
                    style="position: absolute; width: 100%; height: 100%; object-fit: cover;" />

                // === 掲示板 ===
                <img src="public/assets/objects/board.png"
                    style="position: absolute; left: 50px; top: 140px; width: 500px;"
                    on:click=show_task_list
                    />

                // === アリナ（立ち絵） ===
                <img src={move || format!("public/assets/characters/{}", receptionist.get())}
                    style="position: absolute; right: 50px; bottom: 0; height: 600px;" />
        // === セリフウィンドウ ===
        <WindowMessage message={ message }/>

        <Show
            when=move || is_show_task_list.get()
            fallback=|| ()>
            <TaskList is_show_task_list=is_show_task_list receptionist=receptionist/>
        </Show>


        <button
        style="
            position: absolute; 
            right: -100px;
            top: 10px;
            transform: translateX(-50%);
            padding: 16px 32px;
            font-size: 24px;
            border-radius: 8px;
            background:rgba(220, 90, 90, 0.7);
        "
        on:click=move |_| scene.set(Scene::Finish)
        >
        "業務を終了する"
        </button>
    </div>


    </main>
    }
}
