INSERT INTO artist(id, name, created_at) 
VALUES
(gen_random_uuid(),'The Hillbilly Moonshiners','2022-11-23T07:56:30.214162+00:00'),
(gen_random_uuid(),'Pink Floyd','2022-11-23T07:56:30.214162+00:00'),
(gen_random_uuid(),'AC/DC','2022-12-23T07:56:30.214162+00:00');

INSERT INTO gig(id, title, location, date, artist_id) 
VALUES
(gen_random_uuid(),'Zwarte Cross', 'Lichtenvoorde', '2022-11-23T07:56:30.214162+00:00', (SELECT id FROM artists WHERE name = 'The Hillbilly Moonshiners'))
