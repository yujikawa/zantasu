use crate::models::hard_worker::Job;
use leptos::prelude::*;

#[component]
pub fn JobProfileCard(job: Job) -> impl IntoView {
    view! {
        <div class="job-profile-card">
            // <img src={job.icon_path()} alt={job.to_str()} class="job-icon"/>
            <h2>{job.to_str()}</h2>
            <h3>{job.title()}</h3>
            <p>{job.description()}</p>
        </div>
    }
}
