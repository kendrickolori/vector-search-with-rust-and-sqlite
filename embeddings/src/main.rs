// main.rs
pub mod types;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use crate::types::Embedding;
use rusqlite::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("embeddings.db")?;
    let client = Embedding::create_client()?;

    // Initialize database with vector support
    Embedding::init_db(&conn)?;

    println!("=== FAQ Search System ===");
    println!("Commands:");
    println!("  search <query>  - Search for similar questions");
    println!("  load            - Load FAQ from faq.txt");
    println!("  optimize        - Optimize vector index for faster search");
    println!("  quit            - Exit program");
    println!();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];

        match command {
            "quit" | "exit" | "q" => {
                println!("Goodbye!");
                break;
            }
            "load" => {
                println!("Loading FAQ...");
                load_faq(&client, &conn).await?;
                println!("✓ FAQ loaded successfully!");
                println!("  Tip: Run 'optimize' to speed up searches");
            }
            "optimize" => {
                println!("Optimizing vector index...");
                // Placeholder: implement Embedding::optimize_db(conn) when ready
                // Embedding::optimize_db(&conn)?;
                println!("✓ Optimization complete (placeholder)");
            }
            "search" => {
                if parts.len() < 2 {
                    println!("Usage: search <your question>");
                    continue;
                }
                search_faq(parts[1], &client, &conn).await?;
            }
            _ => {
                // Treat any other input as a search query
                search_faq(input, &client, &conn).await?;
            }
        }
    }

    Ok(())
}

async fn search_faq(
    query: &str,
    client: &reqwest::Client,
    conn: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nSearching for: \"{}\"", query);
    println!("Generating embedding...");

    let query_embedding = Embedding::new(query.to_string(), client).await?;
    let results = query_embedding.search(conn, 3)?;

    if results.is_empty() {
        println!("No results found. Try loading the FAQ first with 'load' command.");
        return Ok(());
    }

    println!("\n--- Top {} Results ---", results.len());
    for (i, (label, distance)) in results.iter().enumerate() {
        let similarity = 1.0 - distance;
        println!("\n{}. [Similarity: {:.2}%]", i + 1, similarity * 100.0);
        println!("   {}", label);
        if similarity > 0.7 {
            println!("   ✓ Strong match!");
        }
    }
    println!();

    Ok(())
}

async fn load_faq(
    client: &reqwest::Client,
    conn: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = "faq.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut current_question = String::new();
    let mut current_answer = String::new();
    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("===") {
            continue;
        }

        if trimmed.starts_with("Q: ") {
            if !current_question.is_empty() && !current_answer.is_empty() {
                let combined = format!("Q: {}\nA: {}", current_question, current_answer);
                let embedding = Embedding::new(combined, client).await?;
                embedding.commit(conn)?;
                count += 1;
                print!("\rEmbedded {} questions...", count);
                io::stdout().flush()?;
            }

            current_question = trimmed.strip_prefix("Q: ").unwrap().to_string();
            current_answer.clear();
        } else if trimmed.starts_with("A: ") {
            current_answer = trimmed.strip_prefix("A: ").unwrap().to_string();
        } else if !current_answer.is_empty() {
            current_answer.push('\n');
            current_answer.push_str(trimmed);
        }
    }

    // Commit last Q&A pair
    if !current_question.is_empty() && !current_answer.is_empty() {
        let combined = format!("Q: {}\nA: {}", current_question, current_answer);
        let embedding = Embedding::new(combined, client).await?;
        embedding.commit(conn)?;
        count += 1;
    }

    println!("\n✓ Total embedded: {} Q&A pairs", count);
    Ok(())
}
