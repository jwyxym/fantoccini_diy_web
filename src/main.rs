use fantoccini::{
    Client, Locator,
    elements::{
        Element
    }
};
use anyhow::Result;
use reqwest::Client as ReqwestClient;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::thread;
use std::path::{Path, PathBuf};
async fn print(client: Client) -> Result<Client> {
    let text = client.find(Locator::Css("body")).await?.text().await?;
    println!("{}", text);
    Ok(client)
}
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let client: Client = Client::new("http://localhost:9515").await?;
    
    client.goto("http://localhost:8000/").await?;
    let client: Client = print(client).await?;
    thread::sleep(Duration::from_secs(2));
    let elements:Vec<Element>  = client.find_all(Locator::Css(".pic")).await?;
    fs::create_dir_all("downloads")?;
    println!("{}", elements.len());

    for i in elements.iter() {
        let src = i.text().await?;
        let name = i.attr("id").await.unwrap_or_default();
        match name {
            Some(N) => {
                println!("{}", &N);
                let bytes = base64::decode(src)?;
                let mut path = PathBuf::new();
                path.push("downloads");
                path.push(format!("{}.jpg", N));
                std::fs::write(path, bytes)?;
            }
            None => {}
        }
    }
        
    client.close().await?;
    Ok(())
}