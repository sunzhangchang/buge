use serde::Serialize;

pub async fn post<J>(url: &str, json: &J) -> Result<gloo_net::http::Response, gloo_net::Error>
where
    J: Serialize + ?Sized,
{
    gloo_net::http::Request::post(url)
        .json(json)
        .unwrap()
        .send()
        .await
}
