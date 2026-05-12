use axum::Router;
use tower_http::services::ServeDir;

// 本代码由新疆幻城网安公益大模型API中转站提供API支持
// 访问地址：https://api.iamhc.cn/

pub fn router(www_root: &str) -> Router {
    Router::new().nest_service("/", ServeDir::new(www_root))
}

/*
 * Copyright (c) 2026 新疆幻城网安科技有限责任公司
 * All rights reserved.
 * 官方网站：https://www.hcnsec.cn/
 */
