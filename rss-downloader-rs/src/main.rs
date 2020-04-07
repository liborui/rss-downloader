#[macro_use]
extern crate log;
extern crate simple_logger;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};
use std::str; // for byte -> str
use std::env; // for input args
// use cmd_lib::{run_cmd, run_fun, CmdResult, FunResult};// for exec you-get
use cmd_lib::{run_cmd};// for exec you-get



#[derive(Serialize, Deserialize)]
struct ParamsFromHttp{
    url: String,
    invoker: String,
    video_title: String,
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
 
    // 通过req.method()和req.uri().path()来识别方法和请求路径
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => {
            // 这里需要完整的body，所以需要等待全部的stream并把它们变为bytes
            let full_body = hyper::body::to_bytes(req.into_body()).await?;
            let parse_param: ParamsFromHttp = serde_json::from_str(&str::from_utf8(&full_body).unwrap()).unwrap();// may needs rebuild, fail to react to error json
            info!("{}", parse_param.url);

            let args: Vec<String> = env::args().collect();
            let abs_save_addr = &args[1];
            info!("{}", abs_save_addr);
            
            match run_cmd!("mkdir -p {}/BilibiliDownloads/{}", abs_save_addr, parse_param.video_title){
                Ok(_val)=>info!("Folder created!"),
                Err(err)=>warn!("Folder create failed! {}", err),           
            };

            match run_cmd!("you-get -o {1}/BilibiliDownloads/{2} --playlist {0}", 
                parse_param.url, abs_save_addr, parse_param.video_title){
                Ok(_val)=>info!("Download OK!"),
                Err(err)=>warn!("Download failed! {}", err),
            };//!!Needs further optimization!! >> {1}/BilibiliDownloads/you-get.log

            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };
 
    Ok(response)
}
 
#[tokio::main]
async fn main() {
    // Check if CLI args correct
    if env::args().len() != 2 {
        warn!("*** args illegal!");
    }
    //Initialize logger
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(echo))
    });
 
    let server = Server::bind(&addr).serve(make_svc);
 
    if let Err(e) = server.await {
        warn!("server error: {}", e);
    }
}