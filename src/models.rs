use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ItemInfo {
    pub name: String,
    pub price: f64,
    pub count: u32,
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub items: HashMap<String, u32>,
    pub total_spent: f64,
}

impl PlayerInfo {
    pub fn new(name: String) -> Self {
        Self {
            name,
            items: HashMap::new(),
            total_spent: 0.0,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref ITEM_PRICES: HashMap<&'static str, f64> = {
        let mut m = HashMap::new();
        m.insert("sredni zestaw kluczy", 9.99);
        m.insert("maly zestaw kluczy", 4.99);
        m.insert("dostep do osobnego kanalu (/ch)", 19.99);
        m.insert("gigabox (x5)", 99.99);
        m.insert("gigabox (x3)", 74.99);
        m.insert("gigabox (x2)", 54.99);
        m.insert("gigabox (x1)", 24.99);
        m.insert("range chad na edycje", 49.99);
        m.insert("ogromny zestaw kluczy", 49.99);
        m.insert("paiet chad", 99.99);
        m.insert("motyke 7x7 (49 blokow na raz)", 129.99);
        m.insert("range vip na edycje", 4.99);
        m.insert("range svip na edycje", 9.99);
        m.insert("range elita na edycje", 29.99);
        m.insert("duzy zestaw kluczy", 19.99);
        m.insert("range sponsor na edycje", 19.99);
        m.insert("odblokowanie slotow do /pet", 19.99);
        m.insert("najwiekszy zestaw kluczy", 99.99);
        m.insert("transfer wand", 49.99);
        m.insert("motyke hallowenowa (10x10, az 100 blokow na raz)", 194.99);
        m.insert("przepustka (mnozy zdobywane cukierki x3!)", 49.99);
        m.insert("pakiet chad", 99.99);
        m.insert("najlepsza motyke (/najlepsze)", 49.99);
        m
    };
}