use crate::models::Order;
use tokio::fs;

pub async fn save_order(order: Order) {
    let json = serde_json::to_string_pretty(&order).unwrap();
    fs::create_dir_all("orders").await.unwrap();
    let safe_name = sanitize_filename(&order.name);
    let file_path = format!("orders/order-{}-{}.json", order.id, safe_name);
    fs::write(file_path, json).await.unwrap();
}
