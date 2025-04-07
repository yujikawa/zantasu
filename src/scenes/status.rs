use crate::app::Scene;
use crate::components::board::BoardComponent;
use crate::components::job_profile_card::JobProfileCard;
use crate::components::menu_bar::MenuBarComponent;
use crate::components::radar_chart::RadarChart;
use crate::components::window_message::WindowMessage;
use crate::models::hard_worker::{HardWorker, Job};
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
    let job = hardworker.get().unwrap().job.unwrap_or(Job::Jobless);
    let message = RwSignal::new(Message::new(
        hardworker.get().unwrap().name,
        "ステータスオープン！".to_string(),
    ));

    let stats = job.stats();

    view! {
        <main>
            <div class="zantas-main">
                <MenuBarComponent scene=scene/>
                <img src="public/assets/backgrounds/guild_inside.png" class="zantas-bg" />
                <BoardComponent tasks=tasks/>

                <div class="status-window">
                    // <div class="status-list">"【 ステータス一覧 】"</div>
                    <div class="status-content">
                        <div class="status-left">
                            <div class="status-item">
                                <label>名前</label>
                                <p>{hardworker.get().unwrap().name}</p>
                            </div>

                            <div class="status-item">
                                <label>労働等級</label>
                                <p>{hardworker.get().unwrap().rank.to_str()}</p>
                            </div>

                            <div class="status-item">
                            <label>職業</label>
                            <JobProfileCard job=job/>
                        </div>


                    </div>

                        <div class="status-right">
                            <div class="status-item">
                                <label>能力値</label>
                                <RadarChart stats=stats/>
                            </div>

                            <div class="status-item">
                                <label>現在の依頼受注数</label>
                                <p>{task_count.get()} 件</p>
                            </div>

                            <div class="status-item">
                                <label>依頼達成数</label>
                                <p>{hardworker.get().unwrap().achievement} 件</p>
                            </div>

                            <div class="status-item">
                                <label>依頼達成最終日</label>
                                <p>{
                                    let last_complete = match hardworker.get().unwrap().last_complete {
                                        Some(date) => date,
                                        None => "実績なし".to_string(),
                                    };
                                    last_complete
                                }</p>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="thought-bubble">
                    "何をみているんだろう..?"
                </div>
                <img src="public/assets/characters/status_open.png" class="status-open" />

                <img src="public/assets/characters/receptionist/curious.png" class="zantas-left-person"/>
                <WindowMessage message={message}/>
            </div>
        </main>
    }
}
