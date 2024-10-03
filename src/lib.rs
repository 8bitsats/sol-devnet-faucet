use worker::*;
mod utils;
#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
  utils::set_panic_hook();
  Router::new().post_async("/airdrop/:pubkey", |_req, ctx| async move {
    if let Some(pubkey) = ctx.param("pubkey") {
      let json = json!({
          "jsonrpc": "2.0",
          "id": 1,
          "method": "requestAirdrop",
          "params": [pubkey, 1000000000]
      });
      let body = wasm_bindgen::JsValue::from_str(&json.to_string());
      let req = Request::new_with_init(
        "https://api.devnet.solana.com",
        RequestInit::default()
          .with_method(Method::Post)
          .with_headers({
            let mut headers = Headers::new();
            headers.set("Content-Type", "application/json; charset=utf-8")?;
            headers
          })
          .with_body(Some(body)),
      )?;
      let res = Fetch::Request(req).send().await?;
      return Ok(res);
    }
    Response::error("Bad Request", 400)
  })
  .run(req, env)
  .await
}