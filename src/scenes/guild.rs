use crate::app::Scene;
use crate::components::menu_bar::MenuBarComponent;
use crate::components::window_message::WindowMessage;

use leptos::prelude::*;

#[component]
pub fn GuildScene(scene: RwSignal<Scene>) -> impl IntoView {
    let hard_worker_name = "hogehoge".to_string();
    let welcome_message = format!(
        "{}さん、ようこそ残業ギルドへ！上部のメニューからやりたいことを選択してね。",
        hard_worker_name
    );
    let receptionist = RwSignal::new("normal_stand.png".to_string());

    let task = vec!["hoge", "hoge"];
    let task_count = RwSignal::new(2);

    let welcome_message = if task_count.get() == 0 {
        format!(
            "{}さん、ようこそ残業ギルドへ！上部のメニューからやりたいことを選択してね。",
            hard_worker_name
        )
    } else {
        format!(
            "{}さん、受注している依頼が{}件あるみたいね...しっかり終わらせて報告してくださいね",
            hard_worker_name,
            task_count.get(),
        )
    };
    let message = RwSignal::new(welcome_message);

    view! {
        <main>

        <div style="position: fixed; top:0; left:0; right:0; bottom:0; overflow: hidden;">
        <MenuBarComponent scene=scene/>

                // === 背景 ===
                <img src="public/assets/backgrounds/guild_inside.png"
                    style="position: absolute; width: 100%; height: 100%; object-fit: cover;" />

                // === 掲示板 ===

                <Show
                when=move || task_count.get() !=0
                fallback=|| ()>

                <img src="public/assets/objects/board_with_paper.png"
                    style="position: absolute; left: 50px; top: 140px; width: 500px;"
                    />

                </Show>

                <Show
                when=move || task_count.get() == 0
                fallback=|| ()>

                <img src="public/assets/objects/board.png"
                    style="position: absolute; left: 50px; top: 140px; width: 500px;"
                    />

                </Show>


                // === アリナ（立ち絵） ===
                <img src={move || format!("public/assets/characters/{}", receptionist.get())}
                    style="position: absolute; right: 50px; bottom: 0; height: 600px;" />
        // === セリフウィンドウ ===
        <WindowMessage message={ message }/>


    </div>

    </main>
    }
}
