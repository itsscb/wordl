use web_sys::wasm_bindgen::convert::OptionIntoWasmAbi;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::{classes, function_component, html, Callback, Html};

use crate::CharStatus;

fn check_game_over(words: &[CharStatus<String>]) -> bool {
    if words.iter().all(|i| matches!(i, CharStatus::Match(_))) {
        return true;
    }
    false
}

fn set_focus(index: usize) {
    if let Some(next) = web_sys::window()
        .expect("no global 'window' exists")
        .document()
        .expect("should have a document on window")
        .query_selector(&format!("[tabindex='{}']", index))
        .ok()
        .flatten()
    {
        if let Some(e) = next.dyn_ref::<HtmlElement>() {
            e.focus().ok();
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
                    _ => {
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

#[function_component]
pub fn Home() -> Html {
    let length = 5;
    let got_word = "HALLO";
    let submitted_words = yew::use_state(std::vec::Vec::new);

    let node_refs = use_state(|| vec![NodeRef::default(); length]);
    let input_values = use_state(|| vec!["".to_string(); length]);
    let game_over = use_state(|| false);
    let game_over_check = {
        let submitted_words = submitted_words.clone();
        let game_over = game_over.clone();
        Callback::from(move |_| {
            if submitted_words.iter().count() >= length - 1 {
                game_over.set(true);
            }
        })
    };

    got_word.chars().for_each(|_| {
        let input_values = input_values.clone();
        let mut values = (*input_values).clone();
        values.push("".to_string());

        let node_refs = node_refs.clone();
        let mut values = (*node_refs).clone();
        values.push(NodeRef::default());
    });

    let on_submit = {
        let input_values = input_values.clone();
        let submitted_words = submitted_words.clone();
        let game_over = game_over.clone();
        let game_over_check = game_over_check.clone();
        Callback::from(move |_e: MouseEvent| {
            if *game_over {
                submitted_words.set(vec![]);
                game_over.set(false);
                return;
            }
            let values: Vec<_> = input_values.iter().cloned().collect();
            if !values.iter().all(|v| !v.is_empty()) {
                return;
            }
            let mut new_items = (*submitted_words).clone();
            new_items.push(crate::compare_strings(got_word, &values.join("")));
            submitted_words.set(new_items);
            game_over_check.emit(MouseEvent::none());
        })
    };

    let on_enter = {
        let input_values = input_values.clone();
        let submitted_words = submitted_words.clone();
        let game_over = game_over.clone();
        let game_over_check = game_over_check.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if *game_over {
                    submitted_words.set(vec![]);
                    game_over.set(false);
                    return;
                }
                let values: Vec<_> = input_values.iter().cloned().collect();
                if !values.iter().all(|v| !v.is_empty()) {
                    return;
                }
                let mut new_items = (*submitted_words).clone();
                new_items.push(crate::compare_strings(got_word, &values.join("")));
                submitted_words.set(new_items);
                game_over_check.emit(KeyboardEvent::none());
            }
        })
    };

    let view = {
        let node_refs = node_refs.clone();
        let input_values = input_values.clone();
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
                                    { node_refs.iter().enumerate().map(|(index, node_ref)| {
                                        let on_input = {
                                            let node_ref = node_ref.clone();
                                            let next_index = index +1;
                                            let input_values = input_values.clone();
                                            Callback::from(move |event: InputEvent| {
                                                let value = event.data().unwrap();
                                                let mut values = (*input_values).clone();
                                                values[index] = value.to_uppercase();
                                                input_values.set(values);
                                                if let Some(input) = node_ref.cast::<web_sys::HtmlInputElement>() {
                                                    input.value();
                                                    set_focus(next_index);
                                                }
                                            })
                                        };
                                        let on_enter = on_enter.clone();
                                        html! {
                                            <input
                                            onkeypress={on_enter}
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
                                    }).collect::<Html>() }
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
