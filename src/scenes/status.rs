use crate::app::Scene;
use crate::components::board::BoardComponent;
use crate::components::menu_bar::MenuBarComponent;

use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::task::Task;

use leptos::prelude::*;

#[component]
pub fn StatusScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
    tasks: RwSignal<Option<Vec<Task>>>,
) -> impl IntoView {
    let task_count = RwSignal::new(tasks.get().unwrap().len());

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
                <label>ランク</label>
                <p>{hardworker.get().unwrap().rank} ランク</p>
            </div>
            
            // <div class="status-item">
            //     <label>職業</label>
            //     <p>{hardworker.get().unwrap().job}</p>
            // </div>
            
            // <div class="status-item">
            //     <label>冒険者ランク</label>
            //     <p>{hardworker.get().unwrap().adventurer_rank}</p>
            // </div>
            
            // <div class="status-item">
            //     <label>依頼達成数</label>
            //     <p>{hardworker.get().unwrap().quest_completed} 件</p>
            // </div>
        </div>
        

            // === ステータス ===
            <img src="public/assets/characters/status_open.png"
            class="status-open" />

            <img src="public/assets/characters/rena/curious.png"
             class="zentas-left-person"/>

            // === セリフウィンドウ ===
            // <WindowMessage message={ message }/>

    </div>

    </main>
    }
}
