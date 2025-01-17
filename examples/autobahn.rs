mod answer;
mod utils;

use crate::utils::connect;
use answer::echo;
use std::{io::Result, str};
use web_socket::*;

const ADDR: &str = "localhost:9001";
const AGENT: &str = "agent=web-socket";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let total = get_case_count().await.expect("unable to get case count");
    for case in 1..=total {
        let _ = echo(connect(ADDR, &format!("/runCase?case={case}&{AGENT}")).await?).await;
    }
    update_reports().await
}

async fn get_case_count() -> Option<u32> {
    let Ok(Event::Data { data, .. }) = connect(ADDR, "/getCaseCount").await.ok()?.recv().await else {
        return None
    };
    std::str::from_utf8(&data).ok()?.parse().ok()
}

async fn update_reports() -> Result<()> {
    let ws = connect(ADDR, &format!("/updateReports?{AGENT}")).await?;
    ws.close(()).await
}
