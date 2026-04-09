CREATE TABLE IF NOT EXISTS users (
    id           UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    username     VARCHAR(50)  UNIQUE NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    bio          TEXT         NOT NULL DEFAULT '',
    avatar_url   TEXT,
    banner_url   TEXT,
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT now()
);

INSERT INTO users (id, username, display_name, avatar_url, banner_url) VALUES
    ('00000000-0000-0000-0000-000000000001', 'alice',   'Alice',   'https://api.dicebear.com/9.x/pixel-art/svg?seed=alice',   'https://picsum.photos/seed/alice/600/200'),
    ('00000000-0000-0000-0000-000000000002', 'bob',     'Bob',     'https://api.dicebear.com/9.x/pixel-art/svg?seed=bob',     'https://picsum.photos/seed/bob/600/200'),
    ('00000000-0000-0000-0000-000000000003', 'charlie', 'Charlie', 'https://api.dicebear.com/9.x/pixel-art/svg?seed=charlie', 'https://picsum.photos/seed/charlie/600/200'),
    ('00000000-0000-0000-0000-000000000004', 'diana',   'Diana',   'https://api.dicebear.com/9.x/pixel-art/svg?seed=diana',   'https://picsum.photos/seed/diana/600/200'),
    ('00000000-0000-0000-0000-000000000005', 'evan',    'Evan',    'https://api.dicebear.com/9.x/pixel-art/svg?seed=evan',    'https://picsum.photos/seed/evan/600/200');
