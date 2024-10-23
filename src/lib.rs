use worker::*;

// Handler that redirects based on the path by looking up
// the path in the KV store
async fn redirect_handler(_: Request, ctx: RouteContext<()>) -> Result<Response> {
    let slug = ctx.param("slug").ok_or( Error::from("No slug found"))?;
    let kv = ctx.kv("workers_link_shortener")?;
    let redirect = kv.get(slug).text().await?;
    match redirect {
        Some(destination) => Response::redirect(Url::parse(&destination)?),
        None => Response::error("Not Found", 404),
    }
}

async fn index_handler(_: Request, _: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello, World! Welcome to my link shortener.")
}

/// This function handles all requests
#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/", index_handler)
        .get_async("/:slug", redirect_handler) 
        .run(req, env)
        .await
}