//! Coins & the trading post: the quiet side-economy. Nothing on the main
//! road ever costs a coin — money is a side-layer, like fishing. Greengrocer
//! Marla's stall on the Emberwick square buys what the world yields (forage
//! from the hedgerows and hollows, crops off the garden plots) and sells
//! seeds and small goods. Coins and the basket are the first stored counters
//! (`SaveData::coins` / `SaveData::pantry`, both behind `#[serde(default)]`);
//! everything else here is static content.

use std::collections::BTreeMap;

/// Who keeps the trading post. Talking to her opens the trade screen.
pub const KEEPER: &str = "Greengrocer Marla";

/// Everything the basket can hold: forage, seeds, garden crops, and the
/// little dishes Poppy's ovens turn them into.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Good {
    Mushroom,
    Berries,
    Turnip,
    Pumpkin,
    TurnipSeeds,
    PumpkinSeeds,
    Pinwheel,
}

/// Every good, in basket (and menu) order.
pub const GOODS: [Good; 7] = [
    Good::Mushroom,
    Good::Berries,
    Good::Turnip,
    Good::Pumpkin,
    Good::TurnipSeeds,
    Good::PumpkinSeeds,
    Good::Pinwheel,
];

impl Good {
    /// The stable id written into `save.json` — never rename these.
    pub fn id(self) -> &'static str {
        match self {
            Good::Mushroom => "mushroom",
            Good::Berries => "berries",
            Good::Turnip => "turnip",
            Good::Pumpkin => "pumpkin",
            Good::TurnipSeeds => "turnip-seeds",
            Good::PumpkinSeeds => "pumpkin-seeds",
            Good::Pinwheel => "pinwheel",
        }
    }

    /// The reverse of `id` — unknown ids (a future save wandered back in
    /// time) simply drop out of the basket rather than breaking the load.
    pub fn from_id(id: &str) -> Option<Good> {
        GOODS.iter().copied().find(|g| g.id() == id)
    }

    pub fn name(self) -> &'static str {
        match self {
            Good::Mushroom => "chanterelle",
            Good::Berries => "hedge-berries",
            Good::Turnip => "turnip",
            Good::Pumpkin => "pumpkin",
            Good::TurnipSeeds => "turnip seeds",
            Good::PumpkinSeeds => "pumpkin seeds",
            Good::Pinwheel => "paper pinwheel",
        }
    }

    /// A line about the good, shown under the trade menu's selected row.
    pub fn blurb(self) -> &'static str {
        match self {
            Good::Mushroom => {
                "Golden and apricot-sweet, eased out of the woods' moss. Poppy's ovens could do something with these."
            }
            Good::Berries => {
                "Dark and sharp-sweet, off the village hedgerows. Some made it into the basket."
            }
            Good::Turnip => "A garden turnip, heavy for its size and smug about it.",
            Good::Pumpkin => {
                "Small, round, and very pleased with itself. Grown, not bought. Well - grown from bought."
            }
            Good::TurnipSeeds => {
                "A paper twist of turnip seeds. Tuck them into tilled soil; they do their growing while you sleep."
            }
            Good::PumpkinSeeds => {
                "Pumpkin seeds, flat as coins. A few campfire rests from bragging rights."
            }
            Good::Pinwheel => {
                "A striped paper pinwheel. It spins. That is all it does, and that is plenty."
            }
        }
    }
}

/// The basket: everything gathered, grown, or cooked, by count.
pub type Pantry = BTreeMap<Good, u32>;

/// What Marla pays for a good — `None` for things she won't buy (seeds go
/// the other way, and the pinwheel would only make her keep it).
pub fn sells_for(good: Good) -> Option<u32> {
    match good {
        Good::Mushroom => Some(3),
        Good::Berries => Some(2),
        Good::Turnip => Some(4),
        Good::Pumpkin => Some(6),
        _ => None,
    }
}

/// The stall's standing stock: what Marla sells, and for how much.
pub const STOCK: [(Good, u32); 3] = [
    (Good::TurnipSeeds, 2),
    (Good::PumpkinSeeds, 4),
    (Good::Pinwheel, 5),
];

/// One row of the trade menu: a good, its price, and which way it moves.
#[derive(Clone, Copy)]
pub struct TradeRow {
    pub good: Good,
    pub price: u32,
    pub sell: bool,
}

/// The trade menu, top to bottom: first everything in the basket Marla will
/// buy, then her standing stock. Never empty — the stock rows always stand.
pub fn trade_rows(pantry: &Pantry) -> Vec<TradeRow> {
    let mut rows: Vec<TradeRow> = pantry
        .iter()
        .filter(|&(_, &n)| n > 0)
        .filter_map(|(&good, _)| {
            sells_for(good).map(|price| TradeRow {
                good,
                price,
                sell: true,
            })
        })
        .collect();
    rows.extend(STOCK.iter().map(|&(good, price)| TradeRow {
        good,
        price,
        sell: false,
    }));
    rows
}

/// Marla's greeting — the dialogue that opens the stall.
pub fn greeting() -> Vec<String> {
    vec![
        "Apples from the east orchard, pears from the west - and a fair price for whatever the hedgerows and hollows gave you. Let's see the basket, then.".into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_good_round_trips_through_its_save_id() {
        for good in GOODS {
            assert_eq!(Good::from_id(good.id()), Some(good), "{good:?}");
        }
        assert_eq!(Good::from_id("a-good-from-the-future"), None);
    }

    #[test]
    fn the_menu_lists_basket_then_stock_and_never_stands_empty() {
        let empty = Pantry::new();
        let rows = trade_rows(&empty);
        assert_eq!(rows.len(), STOCK.len(), "an empty basket still shows stock");
        assert!(rows.iter().all(|r| !r.sell));

        let mut pantry = Pantry::new();
        pantry.insert(Good::Mushroom, 2);
        pantry.insert(Good::Pinwheel, 1); // hers already; she won't buy it back
        let rows = trade_rows(&pantry);
        assert_eq!(rows.len(), STOCK.len() + 1);
        assert!(rows[0].sell && rows[0].good == Good::Mushroom);
    }

    #[test]
    fn seeds_cost_less_than_the_crop_sells_for() {
        // Gardening must stay gently worth it.
        for (seed, crop) in [
            (Good::TurnipSeeds, Good::Turnip),
            (Good::PumpkinSeeds, Good::Pumpkin),
        ] {
            let cost = STOCK.iter().find(|(g, _)| *g == seed).unwrap().1;
            assert!(cost < sells_for(crop).unwrap(), "{seed:?} beats {crop:?}");
        }
    }
}
