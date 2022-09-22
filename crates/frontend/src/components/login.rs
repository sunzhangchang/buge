use crate::util::post;
use serde::{Deserialize, Serialize};
use serde_json::json;
use types::LoginState;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    Login,
    SetState(State, Option<LoginState>),
}

#[derive(Default)]
enum State {
    #[default]
    Inputting,
    InvalidUserId,
    WrongPwd,
    Accepted,
    RequestError,
    // DataError,
}

#[derive(Default)]
pub struct Login {
    state: State,
    username: NodeRef,
    pwd: NodeRef,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct User {
    name: String,
    ip: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub on_data: Callback<LoginState>,
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ..Default::default()
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Login => {
                let user_id = self
                    .username
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .parse::<u32>();
                let password = self.pwd.cast::<HtmlInputElement>().unwrap().value();

                if let Ok(user_id) = user_id {
                    ctx.link().send_future(async move {
                        let js = json!({
                            "user_id": user_id,
                            "password": password,
                        });

                        // todo: add websocket
                        let state = match post("http://127.0.0.1:3001/login", &js).await {
                            Ok(response) => match response.json::<LoginState>().await {
                                Ok(data) => (State::Accepted, Some(data)),
                                Err(_) => (State::WrongPwd, None),
                            },
                            Err(_) => (State::RequestError, None),
                        };

                        Msg::SetState(state.0, state.1)
                    });

                    false
                } else {
                    self.state = State::InvalidUserId;
                    true
                }
            }
            Msg::SetState(state, data) => {
                self.state = state;
                if matches!(self.state, State::Accepted) {
                    ctx.props().on_data.emit(data.unwrap());
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = match self.state {
            State::Inputting => ("none", ""),
            State::InvalidUserId => ("block", "请输入正确的用户ID!"),
            State::WrongPwd => ("block", "用户ID错误或密码错误!"),
            // Status::DataError => ("block", "服务器返回数据异常!"),
            State::RequestError => ("block", "请求失败!"),
            State::Accepted => ("none", ""),
        };

        html! {
            <div>
                <div>
                    <label>{"用户ID:"}</label>
                    <input
                        type = {"text"}
                        ref = {&self.username}
                    />
                </div>
                <div>
                    <label>{"密码:"}</label>
                    <input
                        type = {"password"}
                        ref = {&self.pwd}
                    />
                </div>
                <button
                    type = {"button"}
                    onclick = {
                        ctx.link().callback(move |_| Msg::Login)
                    }
                >
                    {"登录"}
                </button>
                <span style = {format!("display:{};color: \"red\"", state.0)}>{ state.1 }</span>
            </div>
        }
    }
}
