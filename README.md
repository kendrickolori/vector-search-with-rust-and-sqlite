# Semantic Search from Scratch: Building a FAQ Chatbot

You might have wondered: how does Google know to show you exactly what you need even when you type in something else?

Meanwhile, your SQL queries return null when even one word is off.

The answer is **semantic search**.

You might have used it before when building RAG systems or working with AI tools, but do you actually know how it works?

"Semantics" is all about meaning and context, and here you'll learn how to encode them in a way that computers can understand so we can make the best use of it.

In this guide, we'll build a fully functional semantic search system from scratch (mostly lol), and by the end, we'll hook it up to a chatbot for RAG.

## What You'll Build

By the end of this guide, you'll have:

- A semantic search engine that understands meaning
- An FAQ chatbot powered by vector embeddings  
- A working RAG (Retrieval-Augmented Generation) system
- Deep understanding of how it all works 

Although we'll build our system with SQLite, the patterns, architecture, and design principles you'll learn here are not SQLite-specific. They apply to any semantic search system you want to build—whether you later move to Postgres with pgvector, specialized vector databases, or even in-memory solutions.

SQLite is a solid choice for this because it has such a low resource footprint and negligible setup cost. It exists on most devices, so that's a plus for you if you ever want to deploy such an app.

## Who This Guide Is For

This guide assumes you already have:

- Basic programming skills
- Some familiarity with databases (SQL basics)
- Curiosity about how AI search works

Optional but helpful:

- Experience with Rust or Python (Python version coming soon)
- Prior exposure to embeddings or vector concepts
- Familiarity with chatbots or LLMs

If you need a refresher, start here:

- [Rust documentation](https://doc.rust-lang.org/)
- [Python documentation](https://docs.python.org/)
- [SQLite basics](https://www.sqlite.org/docs.html)

## Setup Instructions

```bash
# Clone the repo
git clone https://github.com/kendrickolori/vector-search-with-rust-and-sqlite
cd vector-search-with-rust-and-sqlite/embeddings

# (Optional) Replace faq.txt with your own FAQ file

# Run it
cargo run
```

## Chapters

**[Chapter 1: Where Traditional Queries Fail](https://kendrickolori.github.io/vector-search-with-rust-and-sqlite/chapter_1.html)**  
We explore the limits of traditional SQL and what we can use in its place.

**[Chapter 2: Building Vector Search](https://kendrickolori.github.io/vector-search-with-rust-and-sqlite/chapter_2.html)**  
Generate embeddings, store them in SQLite, calculate cosine similarity, and search.

**Chapter 3: The FAQ Chatbot** *(coming soon)*  
Hook it all up to build a complete RAG system.

---

This is very much a work in progress, so it's still rough around the edges.

I'll appreciate any support, contributions, corrections—it keeps me going.

Also star and share this if you find it helpful!

