# 项目路径规划方案

> 说明：整体采用 Monorepo 结构，便于统一规范、复用模块与迭代。移动端目录统一为 `apps/mobile/`。

```
/config/workspace/photo/
  AGENTS.md
  docs/
    PRD.md
    requirements.md
    task_plan.md
    findings.md
    progress.md
    project-structure.md
    architecture.md
    deploy.md

  deploy/
    docker/

  apps/
    mobile/
      android/
      ios/
      lib/
        core/                # 路由、DI、主题、错误处理
        shared/              # 通用组件/工具/常量
        data/                # models、repositories、api clients
        features/
          auth/              # 登录/认证/身份
          profile/           # 个人中心
          photographer/      # 摄影师/团队入驻与作品
          demand/            # 需求发布
          match/             # 搜索/匹配/推荐
          chat/              # IM
          quote/             # 报价/方案
          order/             # 订单/状态流转
          payment/           # 支付（直付）
          delivery/          # 履约/交付
          review/            # 评价与信用
          dispute/           # 纠纷/售后
          merchant/          # 瑜伽馆商户能力
        l10n/
      assets/
      test/

    admin_web/               # 管理后台（React + Ant Design）
      src/
        modules/
          audit/             # 审计日志
          content/           # 内容审核（作品集）
          orders/            # 订单/支付/退款
          disputes/          # 纠纷处理
          ops/               # 运营配置
          users/             # 用户/摄影师管理
      tests/

  backend/
    Cargo.toml               # Rust 单体后端服务
    src/
      main.rs                # API 服务入口
      lib.rs                 # 模块导出
      bin/
        migrate.rs           # 迁移入口（独立二进制）
      state.rs               # AppState（DB/缓存/配置）
      common.rs              # 统一响应结构
      error.rs               # ApiError 与统一错误映射
      config/                # 配置加载与校验
      routes/                # 仅路由注册（不写业务逻辑）
      handlers/              # 控制层（参数解析/鉴权前置校验）
      services/              # 业务层（状态机/事务/核心规则）
      repositories/          # 数据访问层（SeaORM 查询封装）
      dto/                   # 请求/响应模型与校验
      errors/                # 领域错误（thiserror）
      utils/                 # 通用工具
      middleware/            # 中间件与鉴权（AuthUser 等）
      entity/                # SeaORM 实体
      migration/             # SeaORM 迁移（Rust DSL）
    configs/                 # 环境配置模板
    scripts/                 # 数据初始化/修复脚本
    tests/                   # 集成/回归测试

  contracts/
    openapi/                 # API 规范
    events/                  # 领域事件定义

  scripts/
    dev/
    ci/

  .github/
    workflows/
```

## 命名与使用规范
- **移动端**：统一指 Flutter App（iOS/Android），目录为 `apps/mobile/`。
- **管理后台**：统一指 Web 管理后台，目录为 `apps/admin_web/`。
- **后端服务**：统一指 Rust 单体后端服务，目录为 `backend/`。
- **迁移与实体**：迁移目录 `backend/src/migration/`，迁移入口 `backend/src/bin/migrate.rs`，实体目录 `backend/src/entity/`。
- **文档入口**：PRD/需求/架构/API 文档分别为 `docs/PRD.md`、`docs/requirements.md`、`docs/architecture.md`、`docs/api.md`。

## 目录说明（与需求对齐）
- 个人用户与瑜伽馆商户场景：`apps/mobile/lib/features/merchant` + `backend/src/handlers`/`services`/`repositories` 对应商户能力。
- 直付与退款规则：`apps/mobile/lib/features/payment` + `backend/src/handlers`/`services`/`repositories` 的支付与退款模块。
- 纠纷与售后：`apps/mobile/lib/features/dispute` + `backend/src/handlers`/`services`/`repositories` 的纠纷模块。
- 资质/内容审核：`apps/admin_web/src/modules/audit` + `backend/src/handlers`/`services`/`repositories` 的管理后台模块。
- 错误处理：`backend/src/errors`（领域错误） + `backend/src/error.rs`（统一错误映射），遵循 thiserror + anyhow 方案。
