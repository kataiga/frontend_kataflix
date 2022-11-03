pub mod client;
pub mod components;
pub mod layouts;

fn main() {
    yew::start_app::<client::Client>();
}
