use critique_api::user_collection_query::{
    UserCollectionQueryUserCollectionProducts, UserCollectionQueryUserCollectionProductsMedias,
    UserCollectionQueryUserCollectionProductsOtherUserInfos,
};
use dioxus::{logger::tracing, prelude::*};

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
                p { class: "text-secondary",
                    "Number of picks: "
                }
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
                class: "button",
                "data-style": "outline",
                onclick: move |_| roll_wheel(),

                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "24",
                    height: "24",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",

                    path {
                        d: "M2 18h1.4c1.3 0 2.5-.6 3.3-1.7l6.1-8.6c.7-1.1 2-1.7 3.3-1.7H22",
                    }
                    path {
                        d: "m18 2 4 4-4 4",
                    }
                    path {
                        d: "M2 6h1.9c1.5 0 2.9.9 3.6 2.2",
                    }
                    path {
                        d: "M22 18h-5.9c-1.3 0-2.6-.7-3.3-1.8l-.5-.8",
                    }
                    path {
                        d: "m18 14 4 4-4 4",
                    }
                }
            }

            div { class: "flex gap-2 mt-3",
                for pick in picks.iter() {
                    a {
                        class: "group rounded-2xl overflow-hidden shadow-md hover:shadow-xl transition duration-300 transform hover:-translate-y-1",
                        href: "https://senscritique.com{pick.url}",

                        div { class: "flex flex-col h-full",
                            img {
                                src: pick.medias.as_ref().unwrap().picture.as_ref().unwrap().clone(),
                                class: "w-[300px] h-[400px] self-center object-cover transition duration-300 group-hover:scale-105"
                            }
                            div { class: "p-3 flex flex-col flex-1",
                                p { class: "text-lg font-semibold text-primary group-hover:text-indigo-300 transition", "{pick.title}"}
                                if let Some(year) = pick.year_of_production {
                                    p { class: "text-sm text-gray-400", "{year}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
