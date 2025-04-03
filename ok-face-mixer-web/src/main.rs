use std::str::FromStr;

use leptos::prelude::*;
use log::info;
use ok_face_mixer_core::{Smile, SmileType};
use stylers::style_sheet;

const API_PATH: &str = "https://mix.andcool.ru/api/mix_image.gif";

#[component]
fn app() -> impl IntoView {
    info!("rendering app");

    let (left_smile, set_left_smile) = signal(SmileType::Grin);
    let (right_smile, set_right_smile) = signal(SmileType::Grin);

    let style = style_sheet!("ok-face-mixer-web/styles.css");

    let smile_selector = move |set_smile: WriteSignal<SmileType>| {
        info!("rendering smile selector");

        view! { class = style,
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
