CREATE TABLE IF NOT EXISTS tweets (
    id         UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id  UUID         NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content    VARCHAR(280) NOT NULL,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT now()
);

INSERT INTO tweets (author_id, content, created_at) VALUES
    ('00000000-0000-0000-0000-000000000001', 'Just shipped my first Rust API!', now() - interval '5 minutes'),
    ('00000000-0000-0000-0000-000000000002', 'The borrow checker is not your enemy.', now() - interval '1 hour'),
    ('00000000-0000-0000-0000-000000000003', 'Building a Twitter clone in Rust is fun!', now() - interval '3 hours');