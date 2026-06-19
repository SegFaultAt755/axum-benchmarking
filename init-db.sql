CREATE TABLE IF NOT EXISTS requests (
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    age INTEGER NOT NULL
);

INSERT INTO requests (name, description, age) VALUES
    ('Alice Johnson', 'Software engineer with a passion for open-source projects', 28),
    ('Bob Smith', 'High school history teacher who enjoys marathon running', 34),
    ('Carol Williams', 'Graphic designer specializing in minimalist branding', 42),
    ('David Brown', 'Data analyst interested in machine learning and statistics', 31),
    ('Eve Davis', 'Freelance writer and avid traveler currently living abroad', 26),
    ('Frank Miller', 'Automotive mechanic with over 15 years of experience', 39),
    ('Grace Wilson', 'Environmental scientist focusing on sustainable urban planning', 29),
    ('Henry Moore', 'Retired chef who now teaches community cooking classes', 45),
    ('Ivy Taylor', 'Registered nurse working in a busy metropolitan hospital', 33),
    ('Jack Anderson', 'Aspiring entrepreneur launching a local tech startup', 27);
