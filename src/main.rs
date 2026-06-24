use regex::Regex;
use salvo::core::prelude::*;
use salvo::routing::filters::PathFilter;

#[handler]
async fn default_handler(_req: &mut Request, res: &mut Response) {
    res.render(Text::Plain("default_handler"));
}

#[tokio::main]
async fn main() {
    PathFilter::register_wisp_regex("foo", Regex::new(r"([a-f0-9]{7})").unwrap());

    let acceptor = TcpListener::new("127.0.0.1:8000".to_string()).bind().await;
    let router = Router::new()
        .push(Router::with_path("/~{foo:foo}/{**subpath}").get(default_handler))
        .push(Router::with_path("/{**subpath}").get(default_handler));
    let service = Service::new(router);

    Server::new(acceptor).serve(service).await;
}
