INSERT INTO users (name, email, password)
VALUES (
    'Admin',
    'admin@admin.com',
    '$2a$10$3f83A1pAKLUMjb1L/UTBnO2Mur3ZxOhBL1LG1I5NV4YR1YV3nSS8a'
)
ON CONFLICT (lower(email)) DO NOTHING;