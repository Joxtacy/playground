use yew::prelude::*;

enum Msg {
    AddOne,
}

struct CounterComponent {
    count: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::AddOne => {
                self.count += 1;
                true // re-render the conponent
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <Henlo name={"Wurl".to_owned()} />
                <p>{ self.count }</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
struct HenloProperties {
    name: String,
}

#[function_component(Henlo)]
fn app(HenloProperties { name }: &HenloProperties) -> Html {
    html! {
        <h1>{ format!("Henlo {}", name) }</h1>
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
