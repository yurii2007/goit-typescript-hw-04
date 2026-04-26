CREATE TYPE transaction_type AS ENUM ('income', 'expense', 'transfer');
CREATE TYPE transaction_status AS ENUM ('completed', 'pending', 'rejected');

CREATE TABLE transaction_categories (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users (id),
  name TEXT NOT NULL,
  icon TEXT,
  color TEXT,
  transaction_type transaction_type NOT NULL,
  is_system BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE INDEX transaction_categories_user_id ON transaction_categories (user_id);

CREATE TABLE transactions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users (id),
  account_id UUID NOT NULL REFERENCES accounts (id),
  category_id UUID REFERENCES transaction_categories (id),
  transaction_type transaction_type NOT NULL,
  status transaction_status NOT NULL DEFAULT 'completed',
  amount BIGINT NOT NULL,
  description TEXT,
  transaction_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE INDEX transactions_user_id ON transactions (user_id);
CREATE INDEX transactions_account_id ON transactions (account_id);
CREATE INDEX transactions_category_id ON transactions (category_id);
CREATE INDEX transactions_transaction_date ON transactions (transaction_date);
CREATE INDEX transactions_status ON transactions (status);
