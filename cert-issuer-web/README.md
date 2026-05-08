# 证书签发服务 - Web前端

Vue 3 + TypeScript + Vite 前端项目

## 快速启动

### 1. 启动后端服务

```bash
cd 证书签发服务
cargo run --release
```

后端服务将在 http://localhost:8080 运行

### 2. 启动前端开发服务器

```bash
cd cert-issuer-web
npm run dev
```

前端将在 http://localhost:5173 运行

### 3. 生产构建

```bash
npm run build
```

构建产物输出到 `dist/` 目录

## 功能

- 仪表盘：系统健康状态、证书统计
- 证书列表：查看所有已签发证书、支持搜索排序
- 颁发证书：创建新数字证书
- 证书详情：查看完整信息、下载证书

## API 代理

开发模式下，Vite 配置了 `/api` -> `http://localhost:8080` 的代理，无需额外配置即可访问后端API。