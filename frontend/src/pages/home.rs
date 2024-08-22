use gloo_console::log;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::convert::OptionIntoWasmAbi;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::{classes, function_component, Callback, Html};
// use yew::{FetchTask, Request, Response, FetchService};

use crate::CharStatus;

static NEW_WORD_URI: &str = "https://wordl.shuttleapp.rs/word";

fn set_focus(index: usize) {
    if let Some(w) = web_sys::window() {
        if let Some(d) = w.document() {
            if let Some(n) = d
                .query_selector(&format!("[tabindex='{index}']"))
                .ok()
                .flatten()
            {
                if let Some(e) = n.dyn_ref::<HtmlElement>() {
                    e.focus().ok();
                }
            }
        }
    }
}

// fn string_to_html(input: &Vec<CharStatus<String>>) -> Html {
fn string_to_html(input: &[CharStatus<String>]) -> Html {
    let classes = classes!(
        "bg-gray-700",
        "w-16",
        "h-16",
        "text-center",
        "py-4",
        "font-bold",
        "text-lg",
    );
    html! {
            <ul
                class={
                    classes!(
                        "flex",
                        "flex-row",
                        "gap-4",
                        "mt-8",
                        "notranslate",
                    )
                }
            >
        {
            input.iter().map(|e|{
                let mut classes = classes.clone();

                let text = match e {
                    CharStatus::Match(s) => {
                        classes.push("bg-green-400");
                        s
                    },
                    CharStatus::Contained(s) => {
                        classes.push("bg-yellow-400");
                        s
                    },
                    CharStatus::NotContained(s) => {
                        classes.push("bg-gray-700");
                        classes.push("border-white");
                        classes.push("border-2");
                        s
                    }
                    CharStatus::Unknown => {
                        ""
                    },
                };
                html!{
               <li
                    class={
                        classes!(
                            "flex",
                            "items-center"
                        )
                    }
               >
               <span
               class={
                   classes.clone()
               }
           >
               {text}
               </span>
           </li>
        }}).collect::<Html>()
        }
        </ul>
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Word(String);

fn get_word(handle: UseStateHandle<String>) {
    use_effect_with((), move |()| {
        // let word = handle.clone();
        wasm_bindgen_futures::spawn_local(async move {
            log!("retrieving word");
            let res = Request::get(NEW_WORD_URI).send().await;
            log!(format!("RESPONSE: {res:?}"));
            match res {
                Ok(r) => {
                    log!(format!("{r:?}"));
                    match r.body() {
                        Some(j) => {
                            let j = j.to_string();
                            let data = format!("{j:?}");
                            log!(&data);
                            handle.set(data);
                        }
                        None => log!("no body"),
                    }
                }
                Err(e) => log!(format!("FETCH: {e:?}")),
            }
            // let res: Word = Request::get(NEW_WORD_URI).send().await.unwrap().json().await.unwrap();
            // log!(res);
            // handle.set(res.0);
            // if let Ok(r) = res {
            //     if let Some(w) = r.body() {

            //         word.set(w.to_string());
            //     }
            // }
        });
        || ()
    });
}

#[function_component]
pub fn Home() -> Html {
    // let got_word = "HALLO";
    // let length = got_word.len();
    log!("STARTING");

    let word: UseStateHandle<String> = use_state(String::new);

    // get_word(word.clone());

    {
        let handle = word.clone();
        use_effect_with((), move |()| {
            // let word = handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                log!("retrieving word");
                let res = Request::get(NEW_WORD_URI).send().await;
                log!(format!("RESPONSE: {res:?}"));
                match res {
                    // Ok(r) => {
                    //     log!(format!("{r:?}"));
                    //     match r.json::<Word>().await {
                    //         Ok(j) => {
                    //             let data = format!("{j:?}");
                    //             log!(&data);
                    //             handle.set(data);
                    //         },
                    //         Err(e) => log!(format!("{e:?}")),
                    //     }
                    // },
                    Ok(r) => {
                        log!(format!("{r:?}"));
                        match r.body() {
                            Some(j) => {
                                let j = j.to_string();
                                let data = format!("{j:?}");
                                log!(&data);
                                handle.set(data);
                            }
                            None => log!("no body"),
                        }
                    }
                    Err(e) => log!(format!("FETCH: {e:?}")),
                }
                // let res: Word = Request::get(NEW_WORD_URI).send().await.unwrap().json().await.unwrap();
                // log!(res);
                // handle.set(res.0);
                // if let Ok(r) = res {
                //     if let Some(w) = r.body() {

                //         word.set(w.to_string());
                //     }
                // }
            });
            || ()
        });
    }
    let length = word.len();

    let submitted_words: UseStateHandle<Vec<Vec<CharStatus<String>>>> =
        use_state(|| std::vec::Vec::with_capacity(length));

    let node_refs = use_state(|| vec![NodeRef::default(); length]);
    let input_values: UseStateHandle<Vec<String>> = use_state(|| vec![String::new(); length]);
    let game_over = use_state(|| false);
    let game_over_check = {
        let word = word.clone();
        let submitted_words = submitted_words.clone();
        let iv = input_values.clone();
        let game_over = game_over.clone();
        Callback::from(move |_| {
            if submitted_words.iter().count() >= length - 1
                || crate::compare_strings(&word, &iv.join(""))
                    .iter()
                    .all(|v| matches!(v, CharStatus::Match(_)))
            {
                game_over.set(true);
            }
        })
    };

    let on_submit = {
        let input_values = input_values.clone();
        let submitted_words = submitted_words.clone();
        let game_over = game_over.clone();
        let word = word.clone();
        Callback::from(move |_e: MouseEvent| {
            if *game_over {
                submitted_words.set(std::vec::Vec::with_capacity(length));
                input_values.set(vec![String::new(); length]);
                game_over.set(false);
                return;
            }
            let values: Vec<_> = input_values.iter().cloned().collect();
            if !values.iter().all(|v| !v.is_empty()) {
                return;
            }
            let mut new_items = (*submitted_words).clone();
            new_items.push(crate::compare_strings(&word, &values.join("")));
            submitted_words.set(new_items);
            game_over_check.emit(MouseEvent::none());
        })
    };

    let on_enter = {
        let on_submit = on_submit.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Ok(m) = MouseEvent::new("click") {
                    on_submit.emit(m);
                }
            }
        })
    };

    let view = {
        move || {
            html! {
                <div
                    class={
                        classes!(
                            "flex",
                            "flex-col",
                            "mt-12",
                            "items-center",
                            "h-screen",
                        )
                    }
                >
                <h1>{(*word).clone()}</h1>
                <div
                    class="h-4/6 flex flex-col"
                >
                <form
                class="order-last mt-8"
                >
                <div
                    class={
                        classes!(
                            "flex",
                            "flex-row",
                            "font-bold",
                            "text-lg",
                            "gap-4",
                        )
                    }
                >
                    {
                        if !*game_over && !word.is_empty() {
                            node_refs.iter().enumerate().map(|(index, node_ref)| {
                                let on_input = {
                                    let node_ref = node_ref.clone();
                                    let next_index = index +1;
                                    let input_values = input_values.clone();
                                    Callback::from(move |event: InputEvent| {
                                        if let Some(value) = event.data() {
                                            let mut values = (*input_values).clone();
                                            values[index] = value.to_uppercase();
                                            input_values.set(values);
                                            if let Some(input) = node_ref.cast::<web_sys::HtmlInputElement>() {
                                                input.value();
                                                set_focus(next_index);
                                            }
                                        }
                                    })
                                };
                                html! {
                                    <input
                                        onkeypress={on_enter.clone()}
                                            tabindex={index.to_string()}
                                            ref={node_ref.clone()}
                                            value={input_values[index].clone()}
                                            oninput={on_input}
                                        class={
                                            classes!(
                                                "w-16",
                                                "h-16",
                                                "text-center",
                                                "bg-gray-600"
                                            )
                                        }
                                    />
                                }
                            }).collect::<Html>()
                        } else {
                            html!(<div></div>)
                        }
                    }
                </div>
                </form>
                <div class="!order-first">
                { for submitted_words.iter().map(|e| {string_to_html(e)})}
                </div>
                    </div>
                        <button
                        tabindex={"5"}
                        class={
                            classes!(
                                "w-72",
                                "h-16",
                                "text-2xl",
                                "font-bold",
                                "rounded-xl",
                                "bg-green-700",
                                "order-last",
                            )
                        }
                        onclick={on_submit} type="submit">
                        {
                            if *game_over {
                                "Play again"
                            }
                            else {
                                "Submit"
                            }
                        }
                        </button>
                </div>
            }
        }
    };

    view()
}
