use std::time::Duration;

use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::wasm_bindgen::convert::OptionIntoWasmAbi;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::platform::time::sleep;
use yew::prelude::*;
use yew::{classes, function_component, Callback, Html};

use crate::pages::game::GameResult;
use crate::CharStatus;

use super::game::WordList;

static NEW_WORD_URI: &str = "https://wordl.shuttleapp.rs/word";
static WORDS_URI: &str = "https://wordl.shuttleapp.rs/public/wordlist.json";
static MAX_TRIES: usize = 5;

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

fn string_to_html(input: &[CharStatus<String>]) -> Html {
    let classes = classes!(
        "bg-gray-700",
        "w-16",
        "h-16",
        "text-center",
        "py-4",
        "font-bold",
        "text-lg",
        "mb-4",
    );
    html! (
            <ul
                class={
                    classes!(
                        "flex",
                        "flex-row",
                        "gap-4",
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
                        classes.push("bg-gray-900");
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
    )
}

#[allow(clippy::too_many_arguments)]
fn fetch_new_word(
    word_list: &UseStateHandle<WordList>,
    word: &UseStateHandle<String>,
    loading: &UseStateHandle<bool>,
    submitted_words: &UseStateHandle<Vec<Vec<CharStatus<String>>>>,
    input_values: &UseStateHandle<Vec<String>>,
    game_over: &UseStateHandle<bool>,
    length: &UseStateHandle<usize>,
    node_refs: &UseStateHandle<Vec<NodeRef>>,
    result: &UseStateHandle<GameResult>,
) {
    let loading = loading.clone();
    let submitted_words = submitted_words.clone();
    let input_values = input_values.clone();
    let game_over = game_over.clone();
    let length = length.clone();
    let node_refs = node_refs.clone();
    let result = result.clone();
    let word = word.clone();
    // let word_list = word_list.clone();
    wasm_bindgen_futures::spawn_local(async move {
        loading.set(true);
        let res = Request::get(NEW_WORD_URI).send().await;
        if let Ok(r) = res {
            if let Ok(w) = r.text().await {
                length.set(w.len());
                node_refs.set(vec![NodeRef::default(); w.len()]);
                input_values.set(vec![String::new(); w.len()]);
                word.set(w.to_uppercase());
                submitted_words.set(Vec::with_capacity(MAX_TRIES));
                game_over.set(false);
                result.set(GameResult::Lose);
                loading.set(false);
                // sleep(Duration::from_secs(5)).await;
                // word.set(word_list.get_word().to_uppercase());
            }
        }
    });
}

fn fetch_words(state: &UseStateHandle<WordList>) {
    let state = state.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let res = Request::get(WORDS_URI).send().await;
        if let Ok(r) = res {
            if let Ok(w) = r.text().await {
                state.set(WordList::from_json(&w));
            }
        }
    });
}

#[function_component]
pub fn Home() -> Html {
    let word_list: UseStateHandle<WordList> = use_state(WordList::new);

    let word: UseStateHandle<String> = use_state(String::new);
    let loading: UseStateHandle<bool> = use_state(|| true);
    let curr_index: UseStateHandle<usize> = use_state(|| 0usize);

    let length = use_state(|| 0usize);
    let submitted_words: UseStateHandle<Vec<Vec<CharStatus<String>>>> =
        use_state(|| std::vec::Vec::with_capacity(MAX_TRIES));

    let node_refs = use_state(|| vec![NodeRef::default(); 10]);
    let input_values: UseStateHandle<Vec<String>> = use_state(|| vec![String::new(); *length]);
    let game_over = use_state(|| false);

    let result = use_state(|| GameResult::Lose);

    {
        let word_list = word_list.clone();
        let wl = word_list.clone();
        let handle = word.clone();
        let loading = loading.clone();

        let submitted_words = submitted_words.clone();
        let input_values = input_values.clone();
        let game_over = game_over.clone();
        let length = length.clone();
        let node_refs = node_refs.clone();
        let result = result.clone();

        // use_effect_with((), move |()| {
        //     spawn_local(async move {
        //         fetch_words(&wl);
        //     });
        // });

        use_effect_with((), move |()| {
            // sleep(Duration::from_millis(200));
            fetch_new_word(
                &word_list,
                &handle,
                &loading,
                &submitted_words,
                &input_values,
                &game_over,
                &length,
                &node_refs,
                &result,
            );
        });
    }

    let game_over_check = {
        let word = word.clone();
        let submitted_words = submitted_words.clone();
        let iv = input_values.clone();
        let game_over = game_over.clone();
        let length = length.clone();
        let result = result.clone();

        Callback::from(move |_| {
            if submitted_words.iter().count() >= *length - 1
                || crate::compare_strings(&word, &iv.join(""))
                    .iter()
                    .all(|v| matches!(v, CharStatus::Match(_)))
            {
                if crate::compare_strings(&word, &iv.join(""))
                    .iter()
                    .all(|v| matches!(v, CharStatus::Match(_)))
                {
                    result.set(GameResult::Win);
                }
                game_over.set(true);
            }
        })
    };

    let on_disabled = {
        let curr_index = curr_index.clone();
        let input_values = input_values.clone();

        Callback::from(move |_e: MouseEvent| {
            let index = input_values.iter().enumerate().find(|(_, v)| v.is_empty()).map_or(0, |(i,_)| i);
            set_focus(index);
            curr_index.set(index);
        })
    };

    let on_submit = {
        let input_values = input_values.clone();
        let submitted_words = submitted_words.clone();
        let game_over = game_over.clone();
        let length = length.clone();
        let word = word.clone();
        let node_refs = node_refs.clone();
        let loading = loading.clone();
        let result = result.clone();
        let curr_index = curr_index.clone();
        let word_list = word_list.clone();

        Callback::from(move |_e: MouseEvent| {
            if *game_over {
                curr_index.set(0);
                let word_list = word_list.clone();
                let input_values = input_values.clone();
                let submitted_words = submitted_words.clone();
                let game_over = game_over.clone();
                let length = length.clone();
                let word = word.clone();
                let loading = loading.clone();
                let node_refs = node_refs.clone();
                let result = result.clone();
                fetch_new_word(
                    &word_list,
                    &word,
                    &loading,
                    &submitted_words,
                    &input_values,
                    &game_over,
                    &length,
                    &node_refs,
                    &result,
                );
                return;
            }
            let values: Vec<_> = input_values.iter().cloned().collect();
            if !values.iter().all(|v| !v.is_empty()) {
                return;
            }
            let mut new_items = (*submitted_words).clone();
            new_items.push(crate::compare_strings(&word, &values.join("")));
            submitted_words.set(new_items);
            input_values.set(vec![String::new(); word.len()]);
            set_focus(0);
            curr_index.set(0);
            game_over_check.emit(MouseEvent::none());
        })
    };
    let on_enter = {
        let on_submit = on_submit.clone();
        let curr_index = curr_index.clone();
        let node_refs = node_refs.clone();
        let input_values = input_values.clone();
        let length = length.clone();

        Callback::from(move |e: KeyboardEvent| match e.key().as_ref() {
            "Enter" => {
                if let Ok(m) = MouseEvent::new("click") {
                    on_submit.emit(m);
                }
            }
            "Backspace" => {
                e.prevent_default();

                let mut index = *curr_index;
                let mut values = (*input_values).clone();

                if index >= *length {
                    curr_index.set(*length - 1);
                    index = *length - 1;
                }

                if node_refs[index]
                    .cast::<web_sys::HtmlInputElement>()
                    .is_some()
                    && index > 0
                {
                    values[index] = String::new();
                    input_values.set(values);
                    let index = index - 1;
                    curr_index.set(index);
                    set_focus(index);
                }
            }
            _ => {}
        })
    };

    let on_input = {
        let curr_index = curr_index.clone();
        let length = length.clone();
        let input_values = input_values.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(value) = e.data() {
                let value = value.to_uppercase();
                let index = *curr_index;
                let mut values = (*input_values).clone();

                if index >= *length {
                    values[index - 1] = value;
                    input_values.set(values);
                } else if value.len() < values[index].len() && index > 0 && index <= *length {
                    values[index] = String::new();
                    input_values.set(values);
                    let new_index = index - 1;
                    curr_index.set(new_index);
                    set_focus(new_index);
                } else if value.len() == 1 && value.chars().all(char::is_alphabetic) {
                    values[index] = value;
                    input_values.set(values);
                    if index < *length {
                        let new_index = index + 1;
                        curr_index.set(new_index);
                        set_focus(new_index);
                    }
                } else {
                    values[index] = String::new();
                    input_values.set(values);
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
                            "items-center",
                            "h-[90vh]",
                            "pt-12",
                        )
                    }
                >
                // <h1>{format!("{:?}",*word_list)}</h1>
                <div
                    class={
                        classes!(
                            "h-5/6",
                            "flex",
                            "flex-col",
                            "items-center",
                            "pt-12",
                        )
                    }
                >

                    <div class={
                        classes!(
                            "mb-12",
                        )}>
                    { for submitted_words.iter().map(|e| {string_to_html(e)})}
                    </div>
                    <form
                    class="mb-4"
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
                                if *loading {
                                    html!(<p>{"Loading..."}</p>)
                                }
                                else if *game_over {

                                    let (text, color) = match *result {
                                        GameResult::Win => {
                                            ("FOUND", "bg-green-600")
                                        },
                                        GameResult::Lose => {
                                            ("WANTED", "bg-red-600")
                                        }
                                    };
                                    html! (
                                        <div>
                                        <h1>{
                                            text
                                        }</h1>
                                            <ul
                                                class={
                                                    classes!(
                                                        "flex",
                                                        "flex-row",
                                                        "gap-4",
                                                        "notranslate",
                                                    )
                                                }
                                            >
                                        {
                                            word.chars().map(|e|{

                                                let text = e;
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
                                                classes!(
                                                    "w-16",
                                                    "h-16",
                                                    "text-center",
                                                    "py-4",
                                                    "font-bold",
                                                    "text-lg",
                                                    {color},
                                                )
                                            }
                                        >
                                            {text}
                                            </span>
                                        </li>
                                        }}).collect::<Html>()
                                        }
                                        </ul>
                                        </div>
                                    )
                                }
                                else if !*game_over {
                                    node_refs.iter().enumerate().map(|(index, node_ref)| {
                                        let on_focus = {
                                            let curr_index = curr_index.clone();

                                            Callback::from(move |e: FocusEvent| {
                                                let target = e.target_unchecked_into::<web_sys::HtmlElement>();
                                                if let Some(index) = target.get_attribute("tabindex") {
                                                    if let Ok(i) = index.parse::<usize>() {
                                                        curr_index.set(i);
                                                    }
                                                }

                                            })
                                        };
                                        html! {
                                            <input
                                                onkeyup={on_enter.clone()}
                                                oninput={on_input.clone()}
                                                tabindex={index.to_string()}
                                                ref={node_ref.clone()}
                                                value={input_values[index].clone()}
                                                onfocus={on_focus.clone()}
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
                    {
                        if *loading {
                            html!{<></>}
                        } else {
                            html!{
                                <div
                                class={
                                    classes!(
                                        "w-full",
                                        "flex",
                                        "justify-end",

                                    )
                                }
                            >
                                <button
                                // disabled={input_values.iter().any(std::string::String::is_empty)}
                                tabindex={(*length + 1).to_string()}
                                class={
                                    classes!(
                                        "w-24",
                                        "h-16",
                                        "text-2xl",
                                        "font-bold",
                                        "rounded-xl",
                                        "flex",
                                        "items-center",
                                        "justify-center",
                                        {if input_values.iter().any(std::string::String::is_empty) && !*game_over {"bg-gray-700"} else {"bg-green-600"}},
                                    )
                                }
                                onclick={if input_values.iter().any(std::string::String::is_empty) && !*game_over {on_disabled} else {on_submit}} type="submit">
                                {
                                    if *game_over {
                                        html!{
                                            <svg xmlns="http://www.w3.org/2000/svg" class="w-12 h-12" viewBox="0 -960 960 960" fill="white">
                                                <path d="M480-80q-75 0-140.5-28.5t-114-77q-48.5-48.5-77-114T120-440h80q0 117 81.5 198.5T480-160q117 0 198.5-81.5T760-440q0-117-81.5-198.5T480-720h-6l62 62-56 58-160-160 160-160 56 58-62 62h6q75 0 140.5 28.5t114 77q48.5 48.5 77 114T840-440q0 75-28.5 140.5t-77 114q-48.5 48.5-114 77T480-80Z"/>
                                            </svg>
                                        }
                                    }
                                    else if input_values.iter().any(std::string::String::is_empty) {
                                        html!{
                                            <svg xmlns="http://www.w3.org/2000/svg" class="w-12 h-12" viewBox="0 -960 960 960" width="24px" fill="white">
                                                <path d="M480-80q-83 0-156-31.5T197-197q-54-54-85.5-127T80-480q0-83 31.5-156T197-763q54-54 127-85.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 83-31.5 156T763-197q-54 54-127 85.5T480-80Zm0-80q54 0 104-17.5t92-50.5L228-676q-33 42-50.5 92T160-480q0 134 93 227t227 93Zm252-124q33-42 50.5-92T800-480q0-134-93-227t-227-93q-54 0-104 17.5T284-732l448 448Z"/>
                                            </svg>
                                        }
                                    } else {
                                        html!{
                                            <svg xmlns="http://www.w3.org/2000/svg" class="w-12 h-12" viewBox="0 -960 960 960" fill="white">
                                                <path d="m424-296 282-282-56-56-226 226-114-114-56 56 170 170Zm56 216q-83 0-156-31.5T197-197q-54-54-85.5-127T80-480q0-83 31.5-156T197-763q54-54 127-85.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 83-31.5 156T763-197q-54 54-127 85.5T480-80Zm0-80q134 0 227-93t93-227q0-134-93-227t-227-93q-134 0-227 93t-93 227q0 134 93 227t227 93Zm0-320Z"/>
                                            </svg>
                                        }
                                    }
                                }
                                </button>
                            </div>
                            }
                        }
                    }

                </div>
            </div>
            }
        }
    };

    view()
}
