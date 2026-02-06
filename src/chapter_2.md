# Chapter 2

In this chapter, we will implement a basic vector search over an FAQ file.  
If you're not already familiar with the concept of vector search or need a refresher, I encourage you to check out the previous chapter.

Let's get started.

---

## Setting Up the Project

Create a new cargo project and add these to your dependencies:

```toml
[dependencies]
bytemuck = "1.25.0"
reqwest = { version = "0.13.1", features = ["json", "native-tls"] }
rusqlite = "0.38.0"
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.149"
tokio = { version = "1.49.0", features = ["full"] }
```

---

## The Embedding Type

We need a type-safe way to represent embeddings in Rust.

```rust
{{#include ../embeddings/src/types.rs:9:13}}
```

---

## Generating Vectors with Embedding Models

To get our vectors, we use special models known as embedding models.  
These models are trained specifically to represent text in vector format, so you can reliably reproduce the same vector for the same text.

The model we're using is **gemini-embedding-001**.  
This model uses 768-dimensional coordinates to encode data.

First, grab your API key from [Google AI Studio](https://aistudio.google.com/app/apikey).

Set your Gemini API key as an environment variable:

```bash
export GEMINI_API_KEY="your_api_key_here"
```

### Example: REST API Call

Here's an example request to the Gemini embedding model:

```bash
curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-embedding-001:embedContent" \
    -H "Content-Type: application/json" \
    -H "x-goog-api-key: ${GEMINI_API_KEY}" \
    -d '{
        "model": "models/gemini-embedding-001",
        "content": {
            "parts": [{
                "text": "What is the meaning of life?"
            }]
        }
    }'
```

>[!IMPORTANT]
>Whenever you use a model to embed data, ensure the same model is used to embed search queries.

Obviously we're not going to be working with curl.  
We need (I know, I know) a type-safe way of making HTTP requests—so we're using reqwest.

```rust
{{#include ../embeddings/src/types.rs:15:29}}
```

Now we can make calls to the API via reqwest:

```rust
impl Embedding {
{{#include ../embeddings/src/types.rs:97:135}}
}
```

---

**[WIP - More sections coming soon]**
