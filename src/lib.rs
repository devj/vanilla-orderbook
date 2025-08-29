use std::cmp::Ordering;

pub mod orderbook;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub price: u64,
    pub quantity: u64,
    pub timestamp: u128,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub price: u64,
    pub quantity: u64,
    pub maker_id: u64,
    pub taker_id: u64,
}

// Custom key for BTreeMap that implements price-time priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PriceLevel {
    price: u64,
    side: Side,
}

impl PriceLevel {
    pub fn new(price: u64, side: Side) -> Self {
        Self {
            price,
            side
        }
    }
}

impl PartialOrd for PriceLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriceLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.side {
            Side::Buy => {
                // For buy orders, higher price is better (reverse order)
                other.price.cmp(&self.price)
            }
            Side::Sell => {
                // For sell orders, lower price is better (normal order)
                self.price.cmp(&other.price)
            }
        }
    }
}


pub mod prelude {
    pub use crate::{Order, Trade, Side, PriceLevel};
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Side {
        Buy,
        Sell,
    }

    #[test]
    fn test_price_level_equality() {
        let a = PriceLevel::new(100, Side::Buy);
        let b = PriceLevel::new(100, Side::Buy);
        let c = PriceLevel::new(100, Side::Sell);

        assert_eq!(a, b, "Two identical buy price levels should be equal");
        assert_ne!(a, c, "Buy and sell at same price should not be equal");
    }

    #[test]
    fn test_buy_order_priority() {
        let low = PriceLevel::new(90, Side::Buy);
        let high = PriceLevel::new(100, Side::Buy);

        // Higher price comes first
        assert!(high < low, "Higher buy price should have priority");
    }

    #[test]
    fn test_sell_order_priority() {
        let low = PriceLevel::new(90, Side::Sell);
        let high = PriceLevel::new(100, Side::Sell);

        // Lower price comes first
        assert!(low < high, "Lower sell price should have priority");
    }

    #[test]
    fn test_btree_ordering_for_buys() {
        let mut map = BTreeMap::new();
        map.insert(PriceLevel::new(100, Side::Buy), "buy100");
        map.insert(PriceLevel::new(95, Side::Buy), "buy95");
        map.insert(PriceLevel::new(105, Side::Buy), "buy105");

        let keys: Vec<_> = map.keys().collect();
        assert_eq!(
            keys,
            &[
                &PriceLevel::new(105, Side::Buy),
                &PriceLevel::new(100, Side::Buy),
                &PriceLevel::new(95, Side::Buy),
            ],
            "Buys should be ordered high to low"
        );
    }

    #[test]
    fn test_btree_ordering_for_sells() {
        let mut map = BTreeMap::new();
        map.insert(PriceLevel::new(100, Side::Sell), "sell100");
        map.insert(PriceLevel::new(95, Side::Sell), "sell95");
        map.insert(PriceLevel::new(105, Side::Sell), "sell105");

        let keys: Vec<_> = map.keys().collect();
        assert_eq!(
            keys,
            &[
                &PriceLevel::new(95, Side::Sell),
                &PriceLevel::new(100, Side::Sell),
                &PriceLevel::new(105, Side::Sell),
            ],
            "Sells should be ordered low to high"
        );
    }

    #[test]
    fn test_mixed_sides_do_not_conflict() {
        let mut map = BTreeMap::new();
        map.insert(PriceLevel::new(100, Side::Buy), "buy100");
        map.insert(PriceLevel::new(100, Side::Sell), "sell100");

        assert_eq!(map.len(), 2, "Buy and sell at same price should coexist");
    }
}

