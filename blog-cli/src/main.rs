use blog_client::pod;
use clap::{Parser, Subcommand};

use std::env;

use blog_client::http_client::HttpClient;
use blog_client::grpc_client::GrpcClient;
use blog_client::error::ClientError;

const DEFAULT_HTTP_ADDR: &str = "http://127.0.0.1:3000";
const DEFAULT_GRPC_ADDR: &str = "http://127.0.0.1:50051";

#[derive(Subcommand, Debug)]
enum Commands {
    Register{
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        pass: String,
    },
    Login{
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        pass: String,
    },
    Create{
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: String,
    },
    Update{
        #[arg(short, long)]
        id: i64,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        content: Option<String>,
    },
    Delete{
        #[arg(short, long)]
        id: i64,
    },
    Get{
        #[arg(short, long)]
        id: i64,
    },
    List{
        #[arg(short, long)]
        offset: i64,
        #[arg(short, long)]
        limit: i64
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    grpc: bool,
}

fn print_message<T: std::fmt::Debug> (res: &Result<T, ClientError>, message: &str) {
    match res {
        Ok(val) => {
            println!("{}: {:?}", message, val);
        }
        Err(e) => {
            println!("{e}");
        }
    }
}

fn save_token(token: &str) -> Result<(), std::io::Error> {
    std::fs::write("token.txt", token)
}

fn read_token() -> Result<String, std::io::Error> {
    std::fs::read_to_string("token.txt")
}

#[tokio::main]
async fn main() {
    let http_server_addr = env::var("HTTP_SERVER_ADDR").unwrap_or(DEFAULT_HTTP_ADDR.to_string());
    let grpc_server_addr = env::var("GRPC_SERVER_ADDR").unwrap_or(DEFAULT_GRPC_ADDR.to_string());

    let http_client = HttpClient::new(&http_server_addr);
    let mut grpc_client = match GrpcClient::connect(&grpc_server_addr).await {
        Ok(res) => res,
        Err(e) => {
            println!("Can't connect to grpc server: {e}");
            return;
        }
    };
    
    let cli = Cli::parse();
    match cli.command {
        Commands::Register { username, email, pass } => {
            let res =
            if cli.grpc {
                grpc_client.register(username, email, pass).await
            }else{
                let reg_req = pod::RegisterUserReq{
                    username,
                    email,
                    password: pass,
                };
                http_client.register(reg_req).await
            };
            print_message(&res, "Regestration complete: token");
            let reg_user = res.unwrap_or_default();
            save_token(&reg_user.token).expect("Can't save token");
        }
        Commands::Login { username, pass } => {
            let res =
            if cli.grpc {
                grpc_client.login(username, pass).await
            }else{
                let log_req = pod::LoginUserReq{
                    username,
                    password: pass,
                };
                http_client.login(log_req).await
            };
            print_message(&res, "Login complete: token");
            let reg_user = res.unwrap_or_default();
            save_token(&reg_user.token).expect("Can't save token");
        }
        Commands::Create { title, content } => {
            let token = read_token().expect("Can't read token");
            let res =
            if cli.grpc {
                grpc_client.create_post(&token, title, content).await
            }else{
                let create_req = pod::NewPost{
                    title,
                    content,
                };
                http_client.create_post(&token, create_req).await
            };
            print_message(&res, "Create post: ");
        }
        Commands::Update { id, title, content } => {
            let token = read_token().expect("Can't read token");
            let res =
            if cli.grpc {
                grpc_client.update_post(&token, id, title, content).await
            }else{
                let update_req = pod::UpdatePost{
                    title,
                    content,
                };
                http_client.update_post(&token, pod::PostId{id}, update_req).await
            };
            print_message(&res, "Update post: ");
        }
        Commands::Delete { id} => {
            let token = read_token().expect("Can't read token");
            let res =
            if cli.grpc {
                grpc_client.delete_post(&token, id).await
            }else{
                http_client.delete_post(&token, pod::PostId{id}).await
            };
            print_message(&res, "Delete post: ");
        }
        Commands::Get { id} => {
            let res =
            if cli.grpc {
                grpc_client.get_post(id).await
            }else{
                http_client.get_post(pod::PostId{id}).await
            };
            print_message(&res, "Get post: ");
        }
        Commands::List { offset, limit} => {
            let res =
            if cli.grpc {
                grpc_client.get_posts(offset, limit).await
            }else{
                http_client.get_posts(offset, limit).await
            };
            print_message(&res, "List posts: ");
        }
    }
}
