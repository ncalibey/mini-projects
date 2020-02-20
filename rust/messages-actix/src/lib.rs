#[macro_use]
extern crate actix_web;

use actix_web::{
    error::{Error, InternalError, JsonPayloadError},
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result,
};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

// static variables are not inlined, and thus have a fixed address as a shared global
// variable.
static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);
// the r#"..."# is the syntax for a raw string
const LOG_FORMAT: &'static str = r#""%r" %s %b "%{User-Agent}i" %D"#;

struct AppState {
    server_id: usize,
    // The `Cell` type implements interior mutability by moving values in/out of a shared
    // memory location. With `Cell` we copy the `usize`, avoiding the extra lock of a
    // `RefCell`.
    request_count: Cell<usize>,
    // The `Mutex` allows us to safely coordinate access to the `Vec<String>` across
    // multiple threads.
    //
    // Since we want each thread to be an owner of the data, we use an `Arc<T>`, a reference
    // counted pointer. The `T` is allocated on the heap. We use the `Arc` since it is
    // atomic and thus thread safe.
    messages: Arc<Mutex<Vec<String>>>,
}

pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let messages = Arc::new(Mutex::new(vec![]));
        println!("Starting http server: 127.0.0.1:{}", self.port);
        // `move` moves any variables referenced by the closure into the closure, implying
        // that the closure's lifetime is longer than the surrounding environment's.
        HttpServer::new(move || {
            App::new()
                .data(AppState {
                    // `fetch_add()` returns the original value, but then increments it by one.
                    // The second argument determines how atomic operations synchronize across the
                    // various threads. `SeqCst` is the strongest ordering and should be used as
                    // a default.
                    server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                    request_count: Cell::new(0),
                    // By cloning the `Arc` value, which create a new pointer to the data.
                    messages: messages.clone(),
                })
                // `wrap` wraps our app in middleware.
                .wrap(middleware::Logger::new(LOG_FORMAT))
                // `service` adds a new service to the app using the handler specified in
                // the argument.
                .service(index)
                .service(
                    // creates a resource with the path "/send"
                    web::resource("/send")
                        // data is used for a) specifying route specific data or
                        // b) configuring route specific extractors. This uses a
                        // JSON extractor with a limit of 4096 bytes.
                        .data(
                            web::JsonConfig::default()
                                .limit(4096)
                                .error_handler(post_error),
                        )
                        // configure the route to be a POST request using the `post` handler.
                        .route(web::post().to(post)),
                )
                .service(clear)
                .service(lookup)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
    }
}

// `derive` allows us to implement traits without having to do any new work, given that
// the type meets the requirements. `Serialize` from serde provides the deriving logic
// for us.
#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    messages: Vec<String>,
}

#[derive(Deserialize)]
struct PostInput {
    message: String,
}

#[derive(Serialize)]
struct PostResponse {
    server_id: usize,
    request_count: usize,
    message: String,
}

#[derive(Serialize)]
struct PostError {
    server_id: usize,
    request_count: usize,
    error: String,
}

#[derive(Serialize)]
struct LookupResponse {
    server_id: usize,
    request_count: usize,
    result: Option<String>,
}

#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    // we use `get()` to retrieve the value within the `Cell`.
    let request_count = state.request_count.get() + 1;
    // `set()` changes the value within the `Cell`.
    state.request_count.set(request_count);
    // `Arc<T>` implements `Deref`, so we can treat the data as `Mutex<Vec<String>>`.
    //
    // We use `lock()` to lock the mutex and obtain the data. This returns a `Result` that
    // wraps a `MutexGuard`. The error variant will only occur when the mutex is poisoned.
    //
    // We use `unwrap()` which pulls the `Ok` variant out of the `Result`, and panics on the
    // `Err` variant. Since `MutexGuard` implements `Deref`, we treat the result as
    // `Vec<String>`.
    let ms = state.messages.lock().unwrap();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: ms.clone(),
    }))
}

fn post(msg: web::Json<PostInput>, state: web::Data<AppState>) -> Result<web::Json<PostResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let mut ms = state.messages.lock().unwrap();
    // We `clone()` the `message` because we only have a borrowed reference -- it belongs
    // to `msg`.
    ms.push(msg.message.clone());

    Ok(web::Json(PostResponse {
        server_id: state.server_id,
        request_count,
        // Same logic as above needed here.
        message: msg.message.clone(),
    }))
}

#[post("/clear")]
fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let mut ms = state.messages.lock().unwrap();
    ms.clear();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: vec![],
    }))
}

fn post_error(err: JsonPayloadError, req: &HttpRequest) -> Error {
    // This fetches the Actix extensions.
    let extns = req.extensions();
    // We need to use a turbofish here so the compiler knows what type we want to
    // get back from the extensions. If the type wasn't previously stored, then it will
    // return the `None` variant.
    let state = extns.get::<web::Data<AppState>>().unwrap();
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let post_error = PostError {
        server_id: state.server_id,
        request_count,
        error: format!("{}", err),
    };
    // Since `actix_web::Error` implements `From<T>` for any `T` that implements the `ResponseError`
    // trait, we use `into()` to convert our `InternalError` into an `Error`.
    InternalError::from_response(err, HttpResponse::BadRequest().json(post_error)).into()
}

#[get("/lookup/{index}")]
// `web::Path` deserializes the path segment to the type specified. For more than 1 segment,
// we can use a tuple. We can also define our own type that implements `Deserialize` to handle
// more complex cases.
fn lookup(state: web::Data<AppState>, idx: web::Path<usize>) -> Result<web::Json<LookupResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();
    // `into_inner()` converts the `Path<usize>` into the `usize` it carries.
    //
    // `cloned()` converts our `Option<&T>` into `Option<T>`. More specifically, it clones
    // the data within the `Some` variant and does nothing for the `None` variant.
    let result = ms.get(idx.into_inner()).cloned();
    Ok(web::Json(LookupResponse {
        server_id: state.server_id,
        request_count,
        result,
    }))
}
