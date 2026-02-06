// src/types.rs

use reqwest;
use rusqlite::{params, Connection, Result as SqlResult};
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Debug)]
pub struct Embedding {
    pub label: String,
    pub vector: Vec<f32>,
}

#[derive(Deserialize)]
struct GeminiResponse {
    embedding: EmbeddingValues,
}

#[derive(Deserialize)]
struct BatchGeminiResponse {
    embeddings: Vec<EmbeddingValues>,
}

#[derive(Deserialize)]
struct EmbeddingValues {
    values: Vec<f64>,
}

impl Embedding {
    /// Compute cosine distance between two vectors.
    /// Returns:
    /// - 0.0 for identical vectors
    /// - 2.0 for opposite, zero, or mismatched vectors
    fn cosine_distance(a: &[f32], b: &[f32]) -> f64 {
        if a.len() != b.len() {
            return 2.0;
        }

        let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
        let mag_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let mag_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if mag_a == 0.0 || mag_b == 0.0 {
            return 2.0;
        }

        (1.0 - dot / (mag_a * mag_b)) as f64
    }

    /// Initialize the database schema.
    /// Vector math is performed in Rust; no SQLite vector extension is used.
    pub fn init_db(conn: &Connection) -> SqlResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS embeddings (
                id INTEGER PRIMARY KEY,
                label TEXT NOT NULL,
                vector BLOB NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    /// Persist this embedding to SQLite.
    pub fn commit(&self, conn: &Connection) -> SqlResult<()> {
        let bytes: &[u8] = bytemuck::cast_slice(&self.vector);

        conn.execute(
            "INSERT INTO embeddings (label, vector) VALUES (?1, ?2)",
            params![&self.label, bytes],
        )?;
        Ok(())
    }

    /// Perform a naive similarity search.
    /// NOTE: This performs a full table scan and is suitable only for small datasets.
    pub fn search(&self, conn: &Connection, limit: usize) -> SqlResult<Vec<(String, f64)>> {
        let mut stmt = conn.prepare("SELECT label, vector FROM embeddings")?;

        let mut results: Vec<(String, f64)> = stmt
            .query_map([], |row| {
                let label: String = row.get(0)?;
                let bytes: Vec<u8> = row.get(1)?;
                let stored: &[f32] = bytemuck::cast_slice(&bytes);

                let distance = Self::cosine_distance(&self.vector, stored);
                Ok((label, distance))
            })?
            .collect::<Result<_, _>>()?;

        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        results.truncate(limit);
        Ok(results)
    }

    /// Create a reusable HTTP client.
    pub fn create_client() -> Result<reqwest::Client, reqwest::Error> {
        reqwest::Client::builder().build()
    }

    /// Convert a single piece of text into a vector using Gemini.
    pub async fn vectorize(
        text: &str,
        client: &reqwest::Client,
    ) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let key = env::var("GEMINI_API_KEY")?;

        let body = json!({
            "model": "models/gemini-embedding-001",
            "content": {
                "parts": [{ "text": text }]
            }
        });

        let url =
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-embedding-001:embedContent";

        let res = client
            .post(url)
            .header("x-goog-api-key", &key)
            .json(&body)
            .send()
            .await?
            .json::<GeminiResponse>()
            .await?;

        Ok(res
            .embedding
            .values
            .into_iter()
            .map(|v| v as f32)
            .collect())
    }

    /// Convert multiple texts into vectors using Gemini batch embedding.
    pub async fn batch_vectorize(
        texts: &[String],
        client: &reqwest::Client,
    ) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        let key = env::var("GEMINI_API_KEY")?;

        let requests: Vec<_> = texts
            .iter()
            .map(|text| {
                json!({
                    "model": "models/gemini-embedding-001",
                    "content": {
                        "parts": [{ "text": text }]
                    }
                })
            })
            .collect();

        let body = json!({ "requests": requests });

        let url =
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-embedding-001:batchEmbedContents";

        let res = client
            .post(url)
            .header("x-goog-api-key", &key)
            .json(&body)
            .send()
            .await?
            .json::<BatchGeminiResponse>()
            .await?;

        Ok(res
            .embeddings
            .into_iter()
            .map(|e| e.values.into_iter().map(|v| v as f32).collect())
            .collect())
    }

    /// Construct a single embedding from text.
    pub async fn new(
        label: String,
        client: &reqwest::Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let vector = Self::vectorize(&label, client).await?;
        Ok(Self { label, vector })
    }

    /// Construct multiple embeddings from text using batch vectorization.
    pub async fn batch_new(
        labels: Vec<String>,
        client: &reqwest::Client,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let vectors = Self::batch_vectorize(&labels, client).await?;

        Ok(labels
            .into_iter()
            .zip(vectors)
            .map(|(label, vector)| Embedding { label, vector })
            .collect())
    }
}
