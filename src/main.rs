extern crate rlua;
extern crate actix;
extern crate actix_lua;
extern crate actix_web;
extern crate futures;

use rlua::prelude::*;
use actix::prelude::*;
use actix_lua::{LuaActor, LuaActorBuilder, LuaMessage};
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path, State,
};
use futures::Future;

struct AppState { pub lua: Addr<LuaActor> }

fn index((name, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    println!("index");
    state
        .lua
        .send(LuaMessage::from(name.into_inner()))
        .from_err()
        .and_then(|res| match res {
            LuaMessage::String(s) => Ok(HttpResponse::Ok().json(s)),

            // ignore everything else
            _ => Ok(HttpResponse::Ok().body("Unsupported return type")),
        })
        .responder()
}

fn main() {
    let sys = actix::System::new("torchbear");

    let lua = Lua::new();
    lua.globals().set("myfn", lua.create_function(
        |_, s: String| Ok(s)
    ).unwrap()).unwrap();

    lua.exec::<_, ()>(include_str!("init.lua"), None).unwrap();

    let addr = Arbiter::start(move |_| {
        LuaActorBuilder::new()
            .on_handle("handler.lua")
            .build_with_vm(lua)
            .unwrap()
    });

    server::new(move || {
        App::with_state(AppState { lua: addr.clone() })
            .resource("/{name}", |r| r.method(http::Method::GET).with(index))
    }).bind("127.0.0.1:8080").unwrap().start();

    println!("Started server on 127.0.0.1:8080");

    let _ = sys.run();
}
