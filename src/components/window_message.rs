use leptos::prelude::*;

#[component]
pub fn WindowMessage(message: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="message-window">
            <div class="name-box">"アリナ"</div>
            <div class="text-box">
            {move || message.get()}
            </div>
        </div>
    }
}
