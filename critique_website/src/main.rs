use dioxus::logger::tracing;
use dioxus::prelude::*;

use critique_api::MediaUniverse;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/theme.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        ScPicker {}
    }
}

#[component]
fn ScPicker() -> Element {
    let mut username = use_signal(String::new);
    let mut count = use_signal(|| 3);
    let media_type = MediaUniverse::Film;
    let mut picks = use_signal(Vec::new);

    let roll_wheel = move || async move {
        if username().is_empty() {
            return;
        }

        let client = critique_api::CritiqueClient::new();

        let Ok(res) = client
            .get_user_collection(&username(), None, None, Some(media_type))
            .await
        else {
            return;
        };
        let Some(collection) = res.user.and_then(|u| u.collection?.products) else {
            return;
        };
        let mut wishlist = collection
            .into_iter()
            .filter_map(|m| {
                let m = m?;
                let _ = m.medias.as_ref()?.picture.as_ref()?;
                let _ = m.medias.as_ref()?.picture.as_ref()?;
                if m.other_user_infos.as_ref()?.is_wished {
                    Some(m)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        picks.clear();
        for _ in 0..count() {
            let r = js_sys::Math::random() * wishlist.len() as f64;
            let pick = wishlist.remove(r as usize);
            picks.push(pick);
        }
        tracing::info!("picks: {:?}", picks);
    };

    rsx! {
        div { class: "pt-5 h-screen w-full flex flex-col justify-start items-center gap-3",
            h1 { class: "text-2xl mb-5",
                "SensCritique Random Picker"
            }
            input { class: "input",
                placeholder: "SensCritique username",
                oninput: move |e| {
                    username.set(e.value());
                },
                autofocus: "true",
            }
            div { class: "flex justify-center content-center items-center gap-1",
                p { "Number of picks: "}
                input {
                    class: "input w-20",
                    r#type: "number",
                    value: "{count}",
                    oninput: move |e| {
                        if let Ok(value) = e.value().parse::<usize>() {
                            count.set(value);
                        }
                    },
                }
            }

            button {
                class: "button mt-5",
                onclick: move |_| roll_wheel(),
                "Random Picks"
            }

            div { class: "flex gap-2",
                for pick in picks.iter() {
                    div { class: "flex flex-col",
                        p { class: "text-xl", "{pick.title"}
                        if let Some(year) = pick.year_of_production {
                            p { "Year: {year}" }
                        }
                        a { href: "https://senscritique.com{pick.url}" }
                        img { src: pick.medias.as_ref().unwrap().picture.as_ref().unwrap().clone() }
                    }
                }
            }
        }
    }
}
