# API 接口文档（v1）

> 说明：本接口文档基于现有需求与数据库设计，适用于移动端与管理后台。支付为直付模式，平台仅记录交易与服务费。

## 1. 通用约定
### 1.1 基础信息
- Base URL：`/api/v1`
- 数据格式：`application/json`
- 认证：`Authorization: Bearer <token>`
- 时间格式：`ISO 8601`

### 1.2 统一返回结构
**成功**
```json
{ "code": 0, "message": "ok", "data": { } }
```

**失败**
```json
{ "code": 1001, "message": "invalid_param", "data": null }
```

### 1.3 常见错误码
- 1001 参数错误
- 1002 未认证
- 1003 权限不足
- 1004 资源不存在
- 1005 状态不允许
- 1006 频率限制
- 1100 业务校验失败

### 1.4 分页
- 请求：`page`、`page_size`
- 返回：`items`、`total`、`page`、`page_size`

### 1.5 幂等与并发
- 关键写接口支持 `Idempotency-Key` 头
- 订单状态变更需校验当前状态

---

## 2. 认证与用户
### 2.1 发送验证码
- POST `/auth/code`
- req: `{ "phone": "" }`
- res: `{ "code":0, "data": { "expired_at": "" } }`

### 2.2 登录
- POST `/auth/login`
- req（三选一）：
  - 手机号 + 验证码：`{ "phone": "", "code": "" }`
  - 手机号 + 密码：`{ "phone": "", "password": "" }`
  - 用户名 + 密码：`{ "username": "", "password": "" }`
- res: `{ "token": "", "user": { "id": 1, "phone": "", "status": "active" }, "roles": ["user"] }`
- 说明：密码登录需已设置 `password_hash`（管理员账号建议通过初始化脚本或数据库预置）。

### 2.3 获取我的信息
- GET `/users/me`
- res: `{ "id":1, "phone":"", "profile": { "nickname":"" } }`

### 2.4 更新我的信息
- PUT `/users/me`
- req: `{ "nickname":"", "avatar_url":"", "gender":"male", "city_id":1, "bio":"" }`

---

## 3. 摄影师/团队
### 3.1 申请摄影师
- POST `/photographers`
- req: `{ "type":"individual", "city_id":1, "service_area":"" }`

### 3.2 摄影师列表（供给展示）
- GET `/photographers?keyword=&city_id=&type=&status=&page=&page_size=`
- 默认仅返回 `approved` 状态；keyword 支持手机号/昵称模糊匹配。

### 3.3 我的摄影师档案
- GET `/photographers/me`

### 3.4 摄影师详情
- GET `/photographers/{id}`

### 3.5 摄影师订单
- GET `/photographers/me/orders?status=&page=&page_size=`
- GET `/photographers/me/orders/{id}`

### 3.6 作品集
- POST `/portfolios`
- req: `{ "photographer_id":1, "title":"" }`

- POST `/portfolios/{id}/items`
- req: `{ "url":"", "tags":[""], "cover_flag":true }`

- GET `/portfolios?photographer_id=`
- GET `/portfolios/{id}/items`

### 3.7 团队管理
- POST `/teams`
- req: `{ "name":"" }`
- res: `{ "id":1, "name":"", "status":"active", "role":"owner" }`

- GET `/teams`

- PUT `/teams/{id}`
- req: `{ "name":"" }`

- POST `/teams/{id}/members`
- req: `{ "user_id":1, "role":"member" }`
- role 可选：`admin` / `member`（默认 member）

- GET `/teams/{id}/members`
- DELETE `/teams/{id}/members/{user_id}`

---

## 4. 需求
### 4.1 发布需求
- POST `/demands`
- req:
```json
{
  "type":"", "city_id":1, "location":"",
  "schedule_start":"", "schedule_end":"",
  "budget_min":100, "budget_max":500,
  "people_count":2, "style_tags":[""],
  "attachments":[{"file_url":"","file_type":"image"}],
  "is_merchant":false, "merchant_id":null
}
```

### 4.2 需求列表
- GET `/demands?city_id=&type=&status=&schedule_start=&schedule_end=&min_budget=&max_budget=&style_tag=&is_merchant=&page=&page_size=&mine=`
- mine=true 时仅返回本人需求
- is_merchant 可选：true/false
- sort 可选：`time_desc` / `time_asc` / `budget_desc` / `budget_asc`

### 4.3 需求详情
- GET `/demands/{id}`

### 4.4 关闭需求
- POST `/demands/{id}/close`

### 4.5 商户素材库（需求关联）
- GET `/demands/{id}/merchant-assets?asset_type=&page=&page_size=`
- 仅商户需求可访问；asset_type ∈ {logo,brand,style,reference}

---

## 5. 报价
### 5.1 提交报价
- POST `/quotes`
- req:
```json
{
  "demand_id":1,
  "photographer_id":1,
  "team_id":null,
  "total_price":800,
  "items":[{"name":"拍摄","price":500,"quantity":1}],
  "note":"首次报价说明"
}
```
 - 说明：默认有效期 7 天（可通过配置扩展）。

### 5.2 报价列表（需求）
- GET `/quotes?demand_id=&page=&page_size=`

### 5.3 接受报价
- POST `/quotes/{id}/accept`
- res: `{ "order_id": 1001 }`

### 5.4 我的报价（摄影师）
- GET `/quotes/mine?status=&demand_id=&page=&page_size=`
- GET `/quotes/{id}`
- POST `/quotes/{id}/withdraw`
- 说明：仅待处理（pending）可撤回，撤回后状态为 expired。
- 响应附加：`order_id`、`order_status`（若已生成订单）、`version`、`expires_at`

### 5.5 修改报价
- PUT `/quotes/{id}`
- req:
```json
{
  "total_price":900,
  "items":[{"name":"拍摄","price":600,"quantity":1}],
  "note":"调整拍摄清单"
}
```
- 说明：仅 pending 且本人可修改；修改会生成新的版本并刷新有效期。

### 5.6 报价版本
- GET `/quotes/{id}/versions`
- res:
```json
[
  {
    "id":1,
    "version":1,
    "total_price":800,
    "items":[{"name":"拍摄","price":500,"quantity":1}],
    "note":"首次报价说明",
    "created_by":1,
    "created_at":""
  }
]
```

---

## 6. 订单与支付（直付）
### 6.1 订单详情
- GET `/orders/{id}`

### 6.2 订单列表
- GET `/orders?status=&page=&page_size=&keyword=&sort=&min_amount=&max_amount=&start_time=&end_time=`
- keyword：订单 ID（精确）
- sort：`amount_desc` / `amount_asc` / `time_desc` / `time_asc`（默认创建时间倒序）
- min_amount/max_amount：金额区间筛选
- start_time/end_time：下单时间区间（RFC3339）

### 6.3 支付记录（直付凭证）
- POST `/payments`
- req: `{ "order_id":1, "amount":800, "pay_channel":"wx", "proof_url":"", "stage":"deposit" }`
- stage 可选：`deposit` / `mid` / `final`（分期订单必填）

### 6.4 退款申请
- POST `/refunds`
- req: `{ "order_id":1, "amount":200, "reason":"", "proof_url":"", "responsible_party":"user" }`
- responsible_party 可选：`user` / `photographer` / `merchant`

### 6.5 取消与退款预览
- GET `/orders/{id}/refund-preview`
- POST `/orders/{id}/cancel`

---

## 7. 履约与交付
### 7.1 提交交付
- POST `/deliveries`
- req:
```json
{
  "order_id":1,
  "items":[{"file_url":"","version":"v1","note":""}]
}
```

### 7.2 验收交付
- POST `/deliveries/{id}/accept`

### 7.3 交付列表
- GET `/deliveries?order_id=`

---

## 8. 评价与纠纷
### 8.1 评价
- POST `/reviews`
- req: `{ "order_id":1, "score":5, "tags":["专业"], "comment":"" }`

### 8.2 纠纷提交
- POST `/disputes`
- req: `{ "order_id":1, "reason":"", "evidence":[{"file_url":"","note":""}] }`

---

## 9. 商户（瑜伽馆）能力
### 9.1 商户档案
- POST `/merchants`
- req: `{ "name":"", "logo_url":"", "brand_color":"", "contact_user_id":1 }`

- GET `/merchants/mine`

### 9.2 商户套餐模板
- POST `/merchant-templates`
- req:
```json
{
  "merchant_id":1,
  "name":"月度课程拍摄",
  "description":"",
  "delivery_requirements": {"count":50},
  "items":[{"name":"场次","quantity":4,"price":2000}]
}
```

- GET `/merchant-templates?merchant_id=&page=&page_size=`

### 9.3 商户审批流
- POST `/merchant-approvals`
- req: `{ "demand_id":1, "merchant_id":1, "status":"pending", "comment":"" }`

- GET `/merchant-approvals?merchant_id=&status=&page=&page_size=`

### 9.4 合同与发票
- POST `/merchant-contracts`
- req: `{ "order_id":1, "terms":{}, "version":1 }`

- GET `/merchant-contracts?merchant_id=&page=&page_size=`

- POST `/merchant-invoices`
- GET `/merchant-invoices?merchant_id=&status=&page=&page_size=`

### 9.5 商户订单
- GET `/merchants/orders?merchant_id=&status=&page=&page_size=`
- GET `/merchants/orders/{id}`

### 9.6 商户对账导出
- GET `/merchants/reports/orders?merchant_id=&status=&start_date=YYYY-MM-DD&end_date=YYYY-MM-DD&limit=500&format=csv`
- res: `{ "format":"csv", "generated_at":"", "total":0, "items":[], "csv":"" }`

### 9.7 商户门店
- POST `/merchants/{id}/locations`
- req: `{ "name":"", "address":"", "city_id":1 }`

- GET `/merchants/{id}/locations`

- PUT `/merchants/{id}/locations/{location_id}`
- req: `{ "name":"", "address":"", "city_id":1 }`

- DELETE `/merchants/{id}/locations/{location_id}`

### 9.8 商户成员
- POST `/merchants/{id}/members`
- req: `{ "user_id":1, "role":"requester" }`
- role 可选：`requester` / `approver` / `finance`（默认 requester）

- GET `/merchants/{id}/members`
- DELETE `/merchants/{id}/members/{user_id}`

### 9.9 商户素材库与风格库
- POST `/merchants/{id}/assets`
- req: `{ "name":"", "asset_type":"logo", "status":"active", "payload":{} }`
- asset_type ∈ {logo, brand, style, reference}；status ∈ {active, archived}

- GET `/merchants/{id}/assets?asset_type=&status=&page=&page_size=`

- GET `/merchants/assets/{asset_id}/versions?page=&page_size=`
- POST `/merchants/assets/{asset_id}/versions`
- req: `{ "payload":{} }`

---

## 10. 消息与通知
### 10.1 会话
- POST `/conversations`
- req: `{ "type":"order", "order_id":1 }`
- GET `/conversations?page=&page_size=&order_id=`

### 10.2 发送消息
- POST `/messages`
- req: `{ "conversation_id":1, "content":"", "msg_type":"text" }`

### 10.3 通知列表
- GET `/notifications?page=&page_size=&read_status=`
- read_status 可选：`read` / `unread`

### 10.4 通知详情
- GET `/notifications/{id}`

### 10.5 标记已读
- POST `/notifications/{id}/read`

### 10.6 全部标记已读
- POST `/notifications/read-all`

### 10.7 未读汇总
- GET `/notifications/summary`
- res: `{ "unread_count": 0 }`

---

## 11. 文件上传
### 11.1 上传文件
- POST `/uploads`
- form-data: `file`（文件）
- res: `{ "file_name":"", "file_url":"/uploads/xxx.jpg", "size":12345 }`
- 约束：允许 png/jpg/jpeg/gif/webp/pdf，单文件默认 10MB，可通过 `UPLOAD_MAX_BYTES` 调整

### 11.2 下载/预览
- GET `/uploads/{name}`

---

## 12. 管理后台
### 12.1 审核
- POST `/admin/audits`
- req: `{ "action":"approve", "target_type":"photographer", "target_id":1, "detail":{} }`
- GET `/admin/audits?action=&page=&page_size=`

### 12.2 配置
- PUT `/admin/configs/{key}`
- req: `{ "value": {} }`
- GET `/admin/configs/{key}`
 - 常用 key：`order_auto_cancel_hours`、`refund_penalty_rate`、`dispute_priority`、`demand_tags`、`photographer_tags`、`recommend_slots`、`activity_banners`

### 12.3 指标与趋势
- GET `/admin/metrics?days=7`
- res: `{ "period_days":7, "users_total":0, "orders_total":0, "orders_period":0, "orders_today":0, "disputes_open":0, "disputes_period":0, "pending_photographers":0, "pending_merchant_approvals":0, "revenue_today":0, "revenue_period":0 }`

- GET `/admin/metrics/trends?days=7`
- res: `{ "days":7, "items":[{"date":"2026-01-16","orders":0,"disputes":0,"revenue":0}] }`

### 12.4 报表导出
- GET `/admin/reports/orders?start_date=YYYY-MM-DD&end_date=YYYY-MM-DD&status=&limit=500&format=csv`
- res: `{ "format":"csv", "generated_at":"", "total":0, "items":[], "csv":"" }`

### 12.5 用户列表
- GET `/admin/users?keyword=&role=&status=&page=&page_size=`

### 12.6 订单管理
- GET `/admin/orders?status=&page=&page_size=`
- GET `/admin/orders/{id}`
- POST `/admin/orders/{id}/freeze`

### 12.7 纠纷管理
- GET `/admin/disputes?status=&page=&page_size=`
- GET `/admin/disputes/{id}`
- POST `/admin/disputes/{id}/resolve`
- req: `{ "resolution":"", "status":"closed" }`

### 12.8 商户审批与模板
- GET `/admin/merchant-approvals?status=&page=&page_size=`
- POST `/admin/merchant-approvals/{id}/review`
- req: `{ "status":"approved|rejected", "comment":"" }`
- GET `/admin/merchant-templates?page=&page_size=`

### 12.9 摄影师资质审核
- POST `/admin/photographers/{id}/review`
- req: `{ "status":"approved|rejected", "comment":"" }`

### 12.10 内容审核（作品集）
- GET `/admin/portfolios?status=&photographer_id=&page=&page_size=`
- POST `/admin/portfolios/{id}/review`
- req: `{ "status":"approved|rejected", "comment":"" }`

---

## 13. 字段校验规则（关键接口）
> 仅列出关键字段的强校验规则；其他字段按“非空/类型/长度”处理。所有时间字段必须为 ISO 8601。

### 13.1 认证与用户
- `/auth/code`：phone 必填，正则 `^1\d{10}$`（中国大陆手机号），60 秒内限频。
- `/auth/login`：code 6 位数字，5 分钟内有效。
- `/users/me`：nickname 2~20 字，avatar_url 必须为 URL。

### 13.2 摄影师/团队
- `/photographers`：type ∈ {individual, team}；city_id 必填；service_area ≤ 200 字。
- `/portfolios`：title 1~50 字。
- `/portfolios/{id}/items`：url 必填；tags ≤ 10 个。
- `/teams`：name 2~50 字。
- `/teams/{id}/members`：role ∈ {admin, member}。

### 13.3 需求
- `/demands`：type 必填；city_id 必填；schedule_start < schedule_end；
  budget_min ≥ 0，budget_max ≥ budget_min；people_count 1~200；
  is_merchant=true 时 merchant_id 必填。
- `/demands/{id}/close`：仅发布者可关闭，状态需为 open。

### 13.4 报价
- `/quotes`：demand_id 必填；total_price > 0；items 1~50 条；
  photographer_id 与 team_id 二选一。

### 13.5 订单与支付
- `/payments`：amount > 0；pay_channel ∈ {wx, alipay, bank}；
  proof_url 必填（直付凭证）；仅订单所有人可提交。
- `/refunds`：amount > 0 且 ≤ 已支付金额；reason 1~200 字；
  proof_url 可选但建议上传；仅订单双方可提交。

### 13.6 交付
- `/deliveries`：items 1~200；每个 item 的 file_url 必填；version 1~20 字。
- `/deliveries/{id}/accept`：仅订单用户可验收。

### 13.7 评价与纠纷
- `/reviews`：score 1~5；tags ≤ 10；comment ≤ 500 字。
- `/disputes`：reason 1~200 字；evidence ≤ 10 条。

### 13.8 商户能力
- `/merchants`：name 2~50 字；contact_user_id 必填。
- `/merchant-templates`：items 1~50；delivery_requirements 为 JSON。
- `/merchant-approvals`：status ∈ {draft,pending,approved,rejected}。
- `/merchant-invoices`：amount > 0；title 2~100 字；tax_no 格式校验（可配置）。
- `/merchants/{id}/locations`：name 2~50 字；address ≤ 200 字。
- `/merchants/{id}/members`：role ∈ {requester, approver, finance}；不可添加联系人。
- `/merchants/{id}/assets`：name 2~100 字；asset_type ∈ {logo,brand,style,reference}；
  status ∈ {active,archived}；payload 为 JSON（可选）。
- `/merchants/assets/{asset_id}/versions`：payload 为 JSON；版本自动递增。

---

## 13. 状态机约束（核心流程）

### 13.1 需求状态（demands.status）
`draft → open → closed`
- 仅发布者可从 draft/open 关闭为 closed。
- closed 不可再接受报价。

### 13.2 报价状态（quotes.status）
`pending → accepted | expired`
- pending 仅可被需求发布者接受。
- accepted 后不可修改价格与条目。

### 13.3 订单状态（orders.status）
`confirmed → paid → ongoing → completed → reviewed`
`confirmed/paid/ongoing` 可进入 `cancelled`（需符合退款规则）。
- 仅在 paid 后才可创建 deliveries。
- completed 后才能评价与纠纷。
- reviewed 为终态。

### 13.4 交付状态（deliveries.status）
`pending → submitted → accepted | rejected`
- rejected 可重新提交新版本。
- accepted 后订单进入 completed。

### 13.5 退款状态（refunds.status）
`pending → approved | rejected → paid`
- approved 后必须上传凭证后进入 paid。
- 直付场景由责任方执行退款。

### 13.6 纠纷状态（disputes.status）
`submitted → handling → closed`
- closed 需要记录处理结果与责任判定。

---

## 14. 权限矩阵（接口级）

### 14.1 角色说明
- 用户（User）
- 摄影师/团队（Photographer/Team）
- 商户（Merchant）
- 管理员（Admin）

### 14.2 权限矩阵（摘要）
| 接口组 | 用户 | 摄影师/团队 | 商户 | 管理员 |
|---|---|---|---|---|
| 认证与用户 | ✅ | ✅ | ✅ | ✅ |
| 需求发布/管理 | ✅（本人） | ❌ | ✅（商户需求） | ✅（全量） |
| 报价与方案 | ❌ | ✅（本人） | ❌ | ✅（全量） |
| 订单与支付 | ✅（本人） | ✅（本人） | ✅（商户需求） | ✅（全量） |
| 履约与交付 | ✅（验收） | ✅（提交） | ✅（验收） | ✅ |
| 评价与纠纷 | ✅（本人） | ✅（本人） | ✅（本人） | ✅ |
| 商户能力 | ❌ | ❌ | ✅ | ✅ |
| 管理后台 | ❌ | ❌ | ❌ | ✅ |

### 14.3 数据访问约束
- 所有“本人”资源必须通过 user_id 校验。
- 管理员仅可操作具备审核权限的对象。
- 商户接口需校验 merchant_id 与账号归属。

---

## 15. 字段规格明细（关键对象）
> 说明：按“对象字段”维度给出类型、是否必填、长度/范围、默认值与说明。

### 15.1 User（用户）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| id | bigint | 是 | >0 | - | 主键 |
| phone | string | 是 | 长度 11，`^1\d{10}$` | - | 手机号 |
| email | string | 否 | ≤255 | null | 邮箱 |
| status | string | 是 | active/frozen/deleted | active | 状态 |
| credit_score | int | 是 | 0~100 | 100 | 信用分 |
| created_at | datetime | 是 | ISO 8601 | now | 创建时间 |

### 15.2 UserProfile（用户档案）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| nickname | string | 否 | 2~20 | null | 昵称 |
| avatar_url | string | 否 | URL | null | 头像 |
| gender | string | 否 | male/female/unknown | unknown | 性别 |
| birthday | date | 否 | yyyy-mm-dd | null | 生日 |
| city_id | bigint | 否 | >0 | null | 城市 |
| bio | string | 否 | ≤500 | null | 简介 |

### 15.3 Photographer（摄影师）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| id | bigint | 是 | >0 | - | 主键 |
| user_id | bigint | 是 | >0 | - | 关联用户 |
| type | string | 是 | individual/team | individual | 类型 |
| status | string | 是 | pending/approved/rejected/frozen | pending | 状态 |
| city_id | bigint | 否 | >0 | null | 城市 |
| service_area | string | 否 | ≤200 | null | 服务范围 |
| rating_avg | number | 是 | 0.00~5.00 | 0 | 评分 |
| completed_orders | int | 是 | ≥0 | 0 | 完成单量 |

### 15.4 Demand（需求单）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| type | string | 是 | ≤50 | - | 类型 |
| city_id | bigint | 是 | >0 | - | 城市 |
| location | string | 否 | ≤200 | null | 地点 |
| schedule_start | datetime | 是 | < schedule_end | - | 开始时间 |
| schedule_end | datetime | 是 | > schedule_start | - | 结束时间 |
| budget_min | number | 否 | ≥0 | null | 预算下限 |
| budget_max | number | 否 | ≥budget_min | null | 预算上限 |
| people_count | int | 否 | 1~200 | null | 人数 |
| style_tags | array | 否 | ≤20 | [] | 风格标签 |
| status | string | 是 | draft/open/closed | open | 状态 |
| is_merchant | bool | 是 | true/false | false | 商户需求 |
| merchant_id | bigint | 条件 | is_merchant=true 必填 | null | 商户ID |

### 15.5 Quote（报价单）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| demand_id | bigint | 是 | >0 | - | 需求ID |
| photographer_id | bigint | 条件 | 与 team_id 二选一 | null | 摄影师ID |
| team_id | bigint | 条件 | 与 photographer_id 二选一 | null | 团队ID |
| total_price | number | 是 | >0 | - | 总价 |
| status | string | 是 | pending/accepted/expired | pending | 状态 |
| version | int | 是 | ≥1 | 1 | 报价版本 |
| expires_at | datetime | 否 | - | null | 有效期截止 |
| items | array | 是 | 1~50 | - | 报价明细 |

### 15.5.1 QuoteVersion（报价版本）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| quote_id | bigint | 是 | >0 | - | 报价ID |
| version | int | 是 | ≥1 | - | 版本号 |
| total_price | number | 是 | >0 | - | 该版本总价 |
| items | json | 是 | - | - | 该版本明细快照 |
| note | string | 否 | ≤500 | null | 版本说明 |
| created_by | bigint | 是 | >0 | - | 创建人 |
| created_at | datetime | 是 | - | - | 创建时间 |

### 15.6 Order（订单）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| status | string | 是 | confirmed/paid/ongoing/completed/reviewed/cancelled | confirmed | 状态 |
| pay_type | string | 是 | deposit/full/phase | deposit | 支付类型 |
| deposit_amount | number | 否 | ≥0 | 0 | 定金 |
| total_amount | number | 是 | >0 | - | 订单总额 |
| service_fee | number | 是 | ≥0 | 0 | 平台服务费 |
| schedule_start | datetime | 是 | < schedule_end | - | 开始时间 |
| schedule_end | datetime | 是 | > schedule_start | - | 结束时间 |
| cancelled_at | datetime | 否 | - | null | 取消时间 |

### 15.7 Payment（直付记录）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| order_id | bigint | 是 | >0 | - | 订单ID |
| payer_id | bigint | 是 | >0 | - | 付款人 |
| payee_id | bigint | 是 | >0 | - | 收款人 |
| amount | number | 是 | >0 | - | 金额 |
| pay_channel | string | 是 | wx/alipay/bank | - | 渠道 |
| proof_url | string | 是 | URL | - | 凭证 |
| status | string | 是 | pending/success/failed | pending | 状态 |
| stage | string | 否 | deposit/mid/final | null | 分期阶段 |
| paid_at | datetime | 否 | - | null | 支付时间 |

### 15.8 Refund（退款）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| order_id | bigint | 是 | >0 | - | 订单ID |
| applicant_id | bigint | 是 | >0 | - | 申请人 |
| amount | number | 是 | >0 | - | 退款金额 |
| status | string | 是 | pending/approved/rejected/paid | pending | 状态 |
| responsible_party | string | 否 | user/photographer/merchant | null | 责任方 |
| reason | string | 否 | ≤200 | null | 原因 |
| proof_url | string | 否 | URL | null | 凭证 |

### 15.9 Delivery（交付）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| order_id | bigint | 是 | >0 | - | 订单ID |
| status | string | 是 | pending/submitted/accepted/rejected | pending | 状态 |
| items | array | 是 | 1~200 | - | 交付文件 |

### 15.10 Review（评价）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| order_id | bigint | 是 | >0 | - | 订单ID |
| score | int | 是 | 1~5 | - | 评分 |
| tags | array | 否 | ≤10 | [] | 标签 |
| comment | string | 否 | ≤500 | null | 评论 |

### 15.11 Dispute（纠纷）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| order_id | bigint | 是 | >0 | - | 订单ID |
| status | string | 是 | submitted/handling/closed | submitted | 状态 |
| reason | string | 是 | 1~200 | - | 原因 |
| evidence | array | 否 | ≤10 | [] | 证据 |

### 15.12 Merchant（商户）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| name | string | 是 | 2~50 | - | 名称 |
| logo_url | string | 否 | URL | null | Logo |
| brand_color | string | 否 | HEX | null | 品牌色 |
| contact_user_id | bigint | 是 | >0 | - | 联系人 |
| status | string | 是 | pending/approved/frozen | pending | 状态 |

### 15.13 MerchantTemplate（商户模板）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| merchant_id | bigint | 是 | >0 | - | 商户ID |
| name | string | 是 | 2~50 | - | 模板名 |
| description | string | 否 | ≤200 | null | 描述 |
| delivery_requirements | object | 否 | JSON | {} | 交付要求 |
| items | array | 是 | 1~50 | - | 模板条目 |

### 15.14 MerchantApproval（商户审批）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| demand_id | bigint | 是 | >0 | - | 需求ID |
| merchant_id | bigint | 是 | >0 | - | 商户ID |
| status | string | 是 | draft/pending/approved/rejected | pending | 状态 |
| comment | string | 否 | ≤500 | null | 备注 |

### 15.15 MerchantInvoice（发票）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| merchant_id | bigint | 是 | >0 | - | 商户ID |
| order_id | bigint | 否 | >0 | null | 订单ID |
| title | string | 是 | 2~100 | - | 抬头 |
| tax_no | string | 否 | ≤50 | null | 税号 |
| amount | number | 是 | >0 | - | 金额 |
| status | string | 是 | pending/issued/rejected | pending | 状态 |

### 15.16 Portfolio（作品集）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| photographer_id | bigint | 是 | >0 | - | 摄影师ID |
| title | string | 是 | 1~50 | - | 标题 |
| status | string | 是 | draft/published/blocked | draft | 状态 |

### 15.17 PortfolioItem（作品项）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| portfolio_id | bigint | 是 | >0 | - | 作品集ID |
| url | string | 是 | URL | - | 资源链接 |
| tags | array | 否 | ≤20 | [] | 标签 |
| cover_flag | bool | 否 | true/false | false | 是否封面 |

### 15.18 Team / TeamMember（团队）
**Team**
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| owner_user_id | bigint | 是 | >0 | - | 团队所有者 |
| name | string | 是 | 2~50 | - | 团队名 |
| status | string | 是 | active/frozen/deleted | active | 状态 |

**TeamMember**
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| team_id | bigint | 是 | >0 | - | 团队ID |
| user_id | bigint | 是 | >0 | - | 成员用户 |
| role | string | 是 | admin/member | member | 角色 |

### 15.19 DemandAttachment（需求附件）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| demand_id | bigint | 是 | >0 | - | 需求ID |
| file_url | string | 是 | URL | - | 文件链接 |
| file_type | string | 否 | image/file | image | 类型 |

### 15.20 QuoteItem（报价明细）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| quote_id | bigint | 是 | >0 | - | 报价ID |
| name | string | 是 | 1~100 | - | 名称 |
| price | number | 是 | >0 | - | 单价 |
| quantity | int | 是 | 1~1000 | 1 | 数量 |

### 15.21 OrderItem（订单明细）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| order_id | bigint | 是 | >0 | - | 订单ID |
| name | string | 是 | 1~100 | - | 名称 |
| price | number | 是 | >0 | - | 单价 |
| quantity | int | 是 | 1~1000 | 1 | 数量 |

### 15.22 DeliveryItem（交付明细）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| delivery_id | bigint | 是 | >0 | - | 交付ID |
| file_url | string | 是 | URL | - | 文件链接 |
| version | string | 否 | 1~20 | v1 | 版本 |
| note | string | 否 | ≤200 | null | 备注 |

### 15.23 Conversation / Message（会话与消息）
**Conversation**
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| type | string | 是 | order/chat | chat | 类型 |
| order_id | bigint | 否 | >0 | null | 关联订单 |

**Message**
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| conversation_id | bigint | 是 | >0 | - | 会话ID |
| sender_id | bigint | 是 | >0 | - | 发送者 |
| content | string | 否 | ≤2000 | null | 内容 |
| msg_type | string | 是 | text/image/file | text | 类型 |

### 15.24 Notification（通知）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| user_id | bigint | 是 | >0 | - | 用户ID |
| type | string | 是 | ≤50 | - | 类型 |
| title | string | 是 | ≤100 | - | 标题 |
| content | string | 否 | ≤500 | null | 内容 |
| read_at | datetime | 否 | - | null | 已读时间 |

### 15.25 MerchantLocation（商户门店）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| merchant_id | bigint | 是 | >0 | - | 商户ID |
| name | string | 是 | 2~100 | - | 门店名称 |
| address | string | 否 | ≤200 | null | 地址 |
| city_id | bigint | 否 | >0 | null | 城市 |

### 15.26 MerchantUser（商户成员）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| merchant_id | bigint | 是 | >0 | - | 商户ID |
| user_id | bigint | 是 | >0 | - | 用户ID |
| role | string | 是 | requester/approver/finance | requester | 角色 |

### 15.27 MerchantContract（商户合同）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| order_id | bigint | 是 | >0 | - | 订单ID |
| terms | object | 是 | JSON | - | 合同条款 |
| version | int | 是 | ≥1 | 1 | 版本 |

### 15.28 MerchantTemplateItem（商户模板明细）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| template_id | bigint | 是 | >0 | - | 模板ID |
| name | string | 是 | 1~100 | - | 名称 |
| quantity | int | 是 | 1~1000 | 1 | 数量 |
| price | number | 是 | >0 | - | 单价 |

### 15.29 MerchantInvoice（商户发票）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| merchant_id | bigint | 是 | >0 | - | 商户ID |
| order_id | bigint | 否 | >0 | null | 订单ID |
| title | string | 是 | 2~100 | - | 抬头 |
| tax_no | string | 否 | ≤50 | null | 税号 |
| amount | number | 是 | >0 | - | 金额 |
| status | string | 是 | pending/issued/rejected | pending | 状态 |

### 15.30 MerchantAsset（商户素材）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| merchant_id | bigint | 是 | >0 | - | 商户ID |
| asset_type | string | 是 | logo/brand/style/reference | - | 类型 |
| name | string | 是 | 2~100 | - | 名称 |
| status | string | 是 | active/archived | active | 状态 |
| latest_version | int | 否 | ≥1 | null | 最新版本 |
| latest_payload | object | 否 | JSON | null | 最新内容 |

### 15.31 MerchantAssetVersion（素材版本）
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| asset_id | bigint | 是 | >0 | - | 素材ID |
| version | int | 是 | ≥1 | 1 | 版本号 |
| payload | object | 是 | JSON | - | 内容 |
| created_by | bigint | 是 | >0 | - | 创建人 |

### 15.32 AuditLog / Config（审计与配置）
**AuditLog**
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| admin_id | bigint | 是 | >0 | - | 管理员ID |
| action | string | 是 | ≤100 | - | 动作 |
| target_type | string | 否 | ≤50 | null | 目标类型 |
| target_id | bigint | 否 | >0 | null | 目标ID |
| detail | object | 否 | JSON | {} | 详情 |

**Config**
| 字段 | 类型 | 必填 | 规则 | 默认 | 说明 |
|---|---|---|---|---|---|
| key | string | 是 | 1~100 | - | 配置键 |
| value | object | 是 | JSON | - | 配置值 |
