use leptos::prelude::*;

use crate::models::message::Message;

#[component]
pub fn WindowMessage(message: RwSignal<Message>) -> impl IntoView {
    view! {
        <div class="message-window">
            <div class="name-box">{ move || message.get().name }</div>
            <div class="text-box">
            {move || message.get().text }
            </div>
        </div>
    }
}
