use permafacts::PermafactsServer;

#[tokio::main]
async fn main() {
    let server = PermafactsServer::new();

    // Spawn the server task and keep the JoinHandle so that the server keeps running
    let server_task = tokio::spawn(async move {
        server.run().await;
    });

    // Wait for the server task to complete (which will never happen, since the server runs indefinitely)
    if let Err(err) = server_task.await {
        eprintln!("Server task failed: {:?}", err);
    }

}
