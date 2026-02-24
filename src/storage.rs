use crate::models::Order;
use tokio::fs;

pub async fn save_order(order : Order) {
    let json = serde_json::to_string_pretty(&order).unwrap();
    fs::create_dir_all("orders").await.unwrap(); //创建文件和写入json
    fs::write(format!("orders/{}'s_order.json", order.name), json).await.unwrap();
}