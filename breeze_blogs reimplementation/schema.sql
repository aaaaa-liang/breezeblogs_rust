CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    region VARCHAR(50) DEFAULT 'US',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
 
CREATE TABLE interests (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    interest VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_interests_user 
        FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE
);


CREATE TABLE blogposts (
    id INT AUTO_INCREMENT PRIMARY KEY,
    interest VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
 
INSERT INTO blogposts (interest, content) VALUES
-- sports
('sports', 'Top 10 football matches to watch this season.'),
('sports', 'How to stay active even during a busy week.'),

-- fitness
('fitness', '5-minute workouts you can do at home.'),
('fitness', 'Nutrition tips to boost your workout performance.'),

-- music
('music', 'The evolution of jazz music over the decades.'),
('music', 'Top playlists to get you through your workday.'),

-- food
('food', 'Easy 5-ingredient recipes for weeknight dinners.'),
('food', 'Exploring street food around the world.'),

-- fashion
('fashion', 'Top 2025 fashion trends you need to know.'),
('fashion', 'How to style your wardrobe for every season.'),

-- read
('read', '10 must-read books for personal growth.'),
('read', 'Exploring classic literature: a beginner’s guide.'),

-- travel
('travel', 'Top 5 hidden travel gems in Europe.'),
('travel', 'Tips for traveling on a budget.'),

-- tech (new)
('tech', 'The rise of AI and what it means for the future.'),
('tech', 'Top 5 programming languages to learn in 2025.'),

-- movies (new)
('movies', 'Best movies released this year to watch now.'),
('movies', 'The history of cinema: a journey through film.'),

-- lifestyle (new)
('lifestyle', 'Morning routines of highly productive people.'),
('lifestyle', 'How to cultivate mindfulness in everyday life.');

CREATE TABLE emails (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
