use gloo_net::http::Request;
use web_sys::wasm_bindgen::convert::OptionIntoWasmAbi;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::{classes, function_component, Callback, Html};

use crate::pages::game::GameResult;
use crate::CharStatus;

static NEW_WORD_URI: &str = "https://wordl.shuttleapp.rs/word";
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
    );
    html! (
            <ul
                class={
                    classes!(
                        "flex",
                        "flex-row",
                        "gap-4",
                        "mt-4",
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
    word: &UseStateHandle<String>,
    loading: &UseStateHandle<bool>,
    submitted_words: &UseStateHandle<Vec<Vec<CharStatus<String>>>>,
    input_values: &UseStateHandle<Vec<String>>,
    game_over: &UseStateHandle<bool>,
    length: &UseStateHandle<usize>,
    node_refs: &UseStateHandle<Vec<NodeRef>>,
    result: &UseStateHandle<GameResult>,
) {
    let handle = word.clone();
    let loading = loading.clone();
    let submitted_words = submitted_words.clone();
    let input_values = input_values.clone();
    let game_over = game_over.clone();
    let length = length.clone();
    let node_refs = node_refs.clone();
    let result = result.clone();

    wasm_bindgen_futures::spawn_local(async move {
        loading.set(true);
        let res = Request::get(NEW_WORD_URI).send().await;
        if let Ok(r) = res {
            if let Ok(w) = r.text().await {
                length.set(w.len());
                node_refs.set(vec![NodeRef::default(); w.len()]);
                input_values.set(vec![String::new(); w.len()]);
                handle.set(w.to_uppercase());
                submitted_words.set(Vec::with_capacity(MAX_TRIES));
                game_over.set(false);
                result.set(GameResult::Lose);
                loading.set(false);
            }
        }
    });
}

#[function_component]
pub fn Home() -> Html {
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
        let handle = word.clone();
        let loading = loading.clone();

        let submitted_words = submitted_words.clone();
        let input_values = input_values.clone();
        let game_over = game_over.clone();
        let length = length.clone();
        let node_refs = node_refs.clone();
        let result = result.clone();

        use_effect_with((), move |()| {
            fetch_new_word(
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

        Callback::from(move |_e: MouseEvent| {
            if *game_over {
                curr_index.set(0);
                let input_values = input_values.clone();
                let submitted_words = submitted_words.clone();
                let game_over = game_over.clone();
                let length = length.clone();
                let word = word.clone();
                let loading = loading.clone();
                let node_refs = node_refs.clone();
                let result = result.clone();
                fetch_new_word(
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
        let curr_index = curr_index.clone();
        let length = length.clone();
        let on_submit = on_submit.clone();
        let input_values = input_values.clone();
        let node_refs = node_refs.clone();

        Callback::from(move |e: KeyboardEvent| {
            match e.key().as_ref() {
                "Enter" => {
                    if let Ok(m) = MouseEvent::new("click") {
                        on_submit.emit(m);
                    }
                },
                "Backspace" => {
                    e.prevent_default();

                    let index = *curr_index;
                    let mut values = (*input_values).clone();
                                            
                    values[index] = String::new();
                    input_values.set(values);
                    if node_refs[index].cast::<web_sys::HtmlInputElement>().is_some() && index > 0 {
                        let index = index - 1;
                        curr_index.set(index);
                        set_focus(index);
                    }
                },
                k if k.len() == 1 && k.chars().all(char::is_alphabetic) => {
                    let index = *curr_index;
                    let mut values = (*input_values).clone();
                                            
                    values[index] = k.to_uppercase();
                    input_values.set(values);
                    if node_refs[index].cast::<web_sys::HtmlInputElement>().is_some() && index < *length {
                        let index = index + 1;
                        curr_index.set(index);
                        set_focus(index);
                    }
                },
                _ => {
                    let index = *curr_index;
                    let mut values = (*input_values).clone();
                    
                    values[index] = String::new();
                    input_values.set(values);
                },
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
                            "mt-4",
                            "items-center",
                            "h-[90vh]",
                        )
                    }
                >
                <div
                    class="h-5/6 flex flex-col"
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
