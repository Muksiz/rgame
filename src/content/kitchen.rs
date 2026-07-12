//! Cooking at Poppy's ovens, and the favorites the dishes were made for.
//! The kitchen is the second half of the coins-&-kitchens layer: forage and
//! garden crops combine into a handful of little dishes (`e` at the bakery's
//! range opens the recipe book), and every dish is a gift — each named
//! villager has a favorite and one new line for receiving it, remembered by
//! a flag like the moon-mint. No stats, no buffs; the reward is the line.

use crate::content::market::{Good, Pantry};

pub struct Recipe {
    pub dish: Good,
    pub needs: &'static [(Good, u32)],
    /// The toast when it comes out of the oven.
    pub done: &'static str,
}

/// Poppy's whole recipe book, in oven order.
pub const RECIPES: [Recipe; 4] = [
    Recipe {
        dish: Good::MushroomPasty,
        needs: &[(Good::Mushroom, 2)],
        done: "The pasty comes out golden, top to bottom.",
    },
    Recipe {
        dish: Good::BerryTart,
        needs: &[(Good::Berries, 2)],
        done: "The tart comes out bubbling at the seams.",
    },
    Recipe {
        dish: Good::TurnipSoup,
        needs: &[(Good::Turnip, 1), (Good::Mushroom, 1)],
        done: "The soup comes together like it remembered how.",
    },
    Recipe {
        dish: Good::PumpkinPie,
        needs: &[(Good::Pumpkin, 1), (Good::Berries, 1)],
        done: "The pie comes out looking extremely pleased with itself.",
    },
];

/// Whether the basket holds everything a recipe asks for.
pub fn can_cook(recipe: &Recipe, pantry: &Pantry) -> bool {
    recipe
        .needs
        .iter()
        .all(|(good, n)| pantry.get(good).copied().unwrap_or(0) >= *n)
}

/// A villager's favorite dish: who, what, the flag that remembers the gift,
/// and their lines for receiving it.
pub struct Favorite {
    pub npc: &'static str,
    pub dish: Good,
    pub flag: &'static str,
    pub pages: &'static [&'static str],
}

pub const FAVORITES: [Favorite; 4] = [
    Favorite {
        npc: "Elder Rowan",
        dish: Good::TurnipSoup,
        flag: "gift.rowan",
        pages: &[
            "Is that - turnip soup? With chanterelles in? My mother made it just like this, you know. Just exactly like this.",
            "*(You hand over the turnip soup. Rowan holds the bowl with both hands, the way you hold a warm thing you have missed.)*",
        ],
    },
    Favorite {
        npc: "Juno",
        dish: Good::BerryTart,
        flag: "gift.juno",
        pages: &[
            "A WHOLE TART? For ME? I'm going to eat it so slowly it lasts until winter. ...Okay, it's gone. It was the best one ever made.",
            "*(You hand over the berry tart. There is immediately jam on Juno's nose.)*",
        ],
    },
    Favorite {
        npc: "Forager Maren",
        dish: Good::MushroomPasty,
        flag: "gift.maren",
        pages: &[
            "A pasty. With MY mushrooms - the woods' mushrooms, I mean. Do you know, in all these years I've only ever dried them? Never once thought to put them in a crust.",
            "*(You hand over the mushroom pasty. Maren eats half on the spot and pockets the rest 'for the trail', which starts three steps away.)*",
        ],
    },
    Favorite {
        npc: "Hermit Morrow",
        dish: Good::PumpkinPie,
        flag: "gift.morrow",
        pages: &[
            "Hm. Most folk come out here carrying questions. You've come carrying pie. That is a meaningful improvement.",
            "*(You hand over the pumpkin pie. Morrow cuts two slices - and hands one of them back to you. You watch the river together, quiet about it.)*",
        ],
    },
];

/// The favorite waiting on this villager, if any.
pub fn favorite(npc: &str) -> Option<&'static Favorite> {
    FAVORITES.iter().find(|f| f.npc == npc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::market;

    #[test]
    fn every_recipe_cooks_from_obtainable_goods() {
        for recipe in &RECIPES {
            for (good, n) in recipe.needs {
                assert!(*n > 0);
                // Everything a recipe asks for is foraged or grown — never
                // another dish, never a bought trinket.
                assert!(
                    market::sells_for(*good).is_some(),
                    "{:?} needs {:?}, which the world doesn't yield",
                    recipe.dish,
                    good
                );
            }
            assert!(
                market::sells_for(recipe.dish).is_none(),
                "{:?} would be sellable — dishes are gifts",
                recipe.dish
            );
        }
    }

    #[test]
    fn every_favorite_dish_has_a_recipe_and_its_own_flag() {
        let mut flags = std::collections::BTreeSet::new();
        for fav in &FAVORITES {
            assert!(
                RECIPES.iter().any(|r| r.dish == fav.dish),
                "{}'s favorite can't be cooked",
                fav.npc
            );
            assert!(flags.insert(fav.flag), "flag {} is shared", fav.flag);
            assert!(
                fav.pages.len() >= 2,
                "{} deserves a real thank-you",
                fav.npc
            );
        }
    }

    #[test]
    fn can_cook_counts_the_basket_honestly() {
        let soup = &RECIPES[2];
        let mut pantry = Pantry::new();
        assert!(!can_cook(soup, &pantry));
        pantry.insert(Good::Turnip, 1);
        assert!(!can_cook(soup, &pantry), "soup wants a chanterelle too");
        pantry.insert(Good::Mushroom, 1);
        assert!(can_cook(soup, &pantry));
    }
}
