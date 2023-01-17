// 本模板由天机Ceyase编写，基于MIT许可证开源
use tonic::{transport::Server, Request, Response, Status};
use std::net::TcpListener;

use hiper_link::plugin_server::{Plugin, PluginServer};
use hiper_link::{EventsRequest, EventsResponse, InteractionRequest, InteractionResponse};

pub mod hiper_link {
    include!("proto.rs");
}

#[derive(Debug, Default)]
pub struct API {}

#[tonic::async_trait]
impl Plugin for API {

    // 加载插件
    async fn on_load(
        &self,
        _request: Request<EventsRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        let reply = hiper_link::EventsResponse {value : "1".into()};

        Ok(Response::new(reply))
    }

    // 停用插件
    async fn on_unload(
        &self,
        _request: Request<EventsRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        let reply = hiper_link::EventsResponse {value : "1".into()};

        Ok(Response::new(reply))
    }

    // 安装插件
    async fn on_install(
        &self,
        _request: Request<EventsRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        let reply = hiper_link::EventsResponse {value : "1".into()};

        Ok(Response::new(reply))
    }

    // 卸载插件
    async fn on_uninstall(
        &self,
        _request: Request<EventsRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        let reply = hiper_link::EventsResponse {value : "1".into()};

        Ok(Response::new(reply))
    }

    // HL 启动
    async fn on_start(
        &self,
        _request: Request<EventsRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        let reply = hiper_link::EventsResponse {value : "1".into()};

        Ok(Response::new(reply))
    }

    // HL 停止
    async fn on_stop(
        &self,
        _request: Request<EventsRequest>,
    ) -> Result<Response<EventsResponse>, Status> {
        let reply = hiper_link::EventsResponse {value : "1".into()};

        Ok(Response::new(reply))
    }

    // 前后端交互
    async fn interaction(
        &self,
        _request: Request<InteractionRequest>,
    ) -> Result<Response<InteractionResponse>, Status> {

        let reply = hiper_link::InteractionResponse {value : "1".into()};
        Ok(Response::new(reply))
    }
}

fn listen_available_port() -> Option<TcpListener> {
    for port in 32768..61000 {
         match TcpListener::bind(("127.0.0.1", port)) {
             Ok(l) => return Some(l),
             _ => {}
         }
    }
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr_str = listen_available_port().unwrap().local_addr().unwrap().to_string();

    let addr = addr_str.parse()?;
    let plugin = API::default();

    println!("1|1|tcp|{addr_str}|grpc");
    Server::builder()
        .add_service(PluginServer::new(plugin))
        .serve(addr)
        .await?;
    Ok(())
}