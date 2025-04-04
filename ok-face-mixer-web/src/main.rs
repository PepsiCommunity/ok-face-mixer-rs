use std::str::FromStr;

use leptos::prelude::*;
use log::info;
use ok_face_mixer_core::{Smile, SmileType};
use stringcase::Caser;
use strum::IntoEnumIterator;
use stylers::style_sheet;
use wasm_bindgen_futures::JsFuture;

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
            <select
                on:change:target = move |e| {
                    info!("target changed to {}", e.target().value());
                    set_smile.set(SmileType::from_str(e.target().value().as_str()).unwrap());
                }
            >
                {SmileType::iter().map(|x| view! {<option value=x.to_string()>{x.to_string().as_str().to_pascal_case()}</option>}).collect::<Vec<_>>()}
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

        <label>"Top"</label>
        {smile_selector(set_left_smile)}

        <label>"Bottom"</label>
        {smile_selector(set_right_smile)}

        <button on:click = move |_| {
            set_left_smile.set(rand::random());
            set_right_smile.set(rand::random());
        }>"Randomize smile"</button>

        <img src = api_path/>

        <div>
            <p id="url">{api_path}</p>

            <button on:click = move |_| wasm_bindgen_futures::spawn_local(async move {
                info!("copying \"{}\" to clipboard", api_path());
                JsFuture::from(web_sys::window().expect("get window").navigator().clipboard().write_text(&api_path())).await.expect("copy to clipboard error");
            })>"Copy"</button>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);

    mount_to_body(App);
}
