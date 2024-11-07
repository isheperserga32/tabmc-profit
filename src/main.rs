mod analyzer;
mod models;
mod utils;

use std::io::{self, Write};
use std::path::PathBuf;
use std::env;

use analyzer::LogAnalyzer;
use anyhow::Result;
use glob::glob;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let path = PathBuf::from(&args[1]);
        if path.exists() && path.extension().and_then(|s| s.to_str()) == Some("log") {
            analyze_file(&path)?;
            println!("\nPress Enter to exit...");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            return Ok(());
        }
    }

    let log_files: Vec<PathBuf> = glob("*.log")?
        .filter_map(Result::ok)
        .collect();

    if log_files.is_empty() {
        println!("No log files found in the current directory.");
        wait_for_enter()?;
        return Ok(());
    }

    println!("Available .log files:");
    for (i, file) in log_files.iter().enumerate() {
        println!("{}. {}", i + 1, file.display());
    }

    let selected_file = select_file(&log_files)?;
    analyze_file(&selected_file)?;
    wait_for_enter()?;

    Ok(())
}

fn analyze_file(path: &PathBuf) -> Result<()> {
    println!("\nAnalyzing {}...\n", path.display());

    let mut analyzer = LogAnalyzer::new();
    analyzer.process_file(path)?;
    analyzer.display_summary();
    analyzer.display_player_summary();

    Ok(())
}

fn select_file(files: &[PathBuf]) -> Result<PathBuf> {
    loop {
        print!("Enter the number of the file to analyze: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if let Ok(num) = input.trim().parse::<usize>() {
            if num > 0 && num <= files.len() {
                return Ok(files[num - 1].clone());
            }
        }
        println!("Invalid input. Please enter a number between 1 and {}", files.len());
    }
}

fn wait_for_enter() -> Result<()> {
    println!("\nPress Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}