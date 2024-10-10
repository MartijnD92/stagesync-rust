INSERT INTO artists(id, name, fee, currency) 
VALUES
(gen_random_uuid(), 'The Hillbilly Moonshiners', 3000.00, 'EUR'),
(gen_random_uuid(), 'Pink Floyd', 1_000_000.00, 'GBP'),
(gen_random_uuid(), 'AC/DC', 2_000_000.50, 'AUD');

INSERT INTO gigs(id, title, location, date, artist_id) 
VALUES
(gen_random_uuid(), 'Zwarte Cross', 'Lichtenvoorde', '2022-11-23T07:56:30.214162+00:00', (SELECT id FROM artists WHERE name = 'The Hillbilly Moonshiners'))
