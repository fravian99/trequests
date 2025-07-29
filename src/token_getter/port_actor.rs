use std::{collections::HashMap, convert::Infallible};

use http_body_util::Full;
use hyper::{body::Bytes, header::HeaderValue, server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use tokio::{net::TcpListener, sync::mpsc, task::JoinHandle};

use crate::util;

pub(crate) struct ListenerHandle<M> {
    receiver: mpsc::Receiver<M>,
    task: JoinHandle<()>,
}

impl ListenerHandle<Option<HashMap<String, Vec<String>>>> {
    pub fn new(listener: TcpListener) -> Self {
        let (sender, receiver) = mpsc::channel(10);
        let listener_actor = ListenerActor::new(listener, sender);
        let task = tokio::task::spawn(listener_actor.run());
        Self { receiver, task }
    }

    pub async fn receive(&mut self) -> Option<HashMap<String, Vec<String>>> {
        self.receiver.recv().await.unwrap_or_default()
    }

    pub fn kill_actor(self) {
        self.task.abort();
    }
}

struct ListenerActor<M> {
    listener: TcpListener,
    sender: mpsc::Sender<Option<M>>,
}

impl ListenerActor<HashMap<String, Vec<String>>> {
    pub fn new(
        listener: TcpListener,
        sender: mpsc::Sender<Option<HashMap<String, Vec<String>>>>,
    ) -> Self {
        Self { listener, sender }
    }

    pub async fn run(self) {
        loop {
            let (tcp_stream, _) = self.listener.accept().await.unwrap();
            let io = TokioIo::new(tcp_stream);
            let conn = http1::Builder::new()
                .keep_alive(true)
                .serve_connection(
                    io,
                    service_fn(|request| async { self.handle(request).await }),
                )
                .await;
            if let Err(http_err) = conn {
                eprintln!("Error while serving HTTP connection: {}", http_err);
            }
        }
    }

    async fn handle(
        &self,
        request: hyper::Request<hyper::body::Incoming>,
    ) -> Result<hyper::Response<Full<Bytes>>, Infallible> {
        let params = util::get_params(&request);
        let response: &str = if params.contains_key("access_token") {
            let _ = self.sender.send(Some(params)).await;
            r#"<!doctype html>
            <html>
                <head>
                    <style>
                        * {
                            color: white;
                            background: black;
                        }
                    </style>
                </head>
                <body>
                    <h1>Access token received</h1>
                    Close this window
                </body>
            </html>"#
        } else if request.uri().query().is_none() {
            r#"<!doctype html>
            <html>
                <body>
                    <script>
                        let urlString = window.location.toString();
                        urlString = urlString.replace('#','?');
                        window.location.replace(urlString);
                    </script>
                </body>
            </html>"#
        } else {
            let _ = self.sender.send(None).await;
            r#"<!doctype html>
            <html>
                <head>
                    <style>
                        * {
                            color: white;
                            background: black;
                        }
                    </style>
                </head>
                <body>
                    Error: Access token not found
                </body>
            </html>"#
        };
        let mut response = hyper::Response::new(Full::new(Bytes::from(response)));
        response.headers_mut().append(
            "content-type",
            HeaderValue::from_static("text/html, charset=utf8"),
        );
        Ok(response)
    }
}
