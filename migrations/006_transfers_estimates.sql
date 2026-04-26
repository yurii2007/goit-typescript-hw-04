CREATE TABLE transfers (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users (id),
  from_transaction_id UUID NOT NULL REFERENCES transactions (id),
  to_transaction_id UUID NOT NULL REFERENCES transactions (id),
  exchange_rate NUMERIC(18, 8) DEFAULT 1.0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE INDEX transfers_user_id ON transfers (user_id);
CREATE INDEX transfers_from_transaction_id ON transfers (from_transaction_id);
CREATE INDEX transfers_to_transaction_id ON transfers (to_transaction_id);

CREATE TYPE estimate_period AS ENUM ('weekly', 'monthly', 'yearly');

CREATE TABLE estimates (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users (id),
  category_id UUID NOT NULL REFERENCES transaction_categories (id),
  amount BIGINT NOT NULL,
  period estimate_period NOT NULL DEFAULT 'monthly',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ,
  UNIQUE (user_id, category_id, period)
);

CREATE INDEX estimates_user_id ON estimates (user_id);
CREATE INDEX estimates_category_id ON estimates (category_id);
