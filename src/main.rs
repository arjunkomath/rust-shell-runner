use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::io::Write;
use std::process::{Command, Stdio};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

struct AppState {
    shell_command: String,
}

#[derive(Serialize)]
struct HelloResponse {
    status: String,
    time: u64,
}

#[derive(Serialize)]
struct RunnerResponse {
    success: bool,
    error: Option<String>,
    stderr: Option<String>,
    stdout: Option<String>,
}

#[get("/")]
async fn index() -> impl Responder {
    let obj = HelloResponse {
        status: "ok".to_string(),
        time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get the current time")
            .as_secs(),
    };

    web::Json(obj)
}

#[post("/")]
async fn handler(req_body: String, data: web::Data<AppState>) -> HttpResponse {
    let mut child = match Command::new("sh")
        .arg("-c")
        .arg(&data.shell_command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            return HttpResponse::InternalServerError().json(RunnerResponse {
                success: false,
                error: Some(error.to_string()),
                stderr: None,
                stdout: None,
            });
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(req_body.as_bytes());
    }

    match child.wait_with_output() {
        Ok(output) => {
            let success = output.status.success();
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

            let obj = RunnerResponse {
                success,
                error: None,
                stderr: Some(stderr),
                stdout: Some(stdout),
            };

            if success {
                HttpResponse::Ok().json(obj)
            } else {
                HttpResponse::BadRequest().json(obj)
            }
        }
        Err(error) => HttpResponse::InternalServerError().json(RunnerResponse {
            success: false,
            error: Some(error.to_string()),
            stderr: None,
            stdout: None,
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Invalid port");

    let shell_command = env::args()
        .nth(1)
        .expect("Shell command argument is required");

    println!("Listening on port {}", port);

    let payload_limit = env::var("MAX_PAYLOAD_SIZE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1024 * 1024); // 1MB default

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                shell_command: shell_command.clone(),
            }))
            .app_data(web::PayloadConfig::new(payload_limit))
            .service(index)
            .service(handler)
    })
    .bind(("::", port))?
    .run()
    .await
}
