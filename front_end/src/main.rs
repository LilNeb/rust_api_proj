use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let selected = use_state(|| "nothing".to_string());
    let src = use_state(|| "img/plot.png".to_string());

    let on_change = {
        let selected = selected.clone();
        let src = src.clone();
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            selected.set(input.value());
            src.set(format!("img/{}-plot.png", input.value())); // Update the src state
        })
    };

    html! {
        <div>
            <h1>{"Exchange price visualizer"}</h1>
            <select name="exchange" id="exchange-select" onchange={on_change} style="font-size: 16px; margin: 10px;">
              <option value="">{"Choose an exchange"}</option>
              <option value="Kraken">{"Kraken"}</option>
              <option value="Bitfinex">{"Bitfinex"}</option>
              <option value="Cex">{"Cex"}</option>
              <option value="Coinbase">{"Coinbase"}</option>
              <option value="Binance">{"Binance"}</option>
              <option value="Huobi">{"Huobi"}</option>
              <option value="Kucoin">{"Kucoin"}</option>
              <option value="Gate">{"Gate"}</option>
              <option value="All">{"All"}</option>
            </select>
            <img src={(*src).clone()} style="width: 80%; height: 1920; width: 1080px; display: block; margin: 0 auto;"/>
        </div>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
