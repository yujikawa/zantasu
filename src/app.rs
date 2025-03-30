use crate::main;
use crate::scenes::guild::GuildScene;
use crate::scenes::register::RegisterScene;
use crate::scenes::start::StartScene;
use leptos::ev::Event;
use leptos::task::{self, spawn_local};
use leptos::{children, logging};
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Copy, PartialEq)]
pub enum Scene {
    Start,
    Register,
    Guild,
}

#[derive(Deserialize, Serialize, Debug)]
struct Quest {
    title: String,
    description: String,
    rank: String,
    due_date: String,
}

impl Quest {
    fn new(title: String, description: String, rank: String, due_date: String) -> Self {
        return Quest {
            title,
            description,
            rank,
            due_date,
        };
    }
}

#[component]
pub fn App() -> impl IntoView {
    // scene切り替え用
    let scene = RwSignal::new(Scene::Start);

    // let content = match scene.get() {
    //     Scene::Start => <GuildScene />
    //     Scene::Register => <GuildScene />
    //     Scene::Guild => <GuildScene />
    // };
    view! {
        <div>
        <Show
        when=move || scene.get() == Scene::Start
        fallback=|| ()>
        <StartScene />
            // === スタートボタン ===
            <button
                style="
                    position: absolute; 
                    left: 50%;
                    bottom: 100px;
                    transform: translateX(-50%);
                    padding: 16px 32px;
                    font-size: 24px;
                    border-radius: 8px;
                "
                on:click=move |_| scene.set(Scene::Register)
            >
                "スタート"
            </button>
        </Show>

        <Show
        when=move || scene.get() == Scene::Guild
        fallback=|| ()>
        <GuildScene />

        </Show>

        <Show
        when=move || scene.get() == Scene::Register
        fallback=|| ()>
        <RegisterScene />
        <button
        style="
            position: absolute; 
            left: 50%;
            bottom: 100px;
            transform: translateX(-50%);
            padding: 16px 32px;
            font-size: 24px;
            border-radius: 8px;
        "
        on:click=move |_| scene.set(Scene::Guild)
        >
        "登録"
    </button>
        </Show>

        </div>
    }

    // let welcome_message =
    //     "ようこそ、残業ギルドへ！掲示板をクリックするとクエストが見れます！".to_string();
    // let (is_show_task_list, set_show_task_list) = signal(false);
    // let (message, set_message) = signal(welcome_message);
    // let (person_character, set_person_character) = signal("normal_stand.png");

    // let tasks = vec!["薬草採取".to_string(), "ゴブリン退治".to_string()];

    // let (task_list, set_task_list) = signal(tasks);

    // let show_task_list = move |_ev: leptos::ev::MouseEvent| {
    //     // let v = event_target_value(&ev);
    //     logging::log!("clicked");
    //     set_show_task_list.set(true);
    //     logging::log!("{:?}", is_show_task_list.get());
    //     set_message
    //         .set("あなたの受注しているクエストの確認や新たにクエストを登録できますけど..まだ仕事するの..?".to_string());
    //     set_person_character.set("normal.png");
    // };

    // let close_task_list = move |_ev: leptos::ev::MouseEvent| {
    //     // let v = event_target_value(&ev);
    //     set_show_task_list.set(false);
    //     set_message
    //         .set("ようこそ、残業ギルドへ！掲示板をクリックするとクエストが見れます！".to_string());
    //     set_person_character.set("normal_stand.png");
    // };

    // view! {
    //         <main>

    //         <div style="position: fixed; top:0; left:0; right:0; bottom:0; overflow: hidden;">

    //                 // === 背景 ===
    //                 <img src="public/assets/backgrounds/guild_inside.png"
    //                     style="position: absolute; width: 100%; height: 100%; object-fit: cover;" />

    //                 // === 掲示板 ===
    //                 <img src="public/assets/objects/board.png"
    //                     style="position: absolute; left: 50px; top: 140px; width: 500px;"
    //                     on:click=show_task_list
    //                     />

    //                 // === アリナ（立ち絵） ===
    //                 <img src={move || format!("public/assets/characters/{}", person_character.get())}
    //                     style="position: absolute; right: 50px; bottom: 0; height: 600px;" />
    //         // === セリフウィンドウ ===
    //         <div style="
    //                 position: absolute;
    //                 left: 50%;
    //                 bottom: 20px;
    //                 transform: translateX(-50%);
    //                 width: 80%;
    //                 background: rgba(31, 29, 29, 0.7);
    //                 border: 2px solid #000;
    //                 border-radius: 10px;
    //                 padding: 16px;
    //                 font-size: 20px;
    //                 ">
    //         { move || message.get() }
    //             </div>
    //         <Show
    //             when=move || is_show_task_list.get()
    //             fallback=|| ()>
    //             <div style="position: absolute; top:100px; left:10px; padding:10px; width:500px; height:500px; background:rgba(31, 29, 29, 0.7);">
    //             <h2>"依頼一覧"</h2>
    //             <ul>
    //                 <For
    //                     each=move || task_list.get()
    //                     key=|task| task.clone() // task自体がキー
    //                     children=move |task| view! {
    //                         <li>{task}</li>
    //                     }
    //                 />
    //             </ul>
    //             <button style="background:rgba(220, 90, 90, 0.7);" on:click=close_task_list>"閉じる"</button>
    //             <button style="background:rgba(90, 116, 220, 0.7);margin-left:10px;" on:click=close_task_list>"クエスト登録"</button>

    //             </div>
    //         </Show>

    //     </div>
    //     </main>
    // }
}
