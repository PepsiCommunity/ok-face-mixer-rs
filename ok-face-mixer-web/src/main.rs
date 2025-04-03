use std::str::FromStr;

use leptos::prelude::*;
use log::info;
use ok_face_mixer_core::{Smile, SmileType};
use stylers::style;

const API_PATH: &str = "https://mix.andcool.ru/api/mix_image.gif";

#[component]
fn app() -> impl IntoView {
    info!("rendering app");

    let (left_smile, set_left_smile) = signal(SmileType::Grin);
    let (right_smile, set_right_smile) = signal(SmileType::Grin);

    let style = style! { "app",
    body {
        background: #151517;
        margin: 0;
        color: white;
        font-family: Inter, sans-serif;
        text-align: center;
        padding: 1rem;

        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }

    input,
    select {
        padding: .5rem;
        border-radius: 10px;
        outline: none;
        border: none;
        background-color: #1E1E20;
        border: 1px #1E1E20 solid;
        color: inherit;
        width: 15rem;
        box-sizing: border-box;
        transition: border .25s;
        height: 2.4rem;
    }

    input:focus,
    select:focus {
        border: 1px #666666 solid;
    }

    button {
        padding: .8rem;
        border-radius: 10px;
        border: none;
        background-color: #1E1E20;
        transition: background-color .2s;
        color: white;
        cursor: pointer;
        white-space: nowrap;
        width: 15rem;
        box-sizing: border-box;
    }

    button:hover {
        background-color: #37373b;
    }

    #url {
        border-radius: 10px;
        background-color: #1E1E20;
        border: 1px #313131 solid;
        font-size: .9rem;
        padding: .5rem;
    }
    };

    let smile_selector = move |set_smile: WriteSignal<SmileType>| {
        info!("rendering smile selector");

        view! {
            <label>"Top"</label>
            <select
                on:change:target = move |e| {
                    info!("target changed to {}", e.target().value());
                    set_smile.set(SmileType::from_str(e.target().value().as_str()).unwrap());
                }
            >
                <option value="flush">"Flush"</option>
                <option value="he">"He"</option>
                <option value="mad">"Mad"</option>
                <option value="plead">"Plead"</option>
                <option value="sad">"Sad"</option>
                <option value="sg">"Sg"</option>
                <option value="shock">"Shock"</option>
                <option value="sl_smile">"Sl Smile"</option>
                <option value="sleep">"Sleep"</option>
                <option value="smiley">"Smiley"</option>
                <option value="tong">"Tong"</option>
                <option value="unamus">"Unamus"</option>
                <option value="wink">"Wink"</option>
                <option value="zany">"Zany"</option>
            </select>
        }
    };

    let api_path = move || {
        format!(
            "{}{}",
            API_PATH,
            &Smile::new(left_smile.get(), right_smile.get()).api_query()
        )
    };

    view! { class = style,
        <h1>"OK Emoji Mixer"</h1>

        {smile_selector(set_left_smile)}
        {smile_selector(set_right_smile)}

        <img src = api_path/>

        <div>
            <p id="url">{api_path}</p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);

    mount_to_body(App);
}
