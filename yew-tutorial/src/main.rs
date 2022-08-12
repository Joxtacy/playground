use yew::prelude::*;
use serde::Deserialize;
use reqwasm::{websocket::{Message, futures::WebSocket}, http::Request};

use futures::{sink::SinkExt, stream::StreamExt};

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        };

        html! {
            <p onclick={on_video_select}>{ format!("{}: {}", video.speaker, video.title) }</p>
        }
    }).collect()
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {

    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(move |_| {
            let ws = WebSocket::open("ws://localhost:8081/ws").unwrap();

            let (tx, rx) = ws.split();

            wasm_bindgen_futures::spawn_local(async move {
                let mut sender = tx;

                match sender.send(Message::Text("henlo".to_string())).await {
                    Ok(_) => gloo_console::log!("sent"),
                    Err(e) => gloo_console::error!(format!("sending error: {}", e)),
                };
            });

            wasm_bindgen_futures::spawn_local(async move {
                let mut receiver = rx;

                while let Some(val) = receiver.next().await {
                    gloo_console::log!(format!("{:?}", val));
                }
            });

            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });

            || () // Returned `destructor` for `use_effect_with_deps`
        }, ());
    }

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };

    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select} />
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
