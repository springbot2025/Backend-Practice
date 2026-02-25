use tokio::time::{sleep, Duration};
use crate::models::Order;

pub async fn process_coffee(order : Order) {
    let time = order.time as u64;
    println!("请等待制作完成……");
    println!("预计制作需要{}秒", time);
    sleep(Duration::from_secs(time)).await;
    println!("{}制作完成！请取餐", order.typ.name());
}