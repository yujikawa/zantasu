use leptos::prelude::*;

use crate::app::Scene;

#[component]
pub fn MenuBarComponent(scene: RwSignal<Scene>) -> impl IntoView {
    view! {
        <div class="menu-bar">

            <button class="menu-button" on:click=move |_| scene.set(Scene::Guild)>
                <img src="public/assets/logo/home_icon.png" />
                <span>"ホーム"</span>
            </button>

            <button class="menu-button" on:click=move |_| scene.set(Scene::TaskList)>
                <img src="public/assets/logo/task_list_icon.png" />
                <span>"依頼一覧"</span>
            </button>

            <button class="menu-button" on:click=move |_| scene.set(Scene::TaskRegister)>
                <img src="public/assets/logo/task_register_icon.png"/>
                <span>"依頼登録"</span>
            </button>

            <button class="menu-button" on:click=move |_| scene.set(Scene::ScheduleTaskRegister)>
            <img src="public/assets/logo/scheduled_task_register_icon.png"/>
                <span>"定期登録"</span>
            </button>

            <button class="menu-button" on:click=move |_| scene.set(Scene::Status)>
                <img src="public/assets/logo/status_icon.png"/>
                <span>"ステータス"</span>
            </button>

            <button class="menu-button" on:click=move |_| scene.set(Scene::Finish)>
                <img src="public/assets/logo/finish_icon.png"/>
                <span>"業務終了"</span>
            </button>

        </div>
    }
}
