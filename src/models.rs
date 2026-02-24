use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Coffee{
    IceWater,
    HandDrip,
    Espresso,
    Latte,
    Cappuccino,
}

impl Coffee {
    pub const ALL: [Coffee; 5] = [
        Self::IceWater,
        Self::HandDrip,
        Self::Espresso,
        Self::Latte,
        Self::Cappuccino,
    ];
    pub fn name(&self) -> &str {// mingzi
        match self {
            Self::IceWater => "冰水",
            Self::HandDrip => "手冲",
            Self::Espresso => "浓缩咖啡",
            Self::Latte => "拿铁",
            Self::Cappuccino => "卡布奇诺",
        }
    }
    pub fn time(&self) -> u16 {// by second
        match self {
            Self::IceWater => 1,
            Self::HandDrip => 10,
            Self::Espresso => 60,
            Self::Latte => 100,
            Self::Cappuccino => 120,
        }
    }
    pub fn price(&self) -> u16 {// by rmb
        match self {
            Self::IceWater => 1,
            Self::HandDrip => 10,
            Self::Espresso => 20,
            Self::Latte => 25,
            Self::Cappuccino => 28,
        }
    }
}
#[derive(Serialize, Clone)]
pub struct Order {
    pub typ:Coffee,
    pub name: String,
    pub time: u16,
    pub price: u16,
}

