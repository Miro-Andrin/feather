use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Index, IndexMut},
};

use base::{Item, ItemStack};
use itertools::Itertools;

// https://wiki.vg/Protocol#Declare_Recipes

pub type RecipieId = u32;

/// Should be constructed through converting a
/// [[Item ; 3]; 3] into a Shaped by using ".into()".
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Shaped {
    width: u8,
    height: u8,
    /// Recipies are canocicalised by shifing the recipie up to the top left corner.
    items: [[Option<Item>; 3]; 3],
}

impl Shaped {
    fn craft_tipped_arrow(&self) -> Option<ItemStack> {
        if self.items[0][0] == Some(Item::Arrow)
            && self.items[0][1] == Some(Item::Arrow)
            && self.items[0][2] == Some(Item::Arrow)
            && self.items[1][0] == Some(Item::Arrow)
            && self.items[1][1] == Some(Item::LingeringPotion)
            && self.items[0][2] == Some(Item::Arrow)
            && self.items[2][0] == Some(Item::Arrow)
            && self.items[2][1] == Some(Item::Arrow)
            && self.items[2][2] == Some(Item::Arrow)
        {
            // TODO return the correct type of arrow based on the potion, now it just returns a generic tipped arrow
            Some(ItemStack::new(Item::TippedArrow, 8))
        } else {
            None
        }
    }
}

impl From<[[Option<Item>; 3]; 3]> for Shaped {
    fn from(items: [[Option<Item>; 3]; 3]) -> Self {
        let first_row_with_some = items
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().any(|item| item.is_some()))
            .map(|(y, _)| y)
            .next();

        let first_row_with_some = match first_row_with_some {
            Some(y) => y,
            None => {
                // Then every item is none
                return Shaped {
                    width: 0,
                    height: 0,
                    items: [[None; 3]; 3],
                };
            }
        };

        let last_row_with_some = items
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().any(|item| item.is_some()))
            .map(|(y, _)| y)
            .last();

        let last_row_with_some = match last_row_with_some {
            Some(y) => y,
            None => unreachable!(),
        };

        let first_col_with_some = items
            .iter()
            .skip(first_row_with_some)
            .filter(|row| row.iter().any(|item| item.is_some()))
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter(|x| x.1.is_some())
                    .map(|(col, item)| col)
                    .nth(0)
                    .unwrap()
            })
            .min()
            .unwrap();

        let last_col_with_some = items
            .iter()
            .skip(first_row_with_some)
            .filter(|row| row.iter().any(|item| item.is_some()))
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter(|x| x.1.is_some())
                    .map(|(col, item)| col)
                    .nth(0)
                    .unwrap()
            })
            .max()
            .unwrap();

        let mut new_items = [[None; 3]; 3];

        for row in first_row_with_some..=last_row_with_some {
            for col in first_col_with_some..=last_col_with_some {
                new_items[row - first_row_with_some][col - first_col_with_some] = items[row][col];
            }
        }

        return Shaped {
            width: (last_col_with_some - first_col_with_some + 1) as u8,
            height: (last_row_with_some - first_row_with_some + 1) as u8,
            items: new_items,
        };
    }
}

/// Should be constructed through converting a
/// [Item; N] into a Shapeless.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Shapeless {
    // A sorted list of items.
    items: Vec<Item>,
}

impl<const N: usize> From<[Item; N]> for Shapeless {
    fn from(mut items: [Item; N]) -> Self {
        items.sort();
        Self {
            items: items.into(),
        }
    }
}

impl From<Shaped> for Shapeless {
    fn from(shaped: Shaped) -> Self {
        Self {
            items: shaped
                .items
                .iter()
                .flatten()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .sorted()
                .collect(),
        }
    }
}

pub struct RecipieRegistry {
    recipies: Vec<Recipie>,
    shapeless: HashMap<Shapeless, RecipieId>,
    shaped: HashMap<Shaped, RecipieId>,
    smelting: HashMap<Item, RecipieId>,
    blastfurnace: HashMap<Item, RecipieId>,
    smoking: HashMap<Item, RecipieId>,
    campfire: HashMap<Item, RecipieId>,
    stonecutting: HashMap<Item, RecipieId>,
    smitting: HashMap<(Item, Item), RecipieId>,

    /// Recipe for dying leather armor
    /// [wiki](https://minecraft.fandom.com/wiki/Dye#Dyeing_armor)
    /// (is shapeless)
    /// None => disabled
    /// Some => enabled,
    crafting_special_armordye: Option<RecipieId>,

    /// Recipe for copying contents of written books
    /// [wiki](https://minecraft.fandom.com/wiki/Written_Book#Copying)
    /// (is shapeless)
    crafting_special_bookcloning: Option<RecipieId>,

    /// Recipe for making firework rockets
    /// TODO: Figure out exactly what this means.
    /// (is shapeless)
    crafting_special_firework_rocket: Option<RecipieId>,

    /// Recipe for making firework stars
    /// TODO: Figure out exactly what this means.
    /// (is shapeless)
    crafting_special_firework_star: Option<RecipieId>,

    /// Recipe for making firework stars fade between multiple colors
    /// TODO: Figure out exactly what this means.
    /// (is shapeless)
    crafting_special_firework_star_fade: Option<RecipieId>,

    /// Recipe for repairing items via crafting
    /// Shapeless combine two identical items that are damaged
    /// the same way. (is shapeless)
    crafting_special_repairitem: Option<RecipieId>,

    /// Recipe for copying banner patterns
    /// (is shapeless)
    crafting_special_banner_duplicate: Option<RecipieId>,

    /// Recipe for adding patterns to banners
    /// (is shapeless)
    crafting_special_banner_add_pattern: Option<RecipieId>,

    /// Recipe for applying a banner's pattern to a shield
    /// (is shapeless)
    crafting_special_shielddecoration: Option<RecipieId>,

    /// Recipe for recoloring a shulker box
    /// (is shapeless)
    crafting_special_shulkerboxcoloring: Option<RecipieId>,

    /// Recipie for crafting suspicioussetw
    /// (is shapeless)
    crafting_special_suspiciousstew: Option<RecipieId>,

    /// Recipe for crafting tipped arrows
    /// (is shaped)
    crafting_special_tipped_arrows: Option<RecipieId>,
}

impl Index<RecipieId> for RecipieRegistry {
    type Output = Recipie;
    fn index(&self, index: RecipieId) -> &Self::Output {
        &self.recipies[index as usize]
    }
}

impl IndexMut<RecipieId> for RecipieRegistry {
    fn index_mut(&mut self, index: RecipieId) -> &mut Self::Output {
        &mut self.recipies[index as usize]
    }
}

impl RecipieRegistry {
    pub fn add_or_replace(&mut self, recipie: Recipie) -> (Option<Recipie>, RecipieId) {
        match recipie.clone() {
            Recipie::Shaped(shaped) => {
                match shaped {
                    ShapedRecipie::CraftingShaped {
                        shape,
                        group: _,
                        result: _,
                    } => {
                        // See if we have an existing shaped recipie with the same pattern, and
                        // return it if that is the case.

                        let id = self.shaped.get(&shape);

                        match id {
                            Some(id) => {
                                let last = self.recipies.len();
                                self.recipies.push(recipie.clone());
                                self.recipies.swap(*id as usize, last);
                                let old = self.recipies.pop().unwrap();

                                return (Some(old), *id);
                            }
                            None => {
                                let id = self.recipies.len() as u32;
                                self.shaped.insert(shape, id);
                                self.recipies.push(recipie);
                                return (None, id);
                            }
                        }
                    }
                    ShapedRecipie::CraftingTippedArrows => {
                        let new = Recipie::Shaped(ShapedRecipie::CraftingTippedArrows);
                        match self.crafting_special_tipped_arrows {
                            Some(x) => (Some(new), x),
                            None => {
                                let id = self.recipies.len();
                                self.recipies.push(new.clone());
                                self.crafting_special_tipped_arrows = Some(id as u32);
                                return (Some(new), id as u32);
                            }
                        }
                    }
                }
            }
            Recipie::Shapeless(shapeless) => match shapeless {
                ShapelessRecipie::Shapeless {
                    group: _,
                    shape,
                    result: _,
                } => {
                    let id = self.shapeless.get(&shape);

                    match id {
                        Some(id) => {
                            let last = self.recipies.len();
                            self.recipies.push(recipie.clone());
                            self.recipies.swap(*id as usize, last);
                            let old = self.recipies.pop().unwrap();

                            return (Some(old), *id);
                        }
                        None => {
                            let id = self.recipies.len() as u32;
                            self.shapeless.insert(shape, id);
                            self.recipies.push(recipie);
                            return (None, id);
                        }
                    }
                }
                ShapelessRecipie::CraftingSpecialArmordye => {
                    let new = Recipie::Shapeless(ShapelessRecipie::CraftingSpecialArmordye);
                    match self.crafting_special_armordye {
                        Some(x) => (Some(new), x),
                        None => {
                            let id = self.recipies.len();
                            self.recipies.push(new.clone());
                            self.crafting_special_armordye = Some(id as u32);
                            return (Some(new), id as u32);
                        }
                    }
                }
                ShapelessRecipie::CraftingSpecialBookcloning => {
                    let new = Recipie::Shapeless(ShapelessRecipie::CraftingSpecialBookcloning);
                    match self.crafting_special_bookcloning {
                        Some(x) => (Some(new), x),
                        None => {
                            let id = self.recipies.len();
                            self.recipies.push(new.clone());
                            self.crafting_special_bookcloning = Some(id as u32);
                            return (Some(new), id as u32);
                        }
                    }
                }
                ShapelessRecipie::CraftingSpecialFireworkRocket => {
                    let new = Recipie::Shapeless(ShapelessRecipie::CraftingSpecialFireworkRocket);
                    match self.crafting_special_firework_rocket {
                        Some(x) => (Some(new), x),
                        None => {
                            let id = self.recipies.len();
                            self.recipies.push(new.clone());
                            self.crafting_special_firework_rocket = Some(id as u32);
                            return (Some(new), id as u32);
                        }
                    }
                }
                ShapelessRecipie::CraftingSpecialFireworkStar => {
                    let new = Recipie::Shapeless(ShapelessRecipie::CraftingSpecialFireworkStar);
                    match self.crafting_special_firework_star {
                        Some(x) => (Some(new), x),
                        None => {
                            let id = self.recipies.len();
                            self.recipies.push(new.clone());
                            self.crafting_special_firework_star = Some(id as u32);
                            return (Some(new), id as u32);
                        }
                    }
                }
                ShapelessRecipie::CraftingSpecialFireworkStarFade => {
                    let new = Recipie::Shapeless(ShapelessRecipie::CraftingSpecialFireworkStarFade);
                    match self.crafting_special_firework_star_fade {
                        Some(x) => (Some(new), x),
                        None => {
                            let id = self.recipies.len();
                            self.recipies.push(new.clone());
                            self.crafting_special_firework_star_fade = Some(id as u32);
                            return (Some(new), id as u32);
                        }
                    }
                }
                ShapelessRecipie::CraftingSpecialRepairitem => {
                    let new = Recipie::Shapeless(ShapelessRecipie::CraftingSpecialRepairitem);
                    match self.crafting_special_repairitem {
                        Some(x) => (Some(new), x),
                        None => {
                            let id = self.recipies.len();
                            self.recipies.push(new.clone());
                            self.crafting_special_repairitem = Some(id as u32);
                            return (Some(new), id as u32);
                        }
                    }
                }
                ShapelessRecipie::CraftingSpecialBannerDuplicate => {
                    let new = Recipie::Shapeless(ShapelessRecipie::CraftingSpecialBannerDuplicate);
                    match self.crafting_special_banner_duplicate {
                        Some(x) => (Some(new), x),
                        None => {
                            let id = self.recipies.len();
                            self.recipies.push(new.clone());
                            self.crafting_special_banner_duplicate = Some(id as u32);
                            return (Some(new), id as u32);
                        }
                    }
                }
                ShapelessRecipie::CraftingSpecialBannerAddPattern => {
                    let new = Recipie::Shapeless(ShapelessRecipie::CraftingSpecialBannerAddPattern);
                    match self.crafting_special_banner_add_pattern {
                        Some(x) => (Some(new), x),
                        None => {
                            let id = self.recipies.len();
                            self.recipies.push(new.clone());
                            self.crafting_special_banner_add_pattern = Some(id as u32);
                            return (Some(new), id as u32);
                        }
                    }
                }
                ShapelessRecipie::CraftingSpecialShieldDecoration => {
                    let new = Recipie::Shapeless(ShapelessRecipie::CraftingSpecialShieldDecoration);
                    match self.crafting_special_shielddecoration {
                        Some(x) => (Some(new), x),
                        None => {
                            let id = self.recipies.len();
                            self.recipies.push(new.clone());
                            self.crafting_special_shielddecoration = Some(id as u32);
                            return (Some(new), id as u32);
                        }
                    }
                }
                ShapelessRecipie::CraftingSpecialShulkerboxColoring => {
                    let new =
                        Recipie::Shapeless(ShapelessRecipie::CraftingSpecialShulkerboxColoring);
                    match self.crafting_special_shulkerboxcoloring {
                        Some(x) => (Some(new), x),
                        None => {
                            let id = self.recipies.len();
                            self.recipies.push(new.clone());
                            self.crafting_special_shulkerboxcoloring = Some(id as u32);
                            return (Some(new), id as u32);
                        }
                    }
                }
                ShapelessRecipie::CraftingSpecialSuspiciousStew => {
                    let new = Recipie::Shapeless(ShapelessRecipie::CraftingSpecialSuspiciousStew);
                    match self.crafting_special_suspiciousstew {
                        Some(x) => (Some(new), x),
                        None => {
                            let id = self.recipies.len();
                            self.recipies.push(new.clone());
                            self.crafting_special_suspiciousstew = Some(id as u32);
                            return (Some(new), id as u32);
                        }
                    }
                }
            },
            Recipie::Smelting(smelting) => {
                let id = self.smelting.get(&smelting.ingredient);

                match id {
                    Some(id) => {
                        let last = self.recipies.len();
                        self.recipies.push(recipie.clone());
                        self.recipies.swap(*id as usize, last);
                        let old = self.recipies.pop().unwrap();

                        return (Some(old), *id);
                    }
                    None => {
                        let id = self.recipies.len() as u32;
                        self.smelting.insert(smelting.ingredient, id);
                        self.recipies.push(recipie);
                        return (None, id);
                    }
                }
            }
            Recipie::BlastFurnace(blasting) => {
                let id = self.blastfurnace.get(&blasting.ingredient);

                match id {
                    Some(id) => {
                        let last = self.recipies.len();
                        self.recipies.push(recipie.clone());
                        self.recipies.swap(*id as usize, last);
                        let old = self.recipies.pop().unwrap();

                        return (Some(old), *id);
                    }
                    None => {
                        let id = self.recipies.len() as u32;
                        self.blastfurnace.insert(blasting.ingredient, id);
                        self.recipies.push(recipie);
                        return (None, id);
                    }
                }
            }
            Recipie::Smoking(smoking) => {
                let id = self.smoking.get(&smoking.ingredient);

                match id {
                    Some(id) => {
                        let last = self.recipies.len();
                        self.recipies.push(recipie.clone());
                        self.recipies.swap(*id as usize, last);
                        let old = self.recipies.pop().unwrap();

                        return (Some(old), *id);
                    }
                    None => {
                        let id = self.recipies.len() as u32;
                        self.smoking.insert(smoking.ingredient, id);
                        self.recipies.push(recipie);
                        return (None, id);
                    }
                }
            }
            Recipie::CampfireCooking(camping) => {
                let id = self.campfire.get(&camping.ingredient);

                match id {
                    Some(id) => {
                        let last = self.recipies.len();
                        self.recipies.push(recipie.clone());
                        self.recipies.swap(*id as usize, last);
                        let old = self.recipies.pop().unwrap();

                        return (Some(old), *id);
                    }
                    None => {
                        let id = self.recipies.len() as u32;
                        self.campfire.insert(camping.ingredient, id);
                        self.recipies.push(recipie);
                        return (None, id);
                    }
                }
            }
            Recipie::Smitting(smith) => {
                let index = (smith.Base, smith.Addition);
                let id = self.smitting.get(&index);

                match id {
                    Some(id) => {
                        let last = self.recipies.len();
                        self.recipies.push(recipie.clone());
                        self.recipies.swap(*id as usize, last);
                        let old = self.recipies.pop().unwrap();

                        return (Some(old), *id);
                    }
                    None => {
                        let id = self.recipies.len() as u32;
                        self.smitting.insert(index, id);
                        self.recipies.push(recipie);
                        return (None, id);
                    }
                }
            }
        }
    }

    fn craft(&self, shape: &Shaped) -> Option<ItemStack> {
        // Shaped first

        if let Some(_) = self.crafting_special_tipped_arrows {
            if let Some(s) = shape.craft_tipped_arrow() {
                return Some(s)
            }
        }

        if let Some(id) = self.shaped.get(&shape) {
            let recipie = &self[*id as RecipieId];

            if let Shaped(shaped_recipie) = recipie {

            } else {
                // There has been a mixup of id's and a non shaped recipie has been given the id of a shaped one. 
                assert!(false)
            }

            if let Some(s) = recipie.craft(shape) {
                
            }
        }




        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum Recipie {
    Shaped(ShapedRecipie),
    Shapeless(ShapelessRecipie),
    Smelting(SmeltingRecipie),
    BlastFurnace(BlastFurnaceRecipie),
    Smoking(SmokingRecipie),
    CampfireCooking(CampfireCookingRecipie),
    Smitting(SmithingRecipie),
}

#[derive(Clone, Debug)]
pub enum ShapelessRecipie {
    /// Used to group similar recipes together in the recipe book. Tag is present in recipe JSON.
    Shapeless {
        group: String,
        shape: Shapeless,
        result: ItemStack,
    },

    /// Recipe for dying leather armor
    /// [wiki](https://minecraft.fandom.com/wiki/Dye#Dyeing_armor)
    CraftingSpecialArmordye,

    /// Recipe for copying contents of written books
    /// [wiki](https://minecraft.fandom.com/wiki/Written_Book#Copying)
    CraftingSpecialBookcloning,

    /// Recipe for making firework rockets
    /// TODO: Figure out exactly what this means.
    CraftingSpecialFireworkRocket,

    /// Recipe for making firework stars
    /// TODO: Figure out exactly what this means.
    /// (is shapeless)
    CraftingSpecialFireworkStar,

    /// Recipe for making firework stars fade between multiple colors
    /// TODO: Figure out exactly what this means.
    CraftingSpecialFireworkStarFade,

    /// Recipe for repairing items via crafting
    /// Shapeless combine two identical items that are damaged
    /// the same way.
    CraftingSpecialRepairitem,

    /// Recipe for copying banner patterns
    CraftingSpecialBannerDuplicate,

    /// Recipe for adding patterns to banners
    CraftingSpecialBannerAddPattern,

    /// Recipe for applying a banner's pattern to a shield
    CraftingSpecialShieldDecoration,

    /// Recipe for recoloring a shulker box
    CraftingSpecialShulkerboxColoring,

    /// Recipie for crafting suspiciousStew
    CraftingSpecialSuspiciousStew,
}

/// Recipies one can craft in inventory or crafting table.
#[derive(Debug, Clone)]
enum ShapedRecipie {
    CraftingShaped {
        shape: Shaped,

        /// Used to group similar recipes together in the recipe book. Tag is present in recipe JSON.
        group: String,

        /// TODO: How and/or should we specify assosiated data like durabilaty and named items (through an anvil)?
        result: ItemStack,
    },

    /// Recipe for crafting tipped arrows
    /// (is shaped)
    CraftingTippedArrows,
}

impl ShapedRecipie {
    fn craft(&self, shape: &Shaped) -> Option<ItemStack> {}
}

#[derive(Debug, PartialEq, Clone)]
struct SmeltingRecipie {
    /// Used to group similar recipes together in the recipe book.
    group: String,
    ingredient: Item,
    result: ItemStack,
    experience: f32,
    cook_time_ticks: u32,
}

#[derive(Debug, PartialEq, Clone)]
struct BlastFurnaceRecipie {
    /// Used to group similar recipes together in the recipe book.
    group: String,
    ingredient: Item,
    result: ItemStack,
    experience: f32,
    cook_time_ticks: u32,
}

#[derive(Debug, PartialEq, Clone)]
struct SmokingRecipie {
    /// Used to group similar recipes together in the recipe book.
    group: String,
    ingredient: Item,
    result: ItemStack,
    experience: f32,
    cook_time_ticks: u32,
}

#[derive(Debug, PartialEq, Clone)]
struct CampfireCookingRecipie {
    /// Used to group similar recipes together in the recipe book.
    group: String,
    ingredient: Item,
    result: ItemStack,
    experience: f32,
    cook_time_ticks: u32,
}

#[derive(Debug, PartialEq, Clone)]
struct StonecuttingRecipie {
    group: String,
    ingredient: Item,
    result: ItemStack,
}

#[derive(Debug, PartialEq, Clone)]
struct SmithingRecipie {
    Base: Item,
    Addition: Item,
    Result: ItemStack,
}

#[cfg(test)]
mod test {
    use base::Item;

    use super::Shaped;

    #[test]
    fn shape_torch_into() {
        let torch_array = [
            [None, None, None],
            [None, None, Some(Item::Coal)],
            [None, None, Some(Item::Stick)],
        ];

        let shape: Shaped = torch_array.into();
        assert!(shape.width == 1);
        assert!(shape.height == 2);
        assert!(shape.items[0][0] == Some(Item::Coal));
        assert!(shape.items[1][0] == Some(Item::Stick));

        let torch_array = [
            [None, None, None],
            [None, Some(Item::Coal), None],
            [None, Some(Item::Stick), None],
        ];

        let shape: Shaped = torch_array.into();
        assert!(shape.width == 1);
        assert!(shape.height == 2);
        assert!(shape.items[0][0] == Some(Item::Coal));
        assert!(shape.items[1][0] == Some(Item::Stick));

        let torch_array = [
            [Some(Item::Coal), None, None],
            [Some(Item::Stick), None, None],
            [None, None, None],
        ];

        let shape: Shaped = torch_array.into();
        assert!(shape.width == 1);
        assert!(shape.height == 2);
        assert!(shape.items[0][0] == Some(Item::Coal));
        assert!(shape.items[1][0] == Some(Item::Stick));
    }
}
