use once_cell::sync::OnceCell;
use pb::sign_server::{Sign, SignServer};
use pb::{LoginReq, LoginResp, SignUpReq, SignUpResp};
use pgpool::PgPool;
use tonic::{transport::Server, Request, Response, Status};

const ADDR: &str = "[::]:8848";

static PGPOOL: OnceCell<PgPool> = OnceCell::new();

struct MySignServer {}

#[tonic::async_trait]
impl Sign for MySignServer {
    async fn verify(&self, request: Request<LoginReq>) -> Result<Response<LoginResp>, Status> {
        let login_info = request.into_inner();
        let reply = LoginResp {
            result: login_info.verify(PGPOOL.get().unwrap()).await,
        };
        Ok(Response::new(reply))
    }

    async fn register(&self, request: Request<SignUpReq>) -> Result<Response<SignUpResp>, Status> {
        let sign_up_info = request.into_inner();
        let reply = SignUpResp {
            result: sign_up_info.register(PGPOOL.get().unwrap()).await,
        };
        Ok(Response::new(reply))
    }
}

async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = ADDR.parse()?;
    let sign_server = MySignServer {};
    println!("Server listening on http://{addr}");
    Server::builder()
        .add_service(SignServer::new(sign_server))
        .serve(addr)
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    PGPOOL.get_or_init(PgPool::new);
    server().await
}
