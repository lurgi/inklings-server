use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // lib.rs에 있는 run 함수를 호출하여 서버를 실행합니다.
    inklings_server::run().await
}
