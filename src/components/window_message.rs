use leptos::prelude::*;

#[component]
pub fn WindowMessage(message: RwSignal<String>) -> impl IntoView {
    view! {
        // <div style="
        //             position: absolute;
        //             left: 50%;
        //             bottom: 20px;
        //             transform: translateX(-50%);
        //             width: 80%;
        //             background: rgba(31, 29, 29, 0.7);
        //             border: 2px solid #000;
        //             border-radius: 10px;
        //             padding: 16px;
        //             font-size: 20px;
        //             ">
        // {message.get()}
        //     </div>

        <div class="message-window">
            <div class="name-box">"アリナ"</div>
            <div class="text-box">
            {message.get()}
            </div>
        </div>
    }
}
