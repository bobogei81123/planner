use std::net::{SocketAddr, TcpListener};

use axum::Server;

use planner_backend::build_app;
use reqwest::RequestBuilder;
use sea_orm::DatabaseConnection;

pub type Result<T> = anyhow::Result<T>;

pub struct TestServer {
    addr: SocketAddr,
    client: reqwest::Client,
}

impl TestServer {
    pub async fn spawn(db_conn: DatabaseConnection) -> Self {
        let router = build_app(db_conn).await;
        let listener =
            TcpListener::bind("127.0.0.1:0").expect("Cannot bind to 127.0.0.1:0 (dynamic port)");
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async {
            Server::from_tcp(listener)
                .expect("Cannot create Axum server from listener")
                .serve(router.into_make_service())
                .await
                .expect("Server failed");
        });

        Self {
            addr,
            client: reqwest::Client::new(),
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn url_of(&self, path: &str) -> String {
        format!("http://{}{}", self.socket_addr(), path)
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn post(&self, path: &str) -> RequestBuilder {
        self.client.post(self.url_of(path))
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.client.get(self.url_of(path))
    }
}

pub struct UserSession {
    server: TestServer,
    login_token: String,
}

impl UserSession {
    pub async fn login_as(server: TestServer, username: &str) -> Result<Self> {
        #[derive(serde::Serialize)]
        struct LoginRequest<'a> {
            username: &'a str,
        }

        #[derive(serde::Deserialize)]
        pub struct LoginResponse {
            pub token: String,
        }

        let response: LoginResponse = server
            .post("/auth/login")
            .json(&LoginRequest { username })
            .send()
            .await?
            .json()
            .await?;

        let login_token = response.token;
        Ok(Self {
            server,
            login_token,
        })
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.server
            .get(path)
            .header("Authorization", format!("Bearer {}", self.login_token))
    }

    pub fn post(&self, path: &str) -> RequestBuilder {
        self.server
            .post(path)
            .header("Authorization", format!("Bearer {}", self.login_token))
    }

    pub fn server(&self) -> &TestServer {
        &self.server
    }

    pub fn client(&self) -> &reqwest::Client {
        self.server.client()
    }
}
