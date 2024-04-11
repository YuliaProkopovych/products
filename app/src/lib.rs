use leptos::*;

#[component]
pub fn SimpleView(
    url: String,
) -> impl IntoView {
    view! {
        <div>
            <iframe src={url} id="myIframe" title="description" style="width: 100%;height: 800px;"></iframe> 
        </div>
    }
}