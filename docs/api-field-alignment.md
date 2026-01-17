# API 字段对齐清单（关键接口）

> 说明：本清单用于产品/后端/前端（移动端/管理后台）对齐字段含义与来源，具体路由可按工程规范调整。

## 1. 认证与用户
### 1.1 登录/注册
- POST /api/v1/auth/login
  - req: phone, code
  - res: token, user{id, phone, status}, roles[]
  - tables: users, sessions, user_roles

### 1.2 用户档案
- GET /api/v1/users/me
  - res: users + user_profiles
  - tables: users, user_profiles
- PUT /api/v1/users/me
  - req: nickname, avatar_url, gender, city_id, bio
  - tables: user_profiles

## 2. 摄影师/团队
### 2.1 申请入驻
- POST /api/v1/photographers
  - req: type, city_id, service_area
  - tables: photographers

### 2.2 摄影师作品集
- POST /api/v1/portfolios
  - req: title
  - tables: portfolios
- POST /api/v1/portfolios/{id}/items
  - req: url, tags, cover_flag
  - tables: portfolio_items

## 3. 需求
### 3.1 发布需求
- POST /api/v1/demands
  - req: type, city_id, location, schedule_start, schedule_end,
         budget_min, budget_max, people_count, style_tags, attachments[],
         is_merchant, merchant_id
  - tables: demands, demand_attachments

### 3.2 需求列表
- GET /api/v1/demands?city_id&status&schedule_start
  - res: demands[]
  - tables: demands

### 3.3 需求关联商户素材
- GET /api/v1/demands/{id}/merchant-assets
  - res: assets（latest_version/payload）
  - tables: merchant_assets, merchant_asset_versions

## 4. 报价
### 4.1 提交报价
- POST /api/v1/quotes
  - req: demand_id, photographer_id/team_id, total_price, items[]
  - tables: quotes, quote_items

### 4.2 接受报价
- POST /api/v1/quotes/{id}/accept
  - res: order_id
  - tables: quotes, orders

## 5. 订单与支付
### 5.1 创建订单（接受报价后自动生成）
- POST /api/v1/orders
  - req: quote_id
  - tables: orders

### 5.2 支付（直付）
- POST /api/v1/payments
  - req: order_id, amount, pay_channel, proof_url
  - tables: payments, orders

### 5.3 退款申请
- POST /api/v1/refunds
  - req: order_id, amount, reason, proof_url
  - tables: refunds

## 6. 履约与交付
### 6.1 提交交付
- POST /api/v1/deliveries
  - req: order_id, items[file_url, version, note]
  - tables: deliveries, delivery_items

### 6.2 验收
- POST /api/v1/deliveries/{id}/accept
  - tables: deliveries, orders

## 7. 评价与纠纷
### 7.1 提交评价
- POST /api/v1/reviews
  - req: order_id, score, tags, comment
  - tables: reviews, orders

### 7.2 提交纠纷
- POST /api/v1/disputes
  - req: order_id, reason, evidence[]
  - tables: disputes, dispute_evidence

## 8. 商户（瑜伽馆）
### 8.1 商户档案
- POST /api/v1/merchants
  - req: name, logo_url, brand_color, contact_user_id
  - tables: merchants

### 8.2 商户需求模板
- POST /api/v1/merchant-templates
  - req: merchant_id, name, description, delivery_requirements, items[]
  - tables: merchant_templates, merchant_template_items

### 8.3 商户审批流
- POST /api/v1/merchant-approvals
  - req: demand_id, merchant_id, status, comment
  - tables: merchant_approvals

### 8.4 合同与发票
- POST /api/v1/merchant-contracts
  - req: order_id, terms, version
  - tables: merchant_contracts
- POST /api/v1/merchant-invoices
  - req: merchant_id, order_id, title, tax_no, amount
  - tables: merchant_invoices

### 8.5 商户素材库
- POST /api/v1/merchants/{id}/assets
  - req: name, asset_type, status, payload
  - tables: merchant_assets, merchant_asset_versions
- POST /api/v1/merchants/assets/{asset_id}/versions
  - req: payload
  - tables: merchant_asset_versions

## 9. IM 与通知
### 9.1 发送消息
- POST /api/v1/messages
  - req: conversation_id, content, msg_type
  - tables: messages

### 9.2 通知
- GET /api/v1/notifications
  - tables: notifications

## 10. 管理后台
### 10.1 审核
- POST /api/v1/admin/audits
  - req: action, target_type, target_id, detail
  - tables: audit_logs

### 10.2 配置
- PUT /api/v1/admin/configs/{key}
  - req: value
  - tables: configs
