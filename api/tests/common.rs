use tokio::sync::oneshot;
use tokio::task::JoinHandle;

/// Test server helper that starts the API server on a random port
pub struct TestServer {
    pub base_url: String,
    _shutdown_tx: Option<oneshot::Sender<()>>,
    _handle: JoinHandle<()>,
}

impl TestServer {
    /// Start the test server on a random available port
    pub async fn start() -> Result<Self, Box<dyn std::error::Error>> {
        // Create the listener first to bind to an available port
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let port = addr.port();
        let base_url = format!("http://127.0.0.1:{}", port);

        // Create a config with the test port
        let mut config = api::config::settings::Config::new()?;
        config.server_config.host = "127.0.0.1".to_string();
        config.server_config.port = port;

        // Create shutdown channel
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        // Start the server in a background task, moving the listener into it
        let handle = tokio::spawn(async move {
            // Initialize logging for tests (but don't spam output)
            let _ = std::env::var("RUST_LOG").is_ok();

            let router = match api::app::router::get_app_router(&config).await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to create router: {}", e);
                    return;
                }
            };

            // Create a server with shutdown signal using the already-bound listener
            let server = axum::serve(listener, router).with_graceful_shutdown(async {
                shutdown_rx.await.ok();
            });

            if let Err(e) = server.await {
                eprintln!("Server error: {}", e);
            }
        });

        // Give the server a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(TestServer {
            base_url,
            _shutdown_tx: Some(shutdown_tx),
            _handle: handle,
        })
    }

    /// Create an HTTP client for making requests
    pub fn client(&self) -> reqwest::Client {
        reqwest::Client::new()
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        // Send shutdown signal if we still have the sender
        if let Some(tx) = self._shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}
