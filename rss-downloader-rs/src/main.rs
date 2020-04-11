#[macro_use]
extern crate log;
extern crate simple_logger;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};
use std::str; // for byte -> str
use std::env; // for input args
use std::fs;
use std::process::Command;
use std::io::{self, Write};


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
            info!("Recieved a download request!");
            let full_body = hyper::body::to_bytes(req.into_body()).await?;
            let parse_param: ParamsFromHttp = serde_json::from_str(&str::from_utf8(&full_body).unwrap()).unwrap();// may needs rebuild, fail to react to error json
            info!("Dest. URL: {}", parse_param.url);
            info!("Video Title: {}", parse_param.video_title);
            info!("Invoker is: {}", parse_param.invoker);

            let args: Vec<String> = env::args().collect();
            let abs_save_addr = &args[1];
            info!("Video will be saved to: {}", abs_save_addr);
            
            let video_title_without_illegal_char = parse_param.video_title
                .replace("\\", "").replace("/","").replace(":", "").replace("*", "").replace("?", "").replace("\"","").replace("<","").replace(">","").replace("|","");

            info!("Video title without illegal char: {}", video_title_without_illegal_char);
            
            let video_save_folder = format!("{}/BilibiliDownloads/{}", abs_save_addr, video_title_without_illegal_char);
            // info!("{}", video_save_folder);
            fs::create_dir_all(video_save_folder).unwrap_or_else(|why| {
                warn!("! {:?}", why.kind());
            });

            info!("Start download!");
            let video_save_folder = format!("{}/BilibiliDownloads/{}", abs_save_addr, video_title_without_illegal_char);
            //disabled log saving, which will cause unexpected exit of you-get
            // let log_save_addr = format!("{}/BilibiliDownloads/you-get.log", abs_save_addr); // .args(&[">>", &log_save_addr])
            let you_get_output = Command::new("you-get")
                .args(&["--playlist", &parse_param.url])
                .args(&["-o", &video_save_folder])
                .output()
                .expect("failed to execute process");
            info!("You-get output is: ");
            io::stdout().write_all(&you_get_output.stdout).unwrap();
            info!("You-get error info: ");
            io::stderr().write_all(&you_get_output.stderr).unwrap();
            info!("You-get status: {}", you_get_output.status);

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