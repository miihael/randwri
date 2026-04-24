use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use clap::Parser;
use rand::RngExt;
use rand::rngs::SmallRng;
use indicatif::{ProgressBar, ProgressStyle};

/// Parse size string with K, M, G, T suffixes (binary: 1024)
fn parse_size(size_str: &str) -> Result<u64, String> {
    let size_str = size_str.trim();
    if size_str.is_empty() {
        return Err("Empty size string".to_string());
    }

    // Find the last character that is a digit
    let mut split_idx = size_str.len();
    for (i, ch) in size_str.char_indices().rev() {
        if ch.is_ascii_digit() {
            split_idx = i + ch.len_utf8();
            break;
        }
    }

    let (num_str, suffix) = size_str.split_at(split_idx);
    let num: u64 = num_str
        .parse()
        .map_err(|_| format!("Invalid number: {}", num_str))?;

    let multiplier = match suffix.to_uppercase().as_str() {
        "" => 1,
        "K" => 1024,
        "M" => 1024 * 1024,
        "G" => 1024 * 1024 * 1024,
        "T" => 1024_u64.pow(4),
        _ => return Err(format!("Unknown suffix: {}. Use K, M, G, or T", suffix)),
    };

    Ok(num * multiplier)
}

#[derive(Parser)]
#[command(name = "randwri")]
#[command(about = "Fast write random data to a file/device")]
struct Args {
    /// Output file path
    #[arg(short, long)]
    output: PathBuf,

    /// Size of data to write (supports K, M, G, T suffixes, e.g., 1M, 2G)
    #[arg(short, long, default_value = "1G", value_parser = parse_size)]
    size: u64,

    /// Buffer size for writing (in bytes, supports K, M suffixes)
    #[arg(short, long, default_value = "1M", value_parser = parse_size)]
    buffer_size: u64,
}

fn main() {
    let args = Args::parse();

    // Open file/device for writing (supports system devices, no truncation)
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(&args.output)
        .expect("Failed to open output file");
    
    let buffer_size = args.buffer_size.min(args.size) as usize;
    let mut writer = BufWriter::with_capacity(buffer_size, file);

    // Create progress bar
    let pb = ProgressBar::new(args.size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    // Use fast non-crypto RNG (SmallRng is much faster than thread_rng)
    let mut rng: SmallRng = rand::make_rng();
    let mut remaining = args.size;
    let mut written = 0u64;

    // Write data in chunks for better performance
    let mut buffer = vec![0u8; buffer_size];
    const PROGRESS_UPDATE_INTERVAL: u64 = 1024 * 1024; // Update every 1MB

    while remaining > 0 {
        let to_write = remaining.min(buffer_size as u64) as usize;
        
        // Fill buffer with random bytes
        rng.fill(&mut buffer[..to_write]);
        
        // Write buffer
        writer.write_all(&buffer[..to_write]).expect("Failed to write data");
        
        written += to_write as u64;
        remaining -= to_write as u64;
        
        // Update progress bar less frequently for better performance
        if written % PROGRESS_UPDATE_INTERVAL == 0 || remaining == 0 {
            pb.set_position(written);
        }
    }

    writer.flush().expect("Failed to flush data");
    pb.finish_with_message("Complete");
    println!("Successfully wrote {} random bytes to {}", args.size, args.output.display());
}
