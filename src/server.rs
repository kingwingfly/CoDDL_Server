use coddl::sign;
use dotenv::dotenv;
use sign::login_server::{Login, LoginServer};
use sign::sign_up_server::{SignUp, SignUpServer};
use sign::{LoginReq, LoginResp, SignUpReq, SignUpResp};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};

struct MyLoginServer {
    pgpool: Arc<Pool<Postgres>>,
}

#[tonic::async_trait]
impl Login for MyLoginServer {
    async fn verify(&self, request: Request<LoginReq>) -> Result<Response<LoginResp>, Status> {
        let login_info = request.into_inner();
        let reply = LoginResp {
            result: login_info.verify(self.pgpool.clone()).await,
        };
        Ok(Response::new(reply))
    }
}

struct MySignUpServer {
    pgpool: Arc<Pool<Postgres>>,
}

#[tonic::async_trait]
impl SignUp for MySignUpServer {
    async fn register(&self, request: Request<SignUpReq>) -> Result<Response<SignUpResp>, Status> {
        let sign_up_info = request.into_inner();
        let reply = SignUpResp {
            result: sign_up_info.register(self.pgpool.clone()).await,
        };
        Ok(Response::new(reply))
    }
}

async fn server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("can not find DATABASE_URL in environment");
    let pgpool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("can not connect to database");
    let pgpool = Arc::new(pgpool);
    let addr = "0.0.0.0:8848".parse()?;
    let login_server = MyLoginServer {
        pgpool: pgpool.clone(),
    };
    let sign_up_server = MySignUpServer {
        pgpool: pgpool.clone(),
    };
    println!("Server listening on http://{addr}");
    Server::builder()
        .add_service(LoginServer::new(login_server))
        .add_service(SignUpServer::new(sign_up_server))
        .serve(addr)
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    match tokio::join!(tokio::spawn(server())) {
        (Ok(_),) => Ok(()),
        _ => todo!(),
    }
}
