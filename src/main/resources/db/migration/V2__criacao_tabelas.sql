CREATE EXTENSION IF NOT EXISTS pgcrypto;

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'camera_status') THEN
    CREATE TYPE camera_status AS ENUM ('active', 'inactive', 'maintenance');
  END IF;

  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'occurrence_status') THEN
    CREATE TYPE occurrence_status AS ENUM ('new', 'in_review', 'resolved', 'closed');
  END IF;

  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'frequency_type') THEN
    CREATE TYPE frequency_type AS ENUM ('one_time', 'recurring');
  END IF;
END$$;

CREATE OR REPLACE FUNCTION trg_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS users (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name        TEXT NOT NULL CHECK (length(btrim(name)) > 0),
  email       TEXT NOT NULL,
  password    TEXT NOT NULL,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'ux_users_email_lower') THEN
    CREATE UNIQUE INDEX ux_users_email_lower ON users (lower(email));
  END IF;
END$$;

CREATE TABLE IF NOT EXISTS cameras (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name        TEXT NOT NULL CHECK (length(btrim(name)) > 0),
  region      TEXT NOT NULL CHECK (length(btrim(region)) > 0),
  status      camera_status NOT NULL DEFAULT 'active',
  created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS occurrences (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  description   TEXT NOT NULL CHECK (length(btrim(description)) > 0),
  created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  finalized_at  TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS camera_evidences (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  file_path     TEXT NOT NULL CHECK (length(btrim(file_path)) > 0),
  created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  camera_id     UUID NOT NULL REFERENCES cameras (id) ON DELETE CASCADE,
  occurrence_id UUID NULL     REFERENCES occurrences (id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS occurrence_statuses (
  id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  occurrence_id  UUID NOT NULL REFERENCES occurrences (id) ON DELETE CASCADE,
  status         occurrence_status NOT NULL,
  status_date    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS app_occurrence (
  occurrence_id UUID PRIMARY KEY REFERENCES occurrences (id) ON DELETE CASCADE,
  photo_url     TEXT NOT NULL CHECK (length(btrim(photo_url)) > 0),
  description   TEXT NOT NULL CHECK (length(btrim(description)) > 0),
  address       TEXT NOT NULL CHECK (length(btrim(address)) > 0),
  frequency     frequency_type NOT NULL DEFAULT 'one_time'
);

CREATE TABLE IF NOT EXISTS app_occurrence_statuses (
  id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  occurrence_id  UUID NOT NULL REFERENCES occurrences (id) ON DELETE CASCADE,
  status         occurrence_status NOT NULL,
  status_date    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_trigger WHERE tgname = 'set_timestamp_users') THEN
    CREATE TRIGGER set_timestamp_users
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION trg_set_timestamp();
  END IF;

  IF NOT EXISTS (SELECT 1 FROM pg_trigger WHERE tgname = 'set_timestamp_cameras') THEN
    CREATE TRIGGER set_timestamp_cameras
    BEFORE UPDATE ON cameras
    FOR EACH ROW EXECUTE FUNCTION trg_set_timestamp();
  END IF;

  IF NOT EXISTS (SELECT 1 FROM pg_trigger WHERE tgname = 'set_timestamp_occurrences') THEN
    CREATE TRIGGER set_timestamp_occurrences
    BEFORE UPDATE ON occurrences
    FOR EACH ROW EXECUTE FUNCTION trg_set_timestamp();
  END IF;
END$$;

CREATE INDEX IF NOT EXISTS idx_camera_evidences_occurrence ON camera_evidences (occurrence_id);
CREATE INDEX IF NOT EXISTS idx_camera_evidences_camera    ON camera_evidences (camera_id);
CREATE INDEX IF NOT EXISTS idx_occurrence_statuses_occ_dt ON occurrence_statuses (occurrence_id, status_date DESC);

INSERT INTO cameras (id, name, region, status)
VALUES ('c1a7e5e3-4b1d-4b1d-a162-466a3e2a0e2a', 'Câmera de Teste 01', 'Corredor A', 'active')
ON CONFLICT (id) DO NOTHING;
