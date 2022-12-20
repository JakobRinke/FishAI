use std::fmt;

use crate::util::{Element, Error, Result};

use super::Team;

// Ported from https://github.com/software-challenge/backend/blob/a3145a91749abb73ca5ffd426fd2a77d9a90967a/plugin/src/main/kotlin/sc/plugin2023/Field.kt

/// A field on the board.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Field {
    fish: usize,
    penguin: Option<Team>,
}

impl Default for Field {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Field {
    /// The empty field.
    pub const EMPTY: Self = Self { fish: 0, penguin: None };

    /// Creates a new field with the given fish.
    pub const fn with_fish(fish: usize) -> Self {
        Self { fish, ..Self::EMPTY }
    }

    /// Creates a new field with the given penguin.
    pub const fn with_penguin(team: Team) -> Self {
        Self { penguin: Some(team), ..Self::EMPTY }
    }

    /// Whether the field is empty.
    pub fn is_empty(self) -> bool { self.fish == 0 && self.penguin.is_none() }

    /// Whether the field is occupied by a penguin.
    pub fn is_occupied(self) -> bool { self.penguin.is_some() }

    /// The number of fish on this field.
    pub fn fish(self) -> usize { self.fish }

    /// The penguin on this field.
    pub fn penguin(self) -> Option<Team> { self.penguin }

    /// Replaces the fish on this field by a penguin, returning the number of fish.
    pub fn place(&mut self, team: Team) -> usize {
        let fish = self.fish;
        *self = Self::with_penguin(team);
        fish
    }
}

impl From<usize> for Field {
    fn from(fish: usize) -> Self {
        Self::with_fish(fish)
    }
}

impl From<Team> for Field {
    fn from(team: Team) -> Self {
        Self::with_penguin(team)
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(team) = self.penguin {
            write!(f, "{}", team.letter())
        } else {
            write!(f, "{}", self.fish)
        }
    }
}

impl TryFrom<char> for Field {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        if c.is_alphabetic() {
            let team = Team::with_letter(c).ok_or_else(|| Error::Custom(format!("Not a team: {}", c)))?;
            Ok(Field::with_penguin(team))
        } else if let Some(fish) = c.to_digit(10) {
            Ok(Field::with_fish(fish as usize))
        } else {
            Err(Error::Custom(format!("Invalid field: {}", c)))
        }
    }
}

impl TryFrom<&Element> for Field {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self {
            fish: elem.content().parse().unwrap_or(0),
            penguin: elem.content().parse().ok(),
        })
    }
}
