-- 约拍平台 PostgreSQL DDL（MVP）
-- 说明：实际以迁移为准，此文件仅作为结构参考基线。

-- 1) 基础与账号体系
CREATE TABLE IF NOT EXISTS users (
  id            BIGSERIAL PRIMARY KEY,
  phone         VARCHAR(32) UNIQUE NOT NULL,
  email         VARCHAR(255) UNIQUE,
  password_hash TEXT,
  status        TEXT NOT NULL CHECK (status IN ('active','frozen','deleted')),
  credit_score  INT NOT NULL DEFAULT 100,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  deleted_at    TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS user_profiles (
  user_id    BIGINT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
  nickname   VARCHAR(64),
  avatar_url TEXT,
  gender     TEXT CHECK (gender IN ('male','female','unknown')),
  birthday   DATE,
  city_id    BIGINT,
  bio        TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS roles (
  id   BIGSERIAL PRIMARY KEY,
  name TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS user_roles (
  user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  role_id BIGINT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
  scope   TEXT,
  PRIMARY KEY (user_id, role_id)
);

CREATE TABLE IF NOT EXISTS sessions (
  id         BIGSERIAL PRIMARY KEY,
  user_id    BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  token      TEXT UNIQUE NOT NULL,
  expired_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS verification_codes (
  id         BIGSERIAL PRIMARY KEY,
  phone      VARCHAR(32) NOT NULL,
  code       VARCHAR(16) NOT NULL,
  expired_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 2) 摄影师/团队体系
CREATE TABLE IF NOT EXISTS photographers (
  id              BIGSERIAL PRIMARY KEY,
  user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  type            TEXT NOT NULL CHECK (type IN ('individual','team')),
  status          TEXT NOT NULL CHECK (status IN ('pending','approved','rejected','frozen')),
  city_id         BIGINT,
  service_area    TEXT,
  rating_avg      NUMERIC(3,2) NOT NULL DEFAULT 0,
  completed_orders INT NOT NULL DEFAULT 0,
  created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS teams (
  id            BIGSERIAL PRIMARY KEY,
  owner_user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name          TEXT NOT NULL,
  status        TEXT NOT NULL CHECK (status IN ('active','frozen','deleted')),
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS team_members (
  team_id BIGINT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
  user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  role    TEXT NOT NULL CHECK (role IN ('admin','member')),
  PRIMARY KEY (team_id, user_id)
);

CREATE TABLE IF NOT EXISTS portfolios (
  id              BIGSERIAL PRIMARY KEY,
  photographer_id BIGINT NOT NULL REFERENCES photographers(id) ON DELETE CASCADE,
  title           TEXT NOT NULL,
  status          TEXT NOT NULL CHECK (status IN ('draft','published','blocked')),
  created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS portfolio_items (
  id           BIGSERIAL PRIMARY KEY,
  portfolio_id BIGINT NOT NULL REFERENCES portfolios(id) ON DELETE CASCADE,
  url          TEXT NOT NULL,
  tags         JSONB,
  cover_flag   BOOLEAN NOT NULL DEFAULT FALSE,
  created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 3) 需求与报价
CREATE TABLE IF NOT EXISTS demands (
  id             BIGSERIAL PRIMARY KEY,
  user_id        BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  type           TEXT NOT NULL,
  city_id        BIGINT,
  location       TEXT,
  schedule_start TIMESTAMPTZ,
  schedule_end   TIMESTAMPTZ,
  budget_min     NUMERIC(12,2),
  budget_max     NUMERIC(12,2),
  people_count   INT,
  style_tags     JSONB,
  status         TEXT NOT NULL CHECK (status IN ('draft','open','closed')),
  is_merchant    BOOLEAN NOT NULL DEFAULT FALSE,
  merchant_id    BIGINT,
  created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS demand_attachments (
  id        BIGSERIAL PRIMARY KEY,
  demand_id BIGINT NOT NULL REFERENCES demands(id) ON DELETE CASCADE,
  file_url  TEXT NOT NULL,
  file_type TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS quotes (
  id             BIGSERIAL PRIMARY KEY,
  demand_id      BIGINT NOT NULL REFERENCES demands(id) ON DELETE CASCADE,
  photographer_id BIGINT,
  team_id        BIGINT,
  total_price    NUMERIC(12,2) NOT NULL,
  status         TEXT NOT NULL CHECK (status IN ('pending','accepted','expired')),
  version        INT NOT NULL DEFAULT 1,
  expires_at     TIMESTAMPTZ,
  created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS quote_items (
  id        BIGSERIAL PRIMARY KEY,
  quote_id  BIGINT NOT NULL REFERENCES quotes(id) ON DELETE CASCADE,
  name      TEXT NOT NULL,
  price     NUMERIC(12,2) NOT NULL,
  quantity  INT NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS quote_versions (
  id          BIGSERIAL PRIMARY KEY,
  quote_id    BIGINT NOT NULL REFERENCES quotes(id) ON DELETE CASCADE,
  version     INT NOT NULL,
  total_price NUMERIC(12,2) NOT NULL,
  items       JSONB NOT NULL,
  note        TEXT,
  created_by  BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 4) 订单与支付（直付）
CREATE TABLE IF NOT EXISTS orders (
  id             BIGSERIAL PRIMARY KEY,
  user_id        BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  photographer_id BIGINT,
  team_id        BIGINT,
  demand_id      BIGINT REFERENCES demands(id),
  quote_id       BIGINT REFERENCES quotes(id),
  status         TEXT NOT NULL CHECK (status IN ('confirmed','paid','ongoing','completed','reviewed','cancelled')),
  pay_type       TEXT NOT NULL CHECK (pay_type IN ('deposit','full','phase')),
  deposit_amount NUMERIC(12,2) DEFAULT 0,
  total_amount   NUMERIC(12,2) NOT NULL,
  service_fee    NUMERIC(12,2) DEFAULT 0,
  schedule_start TIMESTAMPTZ,
  schedule_end   TIMESTAMPTZ,
  cancelled_at   TIMESTAMPTZ,
  created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS order_items (
  id        BIGSERIAL PRIMARY KEY,
  order_id  BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  name      TEXT NOT NULL,
  price     NUMERIC(12,2) NOT NULL,
  quantity  INT NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS payments (
  id         BIGSERIAL PRIMARY KEY,
  order_id   BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  payer_id   BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  payee_id   BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  amount     NUMERIC(12,2) NOT NULL,
  status     TEXT NOT NULL CHECK (status IN ('pending','success','failed')),
  pay_channel TEXT NOT NULL CHECK (pay_channel IN ('wx','alipay','bank')),
  stage      TEXT CHECK (stage IN ('deposit','mid','final')),
  paid_at    TIMESTAMPTZ,
  proof_url  TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS refunds (
  id          BIGSERIAL PRIMARY KEY,
  order_id    BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  applicant_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  amount      NUMERIC(12,2) NOT NULL,
  status      TEXT NOT NULL CHECK (status IN ('pending','approved','rejected','paid')),
  responsible_party TEXT CHECK (responsible_party IN ('user','photographer','merchant')),
  reason      TEXT,
  proof_url   TEXT,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 5) 履约与交付
CREATE TABLE IF NOT EXISTS deliveries (
  id          BIGSERIAL PRIMARY KEY,
  order_id    BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  status      TEXT NOT NULL CHECK (status IN ('pending','submitted','accepted','rejected')),
  submitted_at TIMESTAMPTZ,
  accepted_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS delivery_items (
  id          BIGSERIAL PRIMARY KEY,
  delivery_id BIGINT NOT NULL REFERENCES deliveries(id) ON DELETE CASCADE,
  file_url    TEXT NOT NULL,
  version     TEXT,
  note        TEXT,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 6) 评价与纠纷
CREATE TABLE IF NOT EXISTS reviews (
  id        BIGSERIAL PRIMARY KEY,
  order_id  BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  rater_id  BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  ratee_id  BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  score     INT NOT NULL CHECK (score BETWEEN 1 AND 5),
  tags      JSONB,
  comment   TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS disputes (
  id          BIGSERIAL PRIMARY KEY,
  order_id    BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  initiator_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  status      TEXT NOT NULL CHECK (status IN ('submitted','handling','closed')),
  reason      TEXT,
  resolution  TEXT,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS dispute_evidence (
  id         BIGSERIAL PRIMARY KEY,
  dispute_id BIGINT NOT NULL REFERENCES disputes(id) ON DELETE CASCADE,
  file_url   TEXT NOT NULL,
  note       TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 7) 商户（瑜伽馆）能力
CREATE TABLE IF NOT EXISTS merchants (
  id             BIGSERIAL PRIMARY KEY,
  name           TEXT NOT NULL,
  logo_url       TEXT,
  brand_color    TEXT,
  contact_user_id BIGINT REFERENCES users(id),
  status         TEXT NOT NULL CHECK (status IN ('pending','approved','frozen')),
  created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS merchant_locations (
  id          BIGSERIAL PRIMARY KEY,
  merchant_id BIGINT NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
  name        TEXT NOT NULL,
  address     TEXT,
  city_id     BIGINT
);

CREATE TABLE IF NOT EXISTS merchant_users (
  id          BIGSERIAL PRIMARY KEY,
  merchant_id BIGINT NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
  user_id     BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  role        TEXT NOT NULL CHECK (role IN ('requester','approver','finance'))
);

CREATE TABLE IF NOT EXISTS merchant_templates (
  id          BIGSERIAL PRIMARY KEY,
  merchant_id BIGINT NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
  name        TEXT NOT NULL,
  description TEXT,
  delivery_requirements JSONB,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS merchant_template_items (
  id          BIGSERIAL PRIMARY KEY,
  template_id BIGINT NOT NULL REFERENCES merchant_templates(id) ON DELETE CASCADE,
  name        TEXT NOT NULL,
  quantity    INT NOT NULL DEFAULT 1,
  price       NUMERIC(12,2) NOT NULL
);

CREATE TABLE IF NOT EXISTS merchant_contracts (
  id        BIGSERIAL PRIMARY KEY,
  order_id  BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  terms     JSONB NOT NULL,
  version   INT NOT NULL DEFAULT 1,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS merchant_invoices (
  id          BIGSERIAL PRIMARY KEY,
  merchant_id BIGINT NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
  order_id    BIGINT REFERENCES orders(id),
  title       TEXT NOT NULL,
  tax_no      TEXT,
  amount      NUMERIC(12,2) NOT NULL,
  status      TEXT NOT NULL CHECK (status IN ('pending','issued','rejected')),
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS merchant_approvals (
  id          BIGSERIAL PRIMARY KEY,
  demand_id   BIGINT NOT NULL REFERENCES demands(id) ON DELETE CASCADE,
  merchant_id BIGINT NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
  status      TEXT NOT NULL CHECK (status IN ('draft','pending','approved','rejected')),
  approver_id BIGINT REFERENCES users(id),
  comment     TEXT,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS merchant_assets (
  id          BIGSERIAL PRIMARY KEY,
  merchant_id BIGINT NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
  asset_type  TEXT NOT NULL CHECK (asset_type IN ('logo','brand','style','reference')),
  name        TEXT NOT NULL,
  status      TEXT NOT NULL CHECK (status IN ('active','archived')),
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_merchant_assets_merchant_type_status
  ON merchant_assets (merchant_id, asset_type, status);

CREATE TABLE IF NOT EXISTS merchant_asset_versions (
  id         BIGSERIAL PRIMARY KEY,
  asset_id   BIGINT NOT NULL REFERENCES merchant_assets(id) ON DELETE CASCADE,
  version    INT NOT NULL,
  payload    JSONB NOT NULL,
  created_by BIGINT NOT NULL REFERENCES users(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  UNIQUE (asset_id, version)
);

-- 8) IM 与通知
CREATE TABLE IF NOT EXISTS conversations (
  id        BIGSERIAL PRIMARY KEY,
  type      TEXT NOT NULL CHECK (type IN ('order','chat')),
  order_id  BIGINT REFERENCES orders(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS messages (
  id              BIGSERIAL PRIMARY KEY,
  conversation_id BIGINT NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
  sender_id       BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  content         TEXT,
  msg_type        TEXT NOT NULL CHECK (msg_type IN ('text','image','file')),
  sent_at         TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS notifications (
  id        BIGSERIAL PRIMARY KEY,
  user_id   BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  type      TEXT NOT NULL,
  title     TEXT,
  content   TEXT,
  read_at   TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 9) 管理与审计
CREATE TABLE IF NOT EXISTS audit_logs (
  id         BIGSERIAL PRIMARY KEY,
  admin_id   BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  action     TEXT NOT NULL,
  target_type TEXT,
  target_id  BIGINT,
  detail     JSONB,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS configs (
  id    BIGSERIAL PRIMARY KEY,
  key   TEXT UNIQUE NOT NULL,
  value JSONB NOT NULL
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_demands_city_status_start ON demands(city_id, status, schedule_start);
CREATE INDEX IF NOT EXISTS idx_orders_user_status_created ON orders(user_id, status, created_at);
CREATE INDEX IF NOT EXISTS idx_messages_conversation_sent ON messages(conversation_id, sent_at);
