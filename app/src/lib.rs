use leptos::*;

/// A simple counter component.
///
/// You can use doc comments like this to document your component.
#[component]
pub fn SimpleView(

    url: String,

) -> impl IntoView {
    //let (value, set_value) = create_signal(initial_value);

    view! {
        <div>
            <iframe src={url} id="myIframe" title="description" style="width: 100%;height: 800px;"></iframe> 
        //     <button on:click=move |_| set_value.set(0)>"Clear"</button>
        //     <button on:click=move |_| set_value.update(|value| *value -= step)>"-1"</button>
        //     <span>"Value: " {value} "!"</span>
        //     <button on:click=move |_| set_value.update(|value| *value += step)>"+1"</button>
        </div>
    }
}