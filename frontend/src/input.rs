use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputStringProps {
    pub value: String,
}

pub enum Msg {
    CharInput(usize, String),
}

pub struct InputString {
    value: String,
    nodes: Vec<NodeRef>,
    focused_index: usize,
}

impl Component for InputString {
    type Message = Msg;
    type Properties = InputStringProps;

    fn create(ctx: &Context<Self>) -> Self {
        let value = ctx.props().value.clone();
        let nodes = vec![NodeRef::default(); value.len()];
        Self {
            value,
            nodes,
            focused_index: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CharInput(index, new_char) => {
                let mut new_value = self.value.clone();
                new_value.replace_range(index..index + 1, &new_char);
                self.value = new_value;

                if index < self.value.len() - 1 {
                    self.focused_index = index + 1;
                    if let Some(next_node) = self.nodes.get(self.focused_index) {
                        if let Some(input) = next_node.cast::<HtmlInputElement>() {
                            input.focus().unwrap();
                        }
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let chars = self.value.chars().enumerate().map(|(index, char)| {
            let on_input = ctx.link().callback(move |input: InputEvent| {
                let new_char = input.data();
                Msg::CharInput(index, new_char.unwrap())
            });

            html! {
                <input
                    type="text"
                    maxlength=1
                    value={char.to_string()}
                    oninput={on_input}
                    class="w-12 h-16 text-center"
                    ref={self.nodes.get(index).unwrap().clone()}
                    style={if index == self.focused_index { "background-color: yellow;" } else { "" }}
                />
            }
        });

        html! {
            <div style="display: flex; gap: 0.5rem;">
                { for chars }
            </div>
        }
    }
}
