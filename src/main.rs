use axum::{
    routing::get,
    Router,
};

#[tokio::main] // 这个宏让 main 函数支持异步运行
async fn main() {
    // 1. 定义路由：访问 "/" 路径时，执行下面的 handler 函数
    let app = Router::new().route("/", get(handler));

    // 2. 绑定监听端口（监听本地 3000 端口）
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("服务器已启动：访问 http://localhost:3000");

    // 3. 启动服务器
    axum::serve(listener, app).await.unwrap();
}

// 一个简单的异步函数，处理请求
async fn handler() -> &'static str {
    "Hello, Rust Backend!"
}