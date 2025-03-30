use leptos::prelude::*;

#[component]
pub fn StartScene() -> impl IntoView {
    view! {
        <div style="position: fixed; top:0; left:0; right:0; bottom:0; overflow: hidden;">
            // === 背景 ===
            <img src="public/assets/backgrounds/guild_outside.png"
                style="position: absolute; width: 100%; height: 100%; object-fit: cover;" />

            // === タイトルロゴ ===
            <img src="public/assets/logo/title.png"
                style="position: absolute; left: 50%; top: -80px; transform: translateX(-50%); width: 400px;" />


        </div>
    }
}
