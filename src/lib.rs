#[macro_use]
extern crate worker;
use serde_json;
use worker::{Date, Env, Request, Response, Result as WorkerResult, RouteContext, Router};

mod convert;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

struct DateInput {
    year: u16,
    month: u8,
    day: u8,
}

fn parse_input<D>(ctx: RouteContext<D>) -> Result<DateInput, std::num::ParseIntError> {
    let year = ctx.param("year").unwrap().trim().parse::<u16>()?;
    let month = ctx.param("month").unwrap().trim().parse::<u8>()?;
    let day = ctx.param("day").unwrap().trim().parse::<u8>()?;

    Ok(DateInput {
        year: year,
        month: month,
        day: day,
    })
}

async fn solar_to_lunar_handler<D>(
    mut _req: Request,
    ctx: RouteContext<D>,
) -> WorkerResult<Response> {
    if let Ok(DateInput { year, month, day }) = parse_input(ctx) {
        if let Some(ld) = convert::solar_to_lunar(year, month, day) {
            Response::ok(serde_json::to_string(&ld)?)
        } else {
            Response::error(format!("Couldn't convert {year}/{month}/{day}"), 400)
        }
    } else {
        Response::error(format!("Couldn't parse input"), 400)
    }
}

async fn lunar_to_solar_handler<D>(_req: Request, ctx: RouteContext<D>) -> WorkerResult<Response> {
    if let Ok(DateInput { year, month, day }) = parse_input(ctx) {
        let out = convert::lunar_to_solar(year, month, day);
        Response::ok(serde_json::to_string(&out)?)
    } else {
        Response::error(format!("Couldn't parse input"), 400)
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> WorkerResult<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get_async("/solar-to-lunar/:year/:month/:day", solar_to_lunar_handler)
        .get_async("/lunar-to-solar/:year/:month/:day", lunar_to_solar_handler)
        .run(req, env)
        .await
}
