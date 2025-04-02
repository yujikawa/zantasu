use leptos::prelude::*;

use crate::app::Scene;
use crate::models::task::Task;

#[component]
pub fn BoardComponent(tasks: RwSignal<Option<Vec<Task>>>) -> impl IntoView {
    let task_count = RwSignal::new(tasks.get().unwrap().len());

    view! {
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
    }
}
