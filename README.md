 # GDPR-Enforcement Framework Evaluation: **Sesame**

This repository contains a Rust reimplementation of the BreezeBlogs demo application along with a second version enhanced with the Sesame privacy framework. The project evaluates Sesame's GDPR-related technical guarantees and its runtime performance.

## Repository Structure

breezeblogs_rust/
- breeze_blogs reimplementation/
  - src/
    - db.rs
    - main.rs
    - routes.rs
    - schema.sql
  - Cargo.lock
  - Cargo.toml
  - benchmark.sh
  - benchmark_3_endpoints.sh
- breeze_blogs reimplementation_sesame/
  - src/
    - db.rs
    - main.rs
    - policy.rs
    - routes.rs
    - schema.sql
  - Cargo.lock
  - Cargo.toml
  - benchmark.sh
  - benchmark_3_endpoints.sh
- compare_results/
  - benchmark outputs for baseline and Sesame versions

## Project Overview

The goal of this project is to evaluate Sesame using:
- A GDPR technical compliance checklist (Kalinowski et al., POPETS 2025)
- Runtime overhead measurements on three endpoints:
  - blog-posts
  - send-news-mails
  - interests

Sesame provides:
- **Policy Containers (PCon):** attach policies to data with compile-time verification
- **Privacy Regions:** restrict code that handles sensitive data based on regional constraints

Both versions of BreezeBlogs maintain identical database schemas for direct comparison.

**To run the implementation, please refer to How to Set Up BreezeBlogs_Readme.md.** 

## Folder Descriptions

### breeze_blogs reimplementation/
Baseline BreezeBlogs application without any privacy enforcement. Includes:
- User registration and login
- Setting and retrieving interests
- Blogposts retrieval
- Standard Rocket + MySQL stack

### breeze_blogs reimplementation_sesame/
Same application, but with Sesame integrated. Includes:
- `policy.rs` defining PCon policies
- Request structs wrapped in PCon types
- Region-based enforcement via Sesame
- Refactored handlers operating on policy-bound values

## Dependencies (What to Install)

- Rust toolchain
- channel = "nightly-2024-01-06"
- MySQL 8.x
- Rocket
- mysql crate
- serde, serde_json
- dotenvy
- Sesame (`sesame` and `sesame_derive`)
- wrk (used for benchmarking)

## Endpoints (Both Versions)

- POST /register
- POST /login
- POST /interests
- GET /interests
- GET /blogposts/:interest
- POST /email
- GET /send_new_mail
- POST /logout
- GET /session

The Sesame version enforces compile-time privacy constraints on all these endpoints.
