use crate::app::Scene;
use crate::components::board::BoardComponent;
use crate::components::menu_bar::MenuBarComponent;

use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::message::Message;
use crate::models::task::Task;

use leptos::prelude::*;

#[component]
pub fn StatusScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
    tasks: RwSignal<Option<Vec<Task>>>,
) -> impl IntoView {
    let task_count = RwSignal::new(tasks.get().unwrap().len());
    let message = RwSignal::new(Message::new(
        hardworker.get().unwrap().name,
        "ステータスオープン！".to_string(),
    ));
    view! {
        <main>

        <div class="zentas-main">
            <MenuBarComponent scene=scene/>

            // === 背景 ===
            <img src="public/assets/backgrounds/guild_inside.png" class="zentas-bg" />

            // === 掲示板 ===
            <BoardComponent tasks=tasks/>

            <div class="status-window">
            <div class="status-list">"【 ステータス一覧 】"</div>

            <div class="status-item">
                <label>名前</label>
                <p>{hardworker.get().unwrap().name}</p>
            </div>

            <div class="status-item">
                <label>労働等級</label>
                <p>{hardworker.get().unwrap().rank} ランク</p>
            </div>

            <div class="status-item">
                <label>職業</label>
                <p>"社畜"</p>
            </div>

            <div class="status-item">
                <label>現在の依頼受注数</label>
                <p>{task_count.get()}</p>
            </div>

            // <div class="status-item">
            //     <label>依頼達成数</label>
            //     <p>{hardworker.get().unwrap().quest_completed} 件</p>
            // </div>
        </div>



        <img src="public/assets/characters/status_open.png"
        class="status-open" />

        <div class="thought-bubble">
            "何をみているんだろう..?"
        </div>

        <img src="public/assets/characters/rena/curious.png"
            class="zentas-left-person"/>
        // === セリフウィンドウ ===
        <WindowMessage message={message}/>

    </div>




    </main>
    }
}
