use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::{Arc, RwLock};

use anyhow::{Context, Result};
use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;
use rayon::prelude::*;

use crate::models::{ItemInfo, PlayerInfo, ITEM_PRICES};
use crate::utils::normalize_text;

lazy_static! {
    static ref PURCHASE_PATTERN: Regex =
        Regex::new(r"([^\s]+) (ᴢᴀᴋᴜᴘɪʟ|zakupil) (.+)").unwrap();
}

pub struct LogAnalyzer {
    item_counts: HashMap<String, u32>,
    player_purchases: HashMap<String, PlayerInfo>,
}

impl LogAnalyzer {
    pub fn new() -> Self {
        Self {
            item_counts: HashMap::new(),
            player_purchases: HashMap::new(),
        }
    }

    pub fn process_file(&mut self, path: &Path) -> Result<()> {
        let file = File::open(path).context("Failed to open log file")?;
        let reader = io::BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

        let recent_messages = Arc::new(RwLock::new(HashMap::new()));
        let item_counts = Arc::new(RwLock::new(HashMap::new()));
        let player_purchases = Arc::new(RwLock::new(HashMap::new()));

        lines.par_iter().for_each(|line| {
            if let Some(caps) = PURCHASE_PATTERN.captures(line) {
                let timestamp = &line[..5];
                let message = &line[23..];

                let should_process = {
                    let recent = recent_messages.read().unwrap();
                    let last_seen = recent.get(message);
                    last_seen.map_or(true, |ts| ts != timestamp)
                };

                if should_process {
                    recent_messages.write().unwrap().insert(message.to_string(), timestamp.to_string());
                    
                    let player = normalize_text(&caps[1]);
                    let item = normalize_text(&caps[3]);

                    {
                        let mut counts = item_counts.write().unwrap();
                        *counts.entry(item.clone()).or_insert(0) += 1;
                    }

                    {
                        let mut purchases = player_purchases.write().unwrap();
                        let player_info = purchases
                            .entry(player.clone())
                            .or_insert_with(|| PlayerInfo::new(player.clone()));

                        *player_info.items.entry(item.clone()).or_insert(0) += 1;

                        if let Some(&price) = ITEM_PRICES.get(item.as_str()) {
                            player_info.total_spent += price;
                        }
                    }
                }
            }
        });

        self.item_counts = item_counts.read().unwrap().clone();
        self.player_purchases = player_purchases.read().unwrap().clone();

        Ok(())
    }

    pub fn display_summary(&self) {
        if self.item_counts.is_empty() {
            println!("{}", "No purchases found in the log file.".red());
            return;
        }
    
        println!("\n{}", "=== Purchase Summary ===".cyan().bold());
    
        let mut items: Vec<ItemInfo> = self.item_counts
            .iter()
            .map(|(name, &count)| {
                let price = ITEM_PRICES.get(name.as_str()).copied().unwrap_or(0.0);
                ItemInfo {
                    name: name.clone(),
                    price,
                    count,
                }
            })
            .collect();
    
        items.sort_by(|a, b| b.count.cmp(&a.count));
    
        let mut total_profit = 0.0;
    
        for item in &items {
            let profit = f64::from(item.count) * item.price;
            total_profit += profit;
    
            // no more \t\t\t
            let display_name = match item.name.as_str() {
                "motyke hallowenowa (10x10, az 100 blokow na raz)" => "motyka 10x10",
                "przepustka (mnozy zdobywane cukierki x3!)" => "przepustka na cuksy",
                name => name,
            };
    
            println!(
                "• {:<40} {:>3}x {:>6.2} PLN each - {:>7.2} PLN total",
                display_name.white(),
                item.count.to_string().green(),
                item.price,
                profit
            );
        }
    
        println!("\n{}", "─".repeat(70));
        println!("{} {:>7.2} PLN", 
            "Total:".bold(), 
            total_profit.to_string().green().bold()
        );
    }

    pub fn display_player_summary(&self) {
        println!("\n{}", "=== Player Purchase Summary ===".cyan().bold());
    
        let mut players: Vec<&PlayerInfo> = self.player_purchases.values().collect();
        players.sort_by(|a, b| b.total_spent.partial_cmp(&a.total_spent).unwrap());
    
        for (i, player) in players.iter().enumerate() {
            println!(
                "{}. {} - spent: {:.2} PLN",
                i + 1,
                player.name.magenta().bold(),
                player.total_spent.to_string().yellow()
            );
    
            print!("   Items: ");
            let items: Vec<String> = player.items
                .iter()
                .map(|(item, count)| format!("{} ({}x)", item, count))
                .collect();
    
            let mut current_line_length = 10;
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                    current_line_length += 2;
                }
                if current_line_length + item.len() > 80 {
                    print!("\n         ");
                    current_line_length = 9;
                }
                print!("{}", item.white());
                current_line_length += item.len();
            }
            println!();
        }
    }
}