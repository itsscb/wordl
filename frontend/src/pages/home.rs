use yew::prelude::*;
use yew::{classes, function_component, html, Callback, Html, NodeRef};

use crate::router::Route;

use crate::CharStatus;

fn string_to_html(input: &Vec<CharStatus<String>>) -> Html {
    let classes = classes!("p-3");
    html! {
            <ul
                class={
                    classes!(
                        "flex",
                        "flex-row",
                        "justify-between",
                        "w-72",
                        "mx-12"
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
                };
                html!{
           <li
               class={
                   classes.clone()
               }
           >
               {text}
           </li>
        }}).collect::<Html>()
        }
        </ul>
    }
}

#[function_component]
pub fn Home() -> Html {
    let got_word = "HALLO";
    let mut submitted_words = yew::use_state(|| vec![]);
    let input = yew::use_state(|| {
        vec![
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
        ]
    });

    let on_submit = {
        let input = input.clone();
        let submitted_words = submitted_words.clone();
        Callback::from(move |event: MouseEvent| {
            let mut new_items = (*submitted_words).clone();
            new_items.push(crate::compare_strings(&got_word, &input.join("")));
            submitted_words.set(new_items);
        })
    };

    let editing_index = yew::use_state(|| None);
    let editing_value = yew::use_state(|| String::new());

    let on_click = {
        let editing_index = editing_index.clone();
        let editing_value = editing_value.clone();
        let input = input.clone();
        Callback::from(move |index: usize| {
            editing_index.set(Some(index));
            editing_value.set(input.to_vec()[index].clone());
        })
    };

    let on_input = {
        let editing_value = editing_value.clone();
        Callback::from(move |value: String| {
            editing_value.set(value);
        })
    };

    let on_blur = {
        let editing_index = editing_index.clone();
        let editing_value = editing_value.clone();
        let input = input.clone();
        Callback::from(move |_| {
            if let Some(index) = *editing_index {
                let mut new_input = input.to_vec();
                new_input[index] = editing_value.to_uppercase().to_string();
                input.set(new_input);
            }
            editing_index.set(None);
        })
    };

    // let mut submitted_words = vec![];

    let res = crate::compare_strings(got_word, "HLLAI");
    html! {
        <div
            class={
                classes!(
                    "w-full",
                    "flex",
                    "flex-col",
                    "mt-6",
                    "input-center"
                )
            }
        >
        {submitted_words.iter().map(|w| string_to_html(&w)).collect::<Html>()}
        // <div>{format!("{:?}",res)}</div>
            // <div
            //     class={
            //         classes!(
            //             "w-full",
            //             "h-16"
            //         )
            //     }
            // >{
            //     got_word
            // }</div>
            <ul
                class={
                    classes!(
                        "flex",
                        "flex-row",
                        "justify-between",
                        "w-72",
                        "h-16",
                        "mx-12"
                    )
                }
            >
            { input.to_vec().iter().enumerate().map(|(index, item)| {
                if let Some(editing_idx) = *editing_index {
                    if editing_idx == index {
                        html! {
                            <li
                            class="w-12 bg-gray-600 flex items-center justify-center"
                            >
                                <input
                                    class="bg-gray-600 h-full w-full"
                                    type="text"
                                    value={editing_value.to_string()}
                                    onblur={on_blur.clone()}
                                    oninput={on_input.reform(|e: InputEvent| e.data().unwrap_or_default())}
                                />
                            </li>
                        }
                    } else {
                        html! { <li

                            class="w-4 bg-gray-600"
                            >{item}</li> }
                    }
                } else {
                    html! { <li
                            class="w-12 bg-gray-600"
                         onclick={on_click.reform(move |_| index)}>{item}</li> }
                }
            }).collect::<Html>() }
        </ul>
        //     <input
        //         value={got_word}
        //         type="text"
        //         class={
        //             classes!(
        //                 "w-full",
        //                 "h-16"
        //             )
        //         }
        //     />
            <button onclick={on_submit}>{"Submit"}</button>
        </div>
    }
}
