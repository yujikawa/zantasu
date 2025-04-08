use crate::models::task::{SchedulePattern, ScheduledTaskFormState};
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum RepeatType {
    Monthly,
    Weekly,
    Daily,
}

#[component]
pub fn ScheduledTaskModalComponent<F, C>(
    form: RwSignal<ScheduledTaskFormState>,
    on_submit: F,
    submit_label: &'static str,
    on_cancel: C,
) -> impl IntoView
where
    F: Fn() + Clone + 'static,
    C: Fn() + Clone + 'static,
{
    let repeat_type: RwSignal<RepeatType> = RwSignal::new(
        if matches!(form.get().pattern, SchedulePattern::Monthly { .. }) {
            RepeatType::Monthly
        } else if matches!(form.get().pattern, SchedulePattern::Weekly { .. }) {
            RepeatType::Weekly
        } else {
            RepeatType::Daily
        },
    );

    view! {
        <div class="modal-backdrop">
            <div class="modal-content">
                <h2>"定期依頼の登録"</h2>
                <label style="color: black">"依頼タイトルを入力"</label>

                <input
                    type="text"
                    placeholder="依頼タイトルを入力"
                    prop:value=move || form.get().task.title
                    on:input=move |ev| {
                        let mut f = form.get();
                        f.task.title = event_target_value(&ev);
                        form.set(f);
                    }
                />
                <label style="color: black">"詳細を入力"</label>

                    <textarea
                    placeholder="詳細を入力"
                        prop:value=move || form.get().task.description
                        on:input=move |ev| {
                            let mut f = form.get();
                            f.task.description = event_target_value(&ev);
                            form.set(f);
                        }
                    />

                <div class="scheduled-task-form-input">
                    <label style="color: black">"繰り返しタイプ"</label>
                    <select
                    on:change=move |e| {
                        let value = event_target_value(&e);
                        let mut f = form.get();

                        match value.as_str() {
                            "Monthly" => {
                                repeat_type.set(RepeatType::Monthly);
                                f.pattern = SchedulePattern::Monthly {
                                    day: 1,
                                    time: "09:00".to_string(),
                                };
                            }
                            "Weekly" => {
                                repeat_type.set(RepeatType::Weekly);
                                f.pattern = SchedulePattern::Weekly {
                                    weekday: 1,
                                    time: "09:00".to_string(),
                                };
                            }
                            "Daily" => {
                                repeat_type.set(RepeatType::Daily);
                                f.pattern = SchedulePattern::Daily {
                                    time: "09:00".to_string(),
                                };
                            }
                            _ => {}
                        }

                        form.set(f);
                        // match event_target_value(&e).as_str() {
                        //     "Monthly" => repeat_type.set(RepeatType::Monthly),
                        //     "Weekly" => repeat_type.set(RepeatType::Weekly),
                        //     "Daily" => repeat_type.set(RepeatType::Daily),
                        //     _ => {},
                        // }
                    }>
                        <option value="Monthly">"毎月"</option>
                        <option value="Weekly">"毎週"</option>
                        <option value="Daily">"毎日"</option>
                    </select>
                </div>

                <Show when=move || repeat_type.get() == RepeatType::Monthly>
                    <div class="scheduled-task-form-input">
                        <label style="color: black">"毎月何日？（1〜31）"</label>
                        <input type="number" min="1" max="31"
                        prop:value=move || match form.get().pattern {
                            SchedulePattern::Monthly {day, ..} => day,
                            _ => 1
                        }
                        on:input=move |e| {
                            if let Ok(val) = event_target_value(&e).parse::<u32>() {
                                let mut f = form.get();
                                if let SchedulePattern::Monthly { time, ..} = f.pattern {
                                    f.pattern = SchedulePattern::Monthly { day: val, time: time }
                                };
                                form.set(f);
                            }
                        } />
                        <label style="color: black">"何時？（例: 10:00）"</label>
                        <input type="time"
                        prop:value=move || match form.get().pattern {
                            SchedulePattern::Monthly {time, ..} => time,
                            _ => "09:00".to_string()
                        }
                        on:input=move |e| {
                            let mut f = form.get();
                            if let SchedulePattern::Monthly { day, ..} = f.pattern {
                                f.pattern = SchedulePattern::Monthly { day: day, time: event_target_value(&e) }
                            };
                            form.set(f);
                        }/>
                    </div>
                </Show>

                <Show when=move || repeat_type.get() == RepeatType::Weekly>
                    <div class="scheduled-task-form-input">
                        <label style="color: black">"曜日（0=日〜6=土）"</label>
                        <input type="number" min="0" max="6"
                        prop:value=move || match form.get().pattern {
                            SchedulePattern::Weekly {weekday, ..} => weekday,
                            _ => 1
                        }
                        on:input=move |e| {
                            if let Ok(val) = event_target_value(&e).parse::<u32>() {

                                let mut f = form.get();
                                if let SchedulePattern::Weekly { time, ..} = f.pattern {
                                    f.pattern = SchedulePattern::Weekly { weekday: val, time: time }
                                };
                                form.set(f);

                            }
                        } />
                        <label style="color: black">"何時？"</label>
                        <input type="time"
                        prop:value=move || match form.get().pattern {
                            SchedulePattern::Weekly {time, ..} => time,
                            _ => "09:00".to_string()
                        }
                        on:input=move |e| {
                            let mut f = form.get();
                            if let SchedulePattern::Weekly { weekday, ..} = f.pattern {
                                f.pattern = SchedulePattern::Weekly { weekday: weekday, time: event_target_value(&e) }
                            };
                            form.set(f);
                        }/>
                    </div>
                </Show>

                <Show when=move || repeat_type.get() == RepeatType::Daily>
                    <div class="scheduled-task-form-input">
                        <label  style="color: black">"何時？"</label>
                        <input type="time"
                        prop:value=move || match form.get().pattern {
                            SchedulePattern::Daily {time, ..} => time,
                            _ => "09:00".to_string()
                        }
                        on:input=move |e| {
                            let mut f = form.get();
                            f.pattern = SchedulePattern::Daily { time: event_target_value(&e) };
                            form.set(f);
                        }/>
                    </div>
                </Show>

                <div class="modal-buttons">

                    <button on:click=move |_| on_cancel()
                    >
                        "キャンセル"
                    </button>

                    <button on:click=move |_| on_submit()>
                        {submit_label}
                    </button>

                </div>

            // mordal
            </div>
        </div>



    }
}
