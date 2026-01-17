# 系统架构与技术方案

> 目标：给出可落地的技术选型、服务拆分与数据设计建议，满足“个人用户约拍 + 瑜伽馆商户”两类需求场景，且符合“直付、不托管”前提。

## 1. 架构原则
- **MVP 优先**：首期采用“单体后端服务 + 模块化拆分”，后续按模块拆为独立服务。
- **稳定可演进**：核心数据与流程稳定后再做服务拆分与读写分离。
- **直付场景对齐**：支付仅记录交易与服务费，不托管资金。
- **角色与权限明确**：用户/摄影师/管理员/商户分层权限。

## 2. 技术选型（建议）
### 2.1 移动端
- **框架**：Flutter（Dart）
- **状态管理**：Riverpod
- **网络**：Dio
- **路由**：go_router
- **本地存储**：Hive/SharedPreferences
- **推送**：FCM/APNs

### 2.2 管理后台
- **框架**：React + TypeScript
- **构建**：Vite
- **组件库**：Ant Design
- **权限**：RBAC（路由级 + 按钮级）

### 2.3 后端服务（Rust + Axum）
- **框架**：Axum（Tokio）
- **数据库**：PostgreSQL（确认采用）
- **ORM**：SeaORM（驱动 PostgreSQL）
- **缓存**：Redis（确认采用）
- **错误处理**：thiserror + anyhow
- **迁移**：SeaORM 迁移（Rust DSL）+ 独立迁移入口（bin/migrate.rs）
- **搜索**：Meilisearch（MVP） / Elasticsearch（规模化）
- **对象存储**：S3 兼容（MinIO/云存储）
- **消息**：Redis Streams（MVP）→ RabbitMQ/Kafka（扩展）
- **观测**：OpenTelemetry + Prometheus + Grafana + Loki

### 2.4 后端代码分层（最佳实践）
- **routes**：仅做路由注册，禁止写业务逻辑。
- **handlers**：控制层，负责参数解析、鉴权前置校验、调用服务层。
- **services**：业务层，承载状态机、事务、核心业务规则。
- **repositories**：数据访问层，封装 SeaORM 查询与持久化。
- **dto**：请求/响应模型与校验规则。
- **middleware**：中间件与鉴权（AuthUser 等）。
- **errors**：领域错误定义（thiserror）。
- **utils/config**：通用工具与配置加载。

### 2.5 错误处理方案（thiserror + anyhow）
- **领域错误（thiserror）**：在 `errors/` 定义 `DomainError`，用于业务规则/状态机错误。
- **基础设施错误（anyhow）**：repo/第三方调用使用 `anyhow::Result`，配合 `anyhow::Context` 补充上下文。
- **统一响应（ApiError）**：handler 负责把 `DomainError`/`anyhow::Error` 映射为 `ApiError`，并输出统一错误码。
- **日志与兜底**：`ApiError` 对内部错误记录日志，未识别错误统一转 `internal`。
- **边界约束**：routes 不直接返回 `anyhow`；service 不直接返回 `ApiError`。

## 3. 服务拆分方案
### 3.1 MVP 阶段（单体后端服务模块化）
- 一个 Rust 单体后端服务，按模块划分：auth、user、photographer、demand、quote、order、payment、message、review、dispute、admin。

### 3.2 扩展阶段（微服务拆分）
| 服务 | 主要职责 | 关键数据 | 依赖 |
|---|---|---|---|
| api_gateway | 统一鉴权、限流、路由 | 无 | 各业务服务 |
| auth_service | 登录/认证/权限 | users、roles | user_service |
| user_service | 用户档案/偏好 | users、profiles | auth_service |
| photographer_service | 摄影师/团队 | photographer_profiles、teams、portfolios | user_service |
| demand_service | 需求管理 | demands、demand_tags | user_service |
| quote_service | 报价与方案 | quotes、quote_items | demand_service |
| order_service | 订单流转 | orders、order_items | quote_service |
| payment_service | 直付记录/服务费 | payments、refunds | order_service |
| messaging_service | IM/通知 | conversations、messages | user_service |
| review_service | 评价与信用 | reviews、scores | order_service |
| dispute_service | 纠纷与售后 | disputes、evidence | order_service |
| admin_service | 审核/运营/审计 | audit_logs、configs | 各服务 |

> 建议路径：MVP 先单体后端服务，核心流程稳定后拆分 payment/message/admin。

## 4. 数据库方案
### 4.1 关系型数据库（PostgreSQL）
**核心表（MVP）**
- users、roles、user_profiles
- photographers、teams、team_members
- portfolios、portfolio_items
- demands、demand_tags、demand_attachments
- quotes、quote_items
- orders、order_items
- payments、refunds
- deliveries、delivery_items
- reviews、review_tags
- disputes、dispute_evidence
- conversations、messages
- notifications
- audit_logs

**商户（瑜伽馆）相关表**
- merchants（商户档案）
- merchant_locations（门店）
- merchant_users（商户账号/审批角色）
- merchant_templates（套餐模板）
- merchant_invoices（发票/结算）
- merchant_contracts（合同要素与版本）

### 4.2 缓存与搜索
- Redis：会话、验证码、热点数据、分布式锁、计数器
- Meilisearch：摄影师检索、需求检索

### 4.3 对象存储
- 作品集、需求图片、聊天附件、交付文件存储在 S3 兼容存储

## 5. 支付与退款（直付模式）
- **支付流程**：用户直接支付摄影师/商户 → 平台记录支付结果 → 计算服务费（如有）。
- **退款流程**：责任方原路退款 → 平台记录凭证 → 信用扣分/降权。
- **对账**：平台仅做订单与支付记录对账，不做资金托管。

## 6. 搜索与推荐
- **搜索**：地点、预算、风格、档期、评分
- **推荐因子**：履约率、响应时长、评分、近期成交量

## 7. 安全与合规
- OAuth2/JWT 鉴权
- 审计日志与操作留痕
- 内容审核（涉黄/侵权）与举报处理
- 未成年人保护提示与协议

## 8. 部署与运维（建议）
- Docker 容器化，K8s 可选
- CI/CD：GitHub Actions
- 监控与告警：Prometheus + Grafana

## 8.1 数据库迁移执行
统一使用独立迁移入口：
```bash
cargo run --bin migrate
```
要求：已设置 `DATABASE_URL` 环境变量。

## 9. 与项目路径关系
- 目录结构见 `docs/project-structure.md`
- 术语与目录命名规范以 `docs/project-structure.md` 为准
- 后端代码按 `backend/src` 模块化，迁移入口使用独立二进制
- 移动端功能按 `apps/mobile/lib/features/*` 对齐需求模块
