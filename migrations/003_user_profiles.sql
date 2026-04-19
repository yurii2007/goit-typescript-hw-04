CREATE INDEX user_email ON users (email);
DROP INDEX balance_transaction_user_id;
DROP INDEX balance_transaction_balance_id;
DROP TABLE balance_transactions;
DROP TYPE balance_transaction_type;
DROP INDEX transaction_categories_user_id;
DROP TABLE transaction_categories;
DROP INDEX balance_user_id;
DROP TABLE balances;

CREATE TYPE auth_provider AS ENUM ('google', 'github');

CREATE TABLE auth_providers (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users (id),
  provider auth_provider NOT NULL,
  provider_user_id TEXT,
  access_token TEXT,
  refresh_token TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX auth_provider_user_id ON auth_providers (user_id);

CREATE TABLE user_profile_configs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users (id),
  avatar_url TEXT,
  default_currency TEXT DEFAULT 'USD',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX user_profile_config_user_id ON user_profile_configs (user_id);

CREATE OR REPLACE FUNCTION create_default_user_profile_config()
RETURNS TRIGGER AS
$$ 
BEGIN
  INSERT INTO user_profile_configs(user_id) VALUES (NEW.id);
  RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER generate_default_profile_config
AFTER INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION create_default_user_profile_config();
