use fantoccini::{
    Client, Locator,
    elements::{
        Element
    }
};
use anyhow::Result;
use std::{
    path::PathBuf,
    fs,
    thread::sleep,
    time::Duration
};
use rusqlite::Connection;

#[tokio::main]
async fn main() -> Result<()> {
    println!("http://localhost:9515");
    let client: Client = Client::new("http://localhost:9515").await?;
    
    client.goto("http://localhost:5173/").await?;
    let conn: Connection = Connection::open("cards.cdb")?;
    let count: u64 = conn.query_row(
        "SELECT COUNT(*) FROM datas JOIN texts ON datas.id = texts.id",
        [],
        |row| row.get(0),
    )?;
    println!("{}", count);
    fs::create_dir_all("downloads")?;
    loop {
        let elements:Vec<Element>  = client.find_all(Locator::Css(".pic")).await?;
        if count + 1 == elements.len() as u64 {
            for i in elements.iter().skip(1) {
                let src: String = i.text().await?;
                let name: Option<String> = i.attr("id").await.unwrap_or_default();
                match name {
                    Some(n) => {
                        println!("{}", &n);
                        let bytes: Vec<u8> = base64::decode(src)?;
                        let mut path: PathBuf = PathBuf::new();
                        path.push("downloads");
                        path.push(format!("{}.jpg", n));
                        std::fs::write(path, bytes)?;
                    }
                    None => {}
                }
            }
            break;
        }
        sleep(Duration::from_secs(1));
    }

        
    client.close().await?;
    Ok(())
}