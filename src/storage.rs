use crate::models::Order;
use tokio::fs;

pub async fn save_order(order: Order) {
    let json = serde_json::to_string_pretty(&order).unwrap();
    fs::create_dir_all("orders").await.unwrap();
    let file_path = format!("orders/order_{}.json", order.id);
    // 用id作为文件名而非用户输入，防止报错（已踩坑）
    fs::write(file_path, json).await.unwrap();
}
