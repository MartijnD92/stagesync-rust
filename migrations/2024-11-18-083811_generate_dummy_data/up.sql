INSERT INTO users(id, first_name, last_name, email) 
VALUES
('d43dad85-eb6e-45f5-87c2-24ade9fb678f', 'Martijn', 'Doensen', 'martijn.doensen@live.nl');

INSERT INTO auth0_users(user_id, auth0_sub) 
VALUES
('d43dad85-eb6e-45f5-87c2-24ade9fb678f', 'github|62554989');

INSERT INTO artists(id, name, fee, currency, user_id) 
VALUES
(gen_random_uuid(), 'The Hillbilly Moonshiners', 3000.00, 'EUR', (SELECT id FROM users WHERE email = 'martijn.doensen@live.nl')),
(gen_random_uuid(), 'Pink Floyd', 1_000_000.00, 'GBP', (SELECT id FROM users WHERE email = 'martijn.doensen@live.nl')),
(gen_random_uuid(), 'AC/DC', 2_000_000.50, 'AUD', (SELECT id FROM users WHERE email = 'martijn.doensen@live.nl'));

INSERT INTO gigs(id, title, location, date, artist_id) 
VALUES
(gen_random_uuid(), 'Zwarte Cross', 'Lichtenvoorde', '2022-11-23T07:56:30.214162+00:00', (SELECT id FROM artists WHERE name = 'The Hillbilly Moonshiners'));
