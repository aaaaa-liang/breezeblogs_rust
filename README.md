## GDPR-Enforcement Framework Evaluation: Sesame

A research implementation evaluating GDPR enforcement using the Sesame privacy framework, built as part of DS593 (Privacy in Data Systems).

- `breeze_blogs reimplementation` folder: The baseline BreezeBlogs application in Rust. 
- `breeze_blogs reimplementation_sesame` folder: The BreezeBlogs application with Sesame implementation. 
- `compare_results` folder: Runtime performance results generated from benchmark.sh and benchmark_3_endpoints.sh in respective folders above. 

This repository contains two Rust applications:
breezeblogs_rust/
│
├── breeze_blogs reimplementation/         
│   └── src/
│       ├── db.rs
│       ├── main.rs
│       ├── routes.rs
│       ├── schema.sql
│       └── readme.md
│
└── breeze_blogs reimplementation_sesame/   
    └── src/
        ├── db.rs
        ├── main.rs
        ├── policy.rs
        ├── routes.rs
        ├── schema.sql
        └── readme.md
        
Project Overview
This project reimplements the BreezeBlogs demo web application in Rust and integrates the Sesame privacy framework to measure:
 GDPR Technical Compliance
(using the Kalinowski et al. POPETS 2025 checklist)
 Runtime Overhead
(based on three core endpoints: blog-posts, send-news-mails, and interests)
 Policy Container (PCon) Behavior
Ensures data is wrapped with compile-time–verified policies.
 Privacy Region Enforcement
Restricts when/where sensitive data can be used inside the program.

Key Findings (Summary)
Framework	Runtime Overhead
Fontus	+14%
RuleKeeper	–11%
GDPR-MFOTL	+477%
Sesame (This Work)	–34% (faster than baseline)

📁 Folder Purpose
1. breeze_blogs reimplementation/
The baseline Rust BreezeBlogs application.
Contains:
Core Rocket server
MySQL integration
Endpoints (register, login, set/get interests)
No privacy enforcement layer

2. breeze_blogs reimplementation_sesame/
The same BreezeBlogs app, now modified to use Sesame:
policy.rs – Sesame Policy Container definitions
PCon-wrapped request structs
Privacy Region enforcement
Refactored handlers to operate on PCon values
Same schema as baseline for apples-to-apples comparison

Dependencies (What to Install)

Rust
rocket = "0.5"
mysql = "24"
serde + serde_json
dotenvy (if using .env)
Sesame Framework
From the Sesame repository:
sesame
sesame_derive
either (used internally by Sesame)
Database
MySQL 8.x
Load schema.sql from each folder when testing independently.
Benchmarking Tools
(used in the evaluation)
wrk (or Locust if replicating original study)
Endpoints (Both Versions)
POST /register
POST /login
POST /interests (set interests)
GET /interests (get interests)
GET /blogposts/:interest

The Sesame version enforces privacy constraints automatically at compile time.
Comparative performance analysis
Checklist-based GDPR support classification
If you want, I can generate a "Research Highlights" section or a citation block for your GitHub too.
