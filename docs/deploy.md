# 部署指南

> 说明：本指南覆盖依赖部署与项目部署，适用于“单体后端 + 管理后台 + 移动端”架构。支付为直付模式，平台仅记录交易与服务费。

## 1. 依赖部署（基础设施）
### 1.1 PostgreSQL
- 版本：建议使用最新稳定版本
- 必需：创建数据库与账号
- 环境变量：`DATABASE_URL`（后端启动必填）

### 1.2 Redis
- 版本：建议使用最新稳定版本
- 用途：会话/验证码/热点缓存

### 1.3 对象存储（可选）
- S3 兼容存储（MinIO/云存储）
- 用于：作品集/附件/交付文件

### 1.4 Docker Compose（依赖）
依赖服务 Compose 文件（PG/Redis）：
- 路径：`deploy/docker/docker-compose.yml`
- 启动：
  ```bash
  docker compose -f deploy/docker/docker-compose.yml up -d
  ```

## 2. 后端部署（Rust + Axum）
### 2.1 环境变量
- `DATABASE_URL`：PostgreSQL 连接字符串（必填）
- `ADMIN_PHONES` / `OPS_PHONES` / `MANAGER_PHONES`：管理后台角色白名单（可选）

### 2.2 迁移执行
```bash
cargo run --bin migrate
```

### 2.3 运行服务
```bash
cargo run
```
默认监听：`0.0.0.0:8080`，API 前缀为 `/api/v1`。

### 2.4 Dockerfile
统一 Dockerfile：`deploy/docker/Dockerfile`（构建后端 + 管理后台静态文件）
```bash
docker build -t yuepai-app -f deploy/docker/Dockerfile .
docker run --rm -p 8080:8080 \
  -e DATABASE_URL=postgres://user:pass@host:5432/db \
  yuepai-app
```

## 3. 管理后台部署（React + Ant Design）
### 3.1 本地构建
```bash
cd apps/admin_web
npm install
npm run build
```
构建产物：`apps/admin_web/dist/`

### 3.2 说明
管理后台静态文件由后端 Axum 服务统一托管（容器内路径 `/app/admin_web`）。如需前后端拆分部署，可单独使用静态服务器托管 `apps/admin_web/dist/`。

## 4. 移动端部署（Flutter）
### 4.1 Android
```bash
cd apps/mobile
flutter build apk --release
```
产物：`apps/mobile/build/app/outputs/flutter-apk/app-release.apk`

### 4.2 iOS
```bash
cd apps/mobile
flutter build ios --release
```
按 Apple 签名/发布流程进行打包与上架。

## 5. 生产环境建议
- 后端使用反向代理（Caddy/Nginx 均可）与 HTTPS
- 数据库与 Redis 单独部署并开启备份
- 日志与监控：Prometheus + Grafana
- 按环境区分配置（dev/staging/prod）
