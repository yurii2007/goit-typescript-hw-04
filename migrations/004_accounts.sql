ALTER TABLE auth_providers
  ALTER COLUMN user_id SET NOT NULL;

ALTER TABLE auth_providers
  ADD CONSTRAINT auth_providers_provider_provider_user_id_key
  UNIQUE (provider, provider_user_id);

ALTER TABLE user_profile_configs
  ALTER COLUMN user_id SET NOT NULL;

CREATE TYPE account_type AS ENUM ('credit_card', 'checking_card', 'cash', 'investment');

CREATE TABLE accounts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users (id),
  name TEXT NOT NULL,
  account_type account_type NOT NULL,
  currency TEXT NOT NULL DEFAULT 'USD',
  balance BIGINT NOT NULL DEFAULT 0,
  icon TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE INDEX accounts_user_id_account_type ON accounts (user_id, account_type);
