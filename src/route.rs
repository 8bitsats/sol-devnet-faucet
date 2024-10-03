static HTML: &str = r#"..."#
Router::new()
  .post_async("/airdrop/:pubkey", |_req, ctx| async move { ... })
  .get("/", |_, _| {
    Response::from_html(HTML)
  })
  .run(req, env)
  .await  