use crate::app::Scene;
use crate::components::board::BoardComponent;
use crate::components::menu_bar::MenuBarComponent;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::HardWorker;
use crate::models::message::Message;
use crate::models::task::Task;
use leptos::prelude::*;

#[component]
pub fn GuildScene(
    scene: RwSignal<Scene>,
    hardworker: RwSignal<Option<HardWorker>>,
    tasks: RwSignal<Option<Vec<Task>>>,
) -> impl IntoView {
    let receptionist = RwSignal::new("rena/normal.png".to_string());
    let task_count = RwSignal::new(tasks.get().unwrap().len());

    let welcome_message = if task_count.get() == 0 {
        format!(
            "{}さん、ようこそ残業ギルドへ！今は依頼が１件もないみたいですね。追加したい場合はメニューから依頼登録してくださいね。",
            hardworker.get().unwrap().name
        )
    } else {
        format!(
            "{}さん、受注している依頼が{}件あるみたいね...しっかり終わらせて報告してくださいね",
            hardworker.get().unwrap().name,
            task_count.get(),
        )
    };
    let message = RwSignal::new(Message::new("レーナ".to_string(), welcome_message));

    view! {
        <main>

        <div class="zentas-main">
        <MenuBarComponent scene=scene/>

                // === 背景 ===
                <img src="public/assets/backgrounds/guild_inside.png" class="zentas-bg" />

                // === 掲示板 ===
                <BoardComponent tasks=tasks/>

                // === 受付嬢（立ち絵） ===
                <img src={move || format!("public/assets/characters/{}", receptionist.get())}
                class="zentas-person" />
        // === セリフウィンドウ ===
        <WindowMessage message={ message }/>


    </div>

    </main>
    }
}
