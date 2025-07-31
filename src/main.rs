use anyhow::Result;
use clap::Parser;
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::Path,
    sync::Arc,
    time::Duration,
};
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(
    name = "m3u8-checker",
    version = "v1.0.0",
    about = "A tool to extract valid m3u8 link.",
    long_about = None
)]

struct Args {
    /// Input directory
    #[arg(short, long, default_value = ".")]
    input_dir: String,

    /// Output file
    #[arg(short, long, default_value = "valid-iptv.m3u8")]
    output_file: String,

    /// HTTP request timeout
    #[arg(short, long, default_value = "10")]
    timeout: u64,
}

#[derive(Debug, Clone, Default)]
struct M3U8Entry {
    description: String,
    url: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut file = fs::File::create(&args.output_file)?;
    let (tx, mut rx) = mpsc::channel(1024);
    let mut entries = vec![];
    let mut unique_set = HashMap::new();

    for entry in fs::read_dir(&args.input_dir)? {
        let entry = entry?;
        let path = entry.path();

        let ext = path.extension().and_then(|s| s.to_str());
        if ext == Some("m3u8") || ext == Some("m3u") {
            println!("[Info] Parse: {:?}", path);
            if let Ok(items) = parse_m3u8_file(&path, &mut unique_set).await {
                entries.extend(items);
            }
        }
    }

    for entry in entries.into_iter() {
        check_url_validity(entry, args.timeout, Arc::new(tx.clone()));
    }

    drop(tx);

    while let Some(entry) = rx.recv().await {
        println!("[Info] Valid: {}", entry.url);
        let item = format!("{}\n{}\n", entry.description, entry.url);
        _ = file.write(item.as_bytes());
    }

    println!("Finished!");
    Ok(())
}

async fn parse_m3u8_file(
    input_path: &Path,
    unique_set: &mut HashMap<String, ()>,
) -> Result<Vec<M3U8Entry>> {
    let content = fs::read_to_string(input_path)?;
    let lines: Vec<&str> = content.lines().collect();

    let mut i = 0;
    let mut entries = vec![];

    while i < lines.len() {
        if lines[i].starts_with("#EXTINF:") {
            if i + 1 < lines.len()
                && !lines[i + 1].is_empty()
                && !lines[i + 1].starts_with("https://")
            {
                let description = lines[i].to_string();
                let url = lines[i + 1].to_string();

                if !unique_set.contains_key(&url) {
                    unique_set.insert(url.clone(), ());
                    entries.push(M3U8Entry { description, url });
                }

                i += 2;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    Ok(entries)
}

fn check_url_validity(entry: M3U8Entry, timeout: u64, tx: Arc<mpsc::Sender<M3U8Entry>>) {
    tokio::spawn(async move {
        match reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap()
            .head(&entry.url)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() || response.status().is_redirection() {
                    _ = tx.send(entry).await;
                } else {
                    eprintln!("[Warn] Invalid: {}", entry.url);
                }
            }
            _ => eprintln!("[Warn] Invalid: {}", entry.url),
        }
    });
}
