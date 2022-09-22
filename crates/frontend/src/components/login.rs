use crate::util::post;
use serde::{Deserialize, Serialize};
use serde_json::json;
use types::LoginStatus;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    Login,
    SetLoginState(Result<gloo_net::http::Response, gloo_net::Error>),
    SetResponse(Result<LoginStatus, gloo_net::Error>),
}

#[derive(Default)]
enum Status {
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
    status: Status,
    refs: Vec<NodeRef>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct User {
    name: String,
    ip: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub on_data: Callback<LoginStatus>,
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            refs: vec![NodeRef::default(), NodeRef::default()],
            ..Default::default()
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Login => {
                let user_id = self.refs[0]
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .parse::<u32>();
                let password = self.refs[1].cast::<HtmlInputElement>().unwrap().value();

                if let Ok(user_id) = user_id {
                    ctx.link().send_future(async move {
                        let js = json!({
                            "user_id": user_id,
                            "password": password,
                        });

                        Msg::SetLoginState(post("http://127.0.0.1:3001/login", &js).await)
                    });

                    false
                } else {
                    self.status = Status::InvalidUserId;
                    true
                }
            }
            Msg::SetLoginState(response) => match response {
                Ok(response) => {
                    ctx.link().send_future(async move {
                        Msg::SetResponse(response.json::<LoginStatus>().await)
                    });

                    false
                }
                Err(_) => {
                    self.status = Status::RequestError;
                    true
                }
            },
            Msg::SetResponse(data) => match data {
                Ok(data) => {
                    web_sys::console::log_1(&"ok data".into());
                    ctx.props().on_data.emit(data);
                    self.status = Status::Accepted;

                    true
                }
                Err(_) => {
                    self.status = Status::WrongPwd;
                    true
                }
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let status = match self.status {
            Status::Inputting => ("none", ""),
            Status::InvalidUserId => ("block", "请输入正确的用户ID!"),
            Status::WrongPwd => ("block", "用户ID错误或密码错误!"),
            // Status::DataError => ("block", "服务器返回数据异常!"),
            Status::RequestError => ("block", "请求失败!"),
            Status::Accepted => ("none", ""),
        };

        html! {
            <>
                <div>
                    <div>
                        <label>{"用户ID:"}</label>
                        <input
                            type = {"text"}
                            ref = {&self.refs[0]}
                        />
                    </div>
                    <div>
                        <label>{"密码:"}</label>
                        <input
                            type = {"password"}
                            ref = {&self.refs[1]}
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
                    <span style = {format!("display:{};color: \"red\"", status.0)}>{ status.1 }</span>
                </div>
            </>
        }
    }
}
