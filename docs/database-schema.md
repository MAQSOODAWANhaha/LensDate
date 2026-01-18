# 数据库设计（PostgreSQL）

> 说明：本设计以 MVP 单体后端服务为前提，后续可按服务拆分。支付为直付模式，平台仅记录交易与服务费，不托管资金。

## 1. 设计原则
- 主键统一使用 `bigint` 自增（`bigserial`）。
- 统一字段：`created_at`、`updated_at`、`deleted_at`（软删除可选）。
- 枚举字段优先使用 `text + check` 或 `enum`（便于迁移）。
- 业务扩展字段使用 `jsonb`，避免频繁改表。

## 2. 核心实体关系（概览）
- 用户（users）↔ 角色（roles）↔ 用户档案（user_profiles）
- 摄影师（photographers）↔ 作品集（portfolios）↔ 作品项（portfolio_items）
- 需求单（demands）↔ 报价单（quotes）↔ 订单（orders）↔ 支付记录（payments）
- 订单（orders）↔ 交付（deliveries）↔ 评价（reviews）↔ 纠纷（disputes）
- 商户（merchants）↔ 门店（merchant_locations）↔ 套餐模板（merchant_templates）
- 商户素材库（merchant_assets）↔ 素材版本（merchant_asset_versions）

## 3. 基础与账号体系
### 3.1 users（用户）
- id PK
- username（唯一，可选）
- phone（唯一）
- email（可选）
- password_hash（若短信登录可为空）
- status（active/frozen/deleted）
- credit_score（信用分）
- created_at, updated_at

**索引**：`phone` 唯一索引

### 3.2 user_profiles（用户档案）
- user_id PK/FK
- nickname, avatar_url
- gender, birthday
- city_id
- bio

### 3.3 roles / user_roles（角色）
- roles: id, name（user/photographer/admin/merchant）
- user_roles: user_id, role_id, scope（可选）

### 3.4 sessions / verification_codes（登录）
- sessions: user_id, token, expired_at
- verification_codes: phone, code, expired_at

## 4. 摄影师/团队体系
### 4.1 photographers
- id PK
- user_id FK
- type（individual/team）
- status（pending/approved/rejected/frozen）
- city_id, service_area
- rating_avg, completed_orders

### 4.2 teams / team_members
- teams: id, owner_user_id, name, status
- team_members: team_id, user_id, role（admin/member）

### 4.3 portfolios / portfolio_items
- portfolios: id, photographer_id, title, status
- portfolio_items: id, portfolio_id, url, tags, cover_flag

## 5. 需求与报价
### 5.1 demands
- id PK
- user_id（发布者）
- type（写真/活动/商业/瑜伽馆宣传）
- city_id, location
- schedule_start, schedule_end
- budget_min, budget_max
- people_count, style_tags（jsonb）
- status（draft/open/closed）
- is_merchant（bool）
- merchant_id（商户需求时）

**索引**：`city_id`、`status`、`schedule_start`

### 5.2 demand_attachments
- id, demand_id, file_url, file_type

### 5.3 quotes / quote_items
- quotes: id, demand_id, photographer_id/team_id, total_price, status（pending/accepted/expired）
- quote_items: id, quote_id, name, price, quantity

**说明**：报价可版本化（可用 `quote_versions` 扩展表）

## 6. 订单与支付（直付）
### 6.1 orders
- id PK
- user_id, photographer_id/team_id
- demand_id, quote_id
- status（confirmed/paid/ongoing/completed/reviewed/cancelled）
- pay_type（deposit/full/phase）
- deposit_amount, total_amount
- service_fee（平台服务费）
- schedule_start, schedule_end
- cancelled_at（取消时间）

### 6.2 order_items
- id, order_id, name, price, quantity

### 6.3 payments
- id PK
- order_id
- payer_id（用户）
- payee_id（摄影师/商户）
- amount
- status（pending/success/failed）
- pay_channel（wx/alipay/bank）
- stage（deposit/mid/final，可选）
- paid_at
- proof_url（直付凭证）

### 6.4 refunds
- id PK
- order_id
- applicant_id
- amount
- status（pending/approved/rejected/paid）
- responsible_party（user/photographer/merchant）
- reason
- proof_url

## 7. 履约与交付
### 7.1 deliveries
- id PK
- order_id
- status（pending/submitted/accepted/rejected）
- submitted_at, accepted_at

### 7.2 delivery_items
- id, delivery_id, file_url, version, note

## 8. 评价与纠纷
### 8.1 reviews
- id PK
- order_id
- rater_id, ratee_id
- score（1~5）
- tags（jsonb）
- comment

### 8.2 disputes
- id PK
- order_id
- initiator_id
- status（submitted/handling/closed）
- reason, resolution

### 8.3 dispute_evidence
- id, dispute_id, file_url, note

## 9. 商户（瑜伽馆）能力
### 9.1 merchants
- id PK
- name, logo_url, brand_color
- contact_user_id
- status（pending/approved/frozen）

### 9.2 merchant_locations
- id, merchant_id, name, address, city_id

### 9.3 merchant_users
- id, merchant_id, user_id, role（requester/approver/finance）

### 9.4 merchant_templates
- id, merchant_id, name, description
- delivery_requirements（jsonb）

### 9.5 merchant_template_items
- id, template_id, name, quantity, price

### 9.6 merchant_contracts
- id, order_id, terms（jsonb）, version

### 9.7 merchant_invoices
- id, merchant_id, order_id, title, tax_no, amount, status

### 9.8 merchant_approvals
- id, demand_id, merchant_id
- status（draft/pending/approved/rejected）
- approver_id, comment

### 9.9 merchant_assets
- id, merchant_id, asset_type, name, status
- created_at, updated_at

### 9.10 merchant_asset_versions
- id, asset_id, version, payload（jsonb）
- created_by, created_at

## 10. IM 与通知
### 10.1 conversations / messages
- conversations: id, type（order/chat）, order_id
- messages: id, conversation_id, sender_id, content, msg_type, sent_at

### 10.2 notifications
- id, user_id, type, title, content, read_at

## 11. 管理与审计
### 11.1 audit_logs
- id, admin_id, action, target_type, target_id, detail_jsonb

### 11.2 configs
- id, key, value_jsonb

## 12. 枚举字段建议
- users.status: active/frozen/deleted
- photographers.status: pending/approved/rejected/frozen
- demands.status: draft/open/closed
- quotes.status: pending/accepted/expired
- orders.status: confirmed/paid/ongoing/completed/reviewed/cancelled
- payments.status: pending/success/failed
- refunds.status: pending/approved/rejected/paid
- deliveries.status: pending/submitted/accepted/rejected
- disputes.status: submitted/handling/closed

## 13. 索引与性能建议
- 高频查询：demands(city_id, status, schedule_start)
- 搜索：摄影师/需求的标签索引（可同步至 Meilisearch）
- 消息：messages(conversation_id, sent_at)
- 订单：orders(user_id, status, created_at)

---
如需，我可以继续输出：
- 关键表的 SQL DDL（建表语句）
- ER 图（可视化）
- 与 API 的字段对齐清单
