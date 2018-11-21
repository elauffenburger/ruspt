use std::panic;

use actix_web::{http, middleware, server, App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse};
use futures::Future;

use rusptlib;

#[derive(Debug, Deserialize)]
struct SubmitCodeRequest {
    pub code: String,
}

#[derive(Debug, Serialize)]
struct SubmitCodeResponse {
    pub output: String,
    pub success: bool,
}

fn run_code(code: String) -> SubmitCodeResponse {
    let exec_result = panic::catch_unwind(|| {
        let mut env = rusptlib::Environment::new();
        let program = rusptlib::parse(code);

        let result = rusptlib::exec_prog(&mut env, program);

        rusptlib::print_cell(result)
    });

    match exec_result {
        Ok(res) => SubmitCodeResponse {
            output: res,
            success: true,
        },
        Err(msg) => {
            let msg = {
                if let Some(msg) = msg.downcast_ref::<&'static str>() {
                    format!("{:?}", msg)
                } else if let Some(msg) = msg.downcast_ref::<String>() {
                    format!("{:?}", msg)
                } else {
                    format!("{:?}", msg)
                }
            };

            SubmitCodeResponse {
                output: msg,
                success: false,
            }
        }
    }
}

fn submit_code_handler(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
        .from_err()
        .and_then(|submit_code_req: SubmitCodeRequest| {
            let response = run_code(submit_code_req.code);

            Ok(HttpResponse::Ok().json(response))
        }).responder()
}

pub fn server(addr: String) {
    println!("Starting server...");

    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let actix_sys = actix::System::new("ruspt-server");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .middleware(middleware::cors::Cors::default())
            .resource("/submit-code", |r| r.method(http::Method::POST).f(submit_code_handler))
    }).bind(&addr)
    .unwrap()
    .shutdown_timeout(1)
    .start();

    println!("Server started on {}", &addr);
    let _ = actix_sys.run();
}
