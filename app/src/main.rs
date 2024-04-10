use counter::SimpleView;
use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <SimpleView
                url=String::from("http://localhost:8000/?url=https://www.nike.com/w/")
            />
        }
    })
}