use std::future::Future;
use tokio::fs;
use anyhow::Result;

pub async fn walk_dir<F, FUT>(root_path: String, f: F) -> Result<()>
    where F: Fn(String) -> FUT, FUT: Future<Output=()>
{
    let mut path_container: Vec<String> = vec![root_path];
    while let Some(curr) = path_container.pop() {
        if fs::metadata(curr.clone()).await?.is_file() {
            f(curr.clone()).await;
            continue;
        }

        let mut curr_dir = fs::read_dir(curr).await?;
        while let Some(entry) = curr_dir.next_entry().await? {
            let path = entry.path();
            let path_of_string = path.into_os_string().into_string().unwrap();
            path_container.push(path_of_string)
        }
    }

    Ok(())
}