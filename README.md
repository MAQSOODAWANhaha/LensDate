# 约拍平台

> 说明：本仓库为“约拍平台”单体后端 + 移动端 + 管理后台的 Monorepo。支付为直付模式，平台仅记录交易与服务费，不托管资金。

## 项目简介
- **移动端**：Flutter（iOS/Android）
- **管理后台**：React + Ant Design（Vite）
- **后端**：Rust + Axum + SeaORM
- **数据库**：PostgreSQL
- **缓存**：Redis

## 目录结构
```
apps/
  mobile/           # Flutter 移动端
  admin_web/        # 管理后台（React）
backend/            # Rust 单体后端服务
docs/               # PRD/需求/架构/API/数据库等文档
```

## 开发环境
### 后端
1. 配置环境变量：
   - `DATABASE_URL`：PostgreSQL 连接字符串（必填）
2. 运行迁移：
   ```bash
   cargo run --bin migrate
   ```
3. 启动服务：
   ```bash
   cargo run
   ```

### 移动端
```bash
cd apps/mobile
flutter pub get
flutter run
```

### 管理后台
```bash
cd apps/admin_web
npm install
npm run dev
```

## 质量规范
- 后端：`cargo clippy --all-targets -- -D warnings`
- 移动端：`flutter analyze`
- 管理后台：`npm run lint`

## 管理后台登录方式
后端支持三种登录方式（见 `docs/api.md`）：
- 手机号 + 验证码
- 手机号 + 密码
- 用户名 + 密码

> 说明：密码登录需提前为管理员设置 `password_hash`，用户名为可选字段。

## 相关文档
- PRD：`docs/PRD.md`
- 需求说明：`docs/requirements.md`
- 架构方案：`docs/architecture.md`
- API 文档：`docs/api.md`
- 数据库设计：`docs/database-schema.md`
- 部署指南：`docs/deploy.md`

## 部署目录
- 部署相关文件统一放在 `deploy/`（Dockerfile 等）。
