mod components;
mod util;

use components::login::Login;
use types::LoginStatus;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// fn get_value_from_input_event(e: InputEvent) -> String {
//     let event: Event = e.dyn_into().unwrap_throw();
//     let event_target = event.target().unwrap_throw();
//     let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
//     // web_sys::console::log_1(&target.value().into());
//     target.value()
// }

// #[derive(Default, Serialize, Deserialize)]
// struct Data {
//     user_list: Vec<User>,
// }

#[derive(Default)]
struct App {
    data: LoginStatus,
    loginned: bool,
}

enum AppMsg {
    LoginStatus(LoginStatus),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::LoginStatus(data) => {
                self.data = data;
                self.loginned = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                if !self.loginned {
                    <Login on_data = {ctx.link().callback(move |data| AppMsg::LoginStatus(data))} />
                } else {
                    {
                        format!("{}", match self.data {
                            LoginStatus::Accepted => "AC",
                            LoginStatus::RepeatLogin => "RE",
                            LoginStatus::WrongPassword => "WA",
                        })
                    }
                }
                // <p>
                    // <button onclick = {move |_| }>{ "Load server api" }</button>
                // </p>
                // {
                //     if let Some(response) = &state.data {
                //         html! {
                //             <p>{ "From backend: " }<b>{ &response.name }</b></p>
                //         }
                //     } else {
                //         html! {}
                //     }
                // }
                // {
                //     if let Some(response) = &state_server.data {
                //         html! {
                //             <p>{ "From server: " }<b>{ &response }</b></p>
                //         }
                //     } else {
                //         html! {}
                //     }
                // }
                // <p>
                //     <button onclick={onopen} disabled={*ws.ready_state != UseWebSocketReadyState::Closed}>{ "Connect to backend websocket" }</button>
                //     <button onclick={onclick2} disabled={*ws.ready_state != UseWebSocketReadyState::Open}>{ "Send to backend websocket" }</button>
                // </p>
                // {
                //     for history.current().iter().map(|message| {
                //         html! {
                //             <p>{ message }</p>
                //         }
                //     })
                // }
            </>
        }
    }
}

// #[function_component(App)]
// fn app() -> Html {
//     // Get backend port automatically from tauri command.
//     let port = use_async_with_options(
//         async move {
//             match get_port().await {
//                 Ok(p) => Ok(p.as_string().unwrap()),
//                 Err(e) => Err(format!("Error: {:?}", e)),
//             }
//         },
//         UseAsyncOptions::enable_auto(),
//     );

//     // Fetch data from backend.
//     let state = {
//         let port = port.clone();
//         use_async(async move {
//             match &port.data {
//                 Some(port) => {
//                     let response = reqwest::get(format!("http://localhost:{}/user", port)).await;
//                     match response {
//                         Ok(data) => match data.json::<UserInfo>().await {
//                             Ok(user) => Ok(user),
//                             Err(_) => Err("Backend body Error".to_owned()),
//                         },
//                         Err(_) => Err("Backend request Error".to_owned()),
//                     }
//                 }
//                 _ => Err("Backend is unavailable".to_owned()),
//             }
//         })
//     };

//     let onclick = {
//         let state = state.clone();
//         Callback::from(move |_| {
//             state.run();
//         })
//     };

//     // Fetch data from server.
//     let state_server = use_async(async move {
//         let poster = reqwest::Client::new()
//             .post("http://localhost:3001/login")
//             .body(
//                 json!({
//                     "id": 1,
//                 })
//                 .to_string(),
//             )
//             .send()
//             .await;
//         match poster {
//             Ok(data) => match data.text().await {
//                 Ok(data) => Ok(data),
//                 Err(_) => Err("Body Error".to_string()),
//             },
//             Err(_) => Err("Request Error".to_string()),
//         }
//         // let response = reqwest::get("http://localhost:3001/user").await;
//         // match response {
//         //     Ok(data) => match data.json::<UserInfo>().await {
//         //         Ok(user) => Ok(user),
//         //         Err(_) => Err("Body Error".to_string()),
//         //     },
//         //     Err(_) => Err("Request Error".to_string()),
//         // }
//     });

//     let onclickserver = {
//         let state_server = state_server.clone();
//         Callback::from(move |_| {
//             state_server.run();
//         })
//     };

//     let history = use_list(vec![]);

//     // Manually connect to websocket with custom options.
//     let ws = {
//         let history = history.clone();
//         let port = port.data.clone().unwrap_or_default();
//         use_web_socket_with_options(
//             format!("ws://localhost:{}/ws", port),
//             UseWebSocketOptions {
//                 // Receive message by callback `onmessage`.
//                 onmessage: Some(Box::new(move |message| {
//                     history.push(format!("ws [recv]: {}", message));
//                 })),
//                 manual: Some(true),
//                 ..Default::default()
//             },
//         )
//     };
//     let onclick2 = {
//         let ws = ws.clone();
//         let history = history.clone();
//         Callback::from(move |_| {
//             let message = "Hello, backend!".to_string();
//             ws.send(message.clone());
//             history.push(format!("ws [send]: {}", message));
//         })
//     };
//     let onopen = {
//         let ws = ws.clone();
//         Callback::from(move |_| {
//             ws.open();
//         })
//     };

//     html! {
//         <>
//             <Login />
//             <p>
//                 <button {onclick}>{ "Load backend api" }</button>
//                 <button onclick={onclickserver}>{ "Load server api" }</button>
//             </p>
//             {
//                 if let Some(response) = &state.data {
//                     html! {
//                         <p>{ "From backend: " }<b>{ &response.name }</b></p>
//                     }
//                 } else {
//                     html! {}
//                 }
//             }
//             {
//                 if let Some(response) = &state_server.data {
//                     html! {
//                         <p>{ "From server: " }<b>{ &response }</b></p>
//                     }
//                 } else {
//                     html! {}
//                 }
//             }
//             <p>
//                 <button onclick={onopen} disabled={*ws.ready_state != UseWebSocketReadyState::Closed}>{ "Connect to backend websocket" }</button>
//                 <button onclick={onclick2} disabled={*ws.ready_state != UseWebSocketReadyState::Open}>{ "Send to backend websocket" }</button>
//             </p>
//             {
//                 for history.current().iter().map(|message| {
//                     html! {
//                         <p>{ message }</p>
//                     }
//                 })
//             }
//         </>
//     }
// }

#[wasm_bindgen(module = "/public/tauri.js")]
extern "C" {
    /// Get backend port from tauri commands.
    #[wasm_bindgen(js_name = getPort, catch)]
    pub async fn get_port() -> Result<JsValue, JsValue>;
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
