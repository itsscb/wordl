use web_sys::wasm_bindgen::convert::OptionIntoWasmAbi;
use yew::prelude::*;
use yew::{classes, function_component, html, Callback, Html};

use crate::CharStatus;

fn string_to_html(input: &Vec<CharStatus<String>>) -> Html {
    let classes = classes!(
        "bg-gray-700",
        "w-16",
        "h-16",
        "text-center",
        "py-4",
        // "justify-center",
        // // "justify-items-center",
        // "object-center",
        // "items-center",
        // "leading-tight"
    );
    html! {
            <ul
                class={
                    classes!(
                        "flex",
                        "flex-row",
                        "gap-8",
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
    let got_word = "HALLO";
    let submitted_words = yew::use_state(std::vec::Vec::new);

    let node_refs = use_state(|| vec![NodeRef::default(); 5]);
    let input_values = use_state(|| vec!["".to_string(); 5]);
    let game_over = use_state(|| false);
    let game_over_check = {
        let submitted_words = submitted_words.clone();
        let game_over = game_over.clone();
        Callback::from(move |_| {
            if submitted_words.iter().count() >= 4 {
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
        Callback::from(move |_| {
            if *game_over {
                submitted_words.set(vec![]);
                // input_values.set(vec![]);
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
                "mt-32",
                // "justify-center",
                // "justify-items-center",
                "items-center",
                "h-screen",
                                )
                            }
                        >
                        <div
                            class="h-4/6"
                        >
                            <form
                            // onsubmit={on_submit}
                            >
                            <div
                            class={
                                classes!(
                                    "flex",
                                    "flex-row",
                                    "gap-8",
                                )
                            }
                            >
                                { node_refs.iter().enumerate().map(|(index, node_ref)| {
                                    let on_input = {
                                        let node_ref = node_ref.clone();
                                        let input_values = input_values.clone();
                                        Callback::from(move |event: InputEvent| {
                                            let value = event.data().unwrap();
                                            let mut values = (*input_values).clone();
                                            values[index] = value.to_uppercase();
                                            input_values.set(values);
                                            if let Some(input) = node_ref.cast::<web_sys::HtmlInputElement>() {
                                                input.value();
                                            }
                                        })
                                    };
                                    html! {
                <div
                    class="flex gap-8"
                >
                                        <input
                                            ref={node_ref.clone()}
                                            value={input_values[index].clone()}
                                            oninput={on_input}
                            class={
                                classes!(
                                    "w-16",
                                    "h-16",
                                    "flex-1",
                                    "text-center",
                                    // "px-4",
                                    // "py-2",
                                    "bg-gray-600"
                                )
                            }
                                        />
                                        </div>
                                    }
                                }).collect::<Html>() }
                                </div>
                            </form>
                { for submitted_words.iter().map(string_to_html)}
                </div>
                                <button
            class={
                classes!(
                    "w-72",
                    "h-16",
                    // "mt-24",
                    "text-2xl",
                    "font-bold",
                    "rounded-xl",
                    "bg-green-700",
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
    // html! {
    // <div
    //     class={
    //         classes!(
    // "mt-[15%]",
    // "flex",
    // "flex-col",
    // "justify-center",
    // "items-center"
    //         )
    //     }
    // >
    // <div
    //     class={
    //         classes!(
    //             "flex",
    //             "flex-row",
    //             "gap-8"
    //         )
    //     }
    // >
    // {view()}
    // { for input.iter().map(|i| {
    //     html!{
    //         <input
    //             class={
    //                 classes!(
    //                     "w-16",
    //                     "h-16",
    //                     "bg-gray-600"
    //                 )
    //             }
    //             value={<std::string::String as Clone>::clone(&*i)}
    //         />
    //     }
    // })}
    // </div>
    // <InputString value={"     ".to_string()}/>
    // <button
    //     class={
    //         classes!(
    //             "w-72",
    //             "h-16",
    //             "mt-24",
    //             "text-2xl",
    //             "font-bold",
    //             "rounded-xl",
    //             "bg-green-700",
    //         )
    //     }
    //  onclick={on_submit}>{"Submit"}</button>
    // </div>
    // }
}
