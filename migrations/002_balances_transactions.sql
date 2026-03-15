CREATE TABLE balances (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users (id),
  name TEXT NOT NULL,
  currency TEXT NOT NULL DEFAULT 'USD',
  amount BIGINT NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX balance_user_id ON balances (user_id);

CREATE TABLE transaction_categories (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users (id),
  name TEXT NOT NULL,
  icon TEXT,
  color TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX transaction_categories_user_id ON transaction_categories (user_id);

CREATE TYPE balance_transaction_type AS ENUM ('income', 'expense');

CREATE TABLE balance_transactions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users (id),
  balance_id UUID REFERENCES balances (id),
  category_id UUID REFERENCES transaction_categories (id),
  transaction_type balance_transaction_type NOT NULL,
  amount BIGINT NOT NULL,
  description TEXT,
  transaction_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX balance_transaction_user_id ON balance_transactions (user_id);
CREATE INDEX balance_transaction_balance_id ON balance_transactions (balance_id);

