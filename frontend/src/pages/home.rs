use wordl::Game;
use yew::{function_component, html, use_state, Html, UseStateHandle};

#[function_component]
pub fn Home() -> Html {
    let game: UseStateHandle<Game> = use_state(Game::default);
    html! {
        game.get_submitted_words().iter().map(|c| html!{<p>{format!("{c:?}")}</p>}).collect::<Html>()
    }
}
