# BreezeBlogs_Rust Setup Guide

This guide helps you set up the BreezeBlogs_Rust project, create the required MySQL database and tables, run the Rust backend, and test endpoints using Postman.

---

## 1. Log in to MySQL
```
mysql -u your_username -p
```

---

## 2. Create a Database
```
CREATE DATABASE breeze_blogs;
USE breeze_blogs;
```

---

## 3. Create Tables

### **Users Table**
```sql
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    region VARCHAR(50) DEFAULT 'US',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### **Interests Table**
```sql
CREATE TABLE interests (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    interest VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_interests_user
        FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE
);
```

### **Blog Post Table**
```sql
CREATE TABLE blogposts (
    id INT AUTO_INCREMENT PRIMARY KEY,
    interest VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### **Insert Blog Post Values**
```sql
INSERT INTO blogposts (interest, content) VALUES
('sports', 'Top 10 football matches to watch this season.'),
('sports', 'How to stay active even during a busy week.'),
('fitness', '5-minute workouts you can do at home.'),
('fitness', 'Nutrition tips to boost your workout performance.'),
('music', 'The evolution of jazz music over the decades.'),
('music', 'Top playlists to get you through your workday.'),
('food', 'Easy 5-ingredient recipes for weeknight dinners.'),
('food', 'Exploring street food around the world.'),
('fashion', 'Top 2025 fashion trends you need to know.'),
('fashion', 'How to style your wardrobe for every season.'),
('read', '10 must-read books for personal growth.'),
('read', 'Exploring classic literature: a beginner’s guide.'),
('travel', 'Top 5 hidden travel gems in Europe.'),
('travel', 'Tips for traveling on a budget.'),
('tech', 'The rise of AI and what it means for the future.'),
('tech', 'Top 5 programming languages to learn in 2025.'),
('movies', 'Best movies released this year to watch now.'),
('movies', 'The history of cinema: a journey through film.'),
('lifestyle', 'Morning routines of highly productive people.'),
('lifestyle', 'How to cultivate mindfulness in everyday life.');
```

### **Emails Table**
```sql
CREATE TABLE emails (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

---

## 4. Clone the Repository
```
git clone https://github.com/aaaaa-liang/breezeblogs_rust
cd breezeblogs_rust
```

---

## 5. Update MySQL Password in the Code
Edit the following files:

- `breeze_blogs reimplementation/src/db.rs` → **line 10**
- `breeze_blogs reimplementation_sesame/src/db.rs` → **line 14**
- `breeze_blogs reimplementation_sesame/src/routes.rs` → **lines 28, 85, 193, 332, 452, 626, 733**

Replace the placeholder password with your MySQL password.

---

## 6. Build and Run the Project
```
cargo build
cargo run
```
Your server should start at:
```
http://127.0.0.1:8000
```

---

## 7. Test Endpoints Using Postman

### **POST /register**
**URL:** `http://127.0.0.1:8000/register`
```json
{
  "username": "charlie",
  "email": "charlie@example.com",
  "password": "charlie321!"
}
```

### **POST /login**
**URL:** `http://127.0.0.1:8000/login`
```json
{
  "email": "charlie@example.com",
  "password": "charlie321!"
}
```

### **POST /interests**
**URL:** `http://127.0.0.1:8000/interests`
```json
{
  "interests": ["sports", "fitness"]
}
```

### **GET /interests**
**URL:** `http://127.0.0.1:8000/interests`
No JSON required.

### **GET /blog-posts**
**URL:** `http://127.0.0.1:8000/blog-posts`
No JSON required.

### **POST /email**
**URL:** `http://127.0.0.1:8000/email`
```json
{
  "email": "charliebrown@example.com"
}
```

### **POST /send-news-mails**
**URL:** `http://127.0.0.1:8000/send-news-mails`
No JSON required.

### **GET /session**
**URL:** `http://127.0.0.1:8000/session`
No JSON required.

---

## Setup Complete
You are now ready to use BreezeBlogs_Rust.