use coddl::sign;
use sign::login_server::{Login, LoginServer};
use sign::sign_up_server::{SignUp, SignUpServer};
use sign::{LoginReq, LoginResp, SignUpReq, SignUpResp};
use sqlx::postgres::PgPoolOptions;
use tonic::{transport::Server, Request, Response, Status};

struct MyLoginServer {}

#[tonic::async_trait]
impl Login for MyLoginServer {
    async fn verify(&self, request: Request<LoginReq>) -> Result<Response<LoginResp>, Status> {
        let login_req_info = request.into_inner();
        let reply = LoginResp {
            result: login_req_info.verify(),
        };
        Ok(Response::new(reply))
    }
}

struct MySignUpServer {}

#[tonic::async_trait]
impl SignUp for MySignUpServer {
    async fn register(&self, request: Request<SignUpReq>) -> Result<Response<SignUpResp>, Status> {
        let sign_up_info = request.into_inner();
        let reply = SignUpResp {
            result: sign_up_info.register(),
        };
        Ok(Response::new(reply))
    }
}

async fn server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "0.0.0.0:8848".parse()?;
    let login_server = MyLoginServer {};
    let sign_up_server = MySignUpServer {};
    println!("Server listening on http://{addr}");
    Server::builder()
        .add_service(LoginServer::new(login_server))
        .add_service(SignUpServer::new(sign_up_server))
        .serve(addr)
        .await?;
    Ok(())
}

async fn sql_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:Louis@localhost/coddl")
        .await?;
    sqlx::query(
        "
CREATE TABLE userinfo (
    ID INT PRIMARY KEY NOT NULL,
    USERNAME VARCHAR(80) NOT NULL,
    PASSWORD VARCHAR(80) NOT NULL
)
    ",
    )
    .execute(&pool)
    .await?;
    println!("finish create table");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    match tokio::join!(tokio::spawn(server()), tokio::spawn(sql_server())) {
        (Ok(_), Ok(_)) => Ok(()),
        _ => todo!(),
    }
}
