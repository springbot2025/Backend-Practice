mod models;
mod storage;
mod processor;

use tokio::io::{self, AsyncBufReadExt, BufReader};
use std::sync::atomic::{AtomicU64, Ordering};

// 定义一个全局静态变量作为计数器，初始值为 1
static NEXT_ORDER_ID: AtomicU64 = AtomicU64::new(1);
#[tokio::main]

async fn main() {
    println!("Spring's coffee shop is running……");

    println!("请输入你的名字:");
    let stdin = io::stdin();
    let mut name = String::new();
    let mut reader = BufReader::new(stdin);
    reader.read_line(&mut name).await.unwrap();
    // 异步读取，防止卡死
    let name = name.trim();
    println!("你好,{}", name);
    loop {
        println!("咖啡菜单：");
        for (i, coffee) in models::Coffee::ALL.iter().enumerate() {
            println!("{}. {}", i + 1, coffee.name());
        }
        println!("请输入编号下单（输入quit退出程序）");

        let mut choice = String::new();
        reader.read_line(&mut choice).await.unwrap();
        if choice.trim() == "quit"{
            println!("谢谢光临，再见！");
            break; 
        }
        print!("你选择了：");
        let coffee = match choice.trim(){

            "1" => models::Coffee::IceWater,
            "2" => models::Coffee::HandDrip,
            "3" => models::Coffee::Espresso,
            "4" => models::Coffee::Latte,
            "5" => models::Coffee::Cappuccino,
            _ => {
                println!("这是无效的输入，请重新选择");
                continue;
            },
        };
        let order_id = NEXT_ORDER_ID.fetch_add(1, Ordering::SeqCst);
        let order = models::Order{
            id: order_id,
            name: name.to_string(),
            time: coffee.time(),
            price: coffee.price(),
            typ: coffee,
        };
        println!("价格为{}元，请付款", &coffee.price());
    // 下面是制作过程，发送订单到后台制作和处理为Json
    {
        let order = order.clone();//order.clone()仅在这个作用域生效
        tokio::spawn(async move{
            processor::process_coffee(&order).await;
        });
    }
    storage::save_order(order).await;
    //制作完成
    }
}
