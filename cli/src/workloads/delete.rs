use crate::display::{self, section};
use crate::http::{auth, base_url, delete_req};

pub async fn run(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/workloads/{}", base_url(&server), id);

    let pb = display::spinner("deleting workload...");
    let result = delete_req(&client, &url, &token).await;
    pb.finish_and_clear();

    result?;

    section("Workloads  /  Delete");
    display::success(&format!("workload {} deleted", id));
    println!();

    Ok(())
}
