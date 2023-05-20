use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::process::Command;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

mod tempfile;

#[derive(Serialize)]
struct HelloResponse {
    status: String, // "ok"
    time: u64,      // Current timestamp
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
async fn handler(req_body: String) -> HttpResponse {
    let args = env::args().collect::<Vec<String>>();
    let shell_command: Option<&String> = args.get(1);

    match shell_command {
        Some(command) => {
            let temp_file_path = tempfile::write_data(req_body);

            if let Ok(temp_file_path) = temp_file_path {
                let command = format!("cat {} | {}", &temp_file_path, &command);
                let output = Command::new("sh").arg("-c").arg(command).output();

                match output {
                    Ok(output) => {
                        let stdout = String::from_utf8(output.stdout).unwrap();
                        let stderr = String::from_utf8(output.stderr).unwrap();

                        let obj = RunnerResponse {
                            success: true,
                            error: None,
                            stderr: Some(stderr),
                            stdout: Some(stdout),
                        };

                        if !obj
                            .stderr
                            .as_ref()
                            .map(String::as_str)
                            .unwrap_or("")
                            .is_empty()
                        {
                            return HttpResponse::BadRequest().json(obj);
                        }

                        return HttpResponse::Ok().json(obj);
                    }
                    Err(error) => {
                        let obj = RunnerResponse {
                            success: false,
                            error: Some(error.to_string()),
                            stderr: None,
                            stdout: None,
                        };

                        return HttpResponse::InternalServerError().json(obj);
                    }
                }
            }

            let obj = RunnerResponse {
                success: false,
                error: Some("Internal Error: Failed to write tmp file".to_string()),
                stderr: None,
                stdout: None,
            };

            return HttpResponse::InternalServerError().json(obj);
        }
        _ => {
            // return runner response with error
            let obj = RunnerResponse {
                success: false,
                error: Some("Shell command is missing".to_string()),
                stderr: None,
                stdout: None,
            };

            return HttpResponse::BadRequest().json(obj);
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(handler))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
