use std::{ops::{Index, IndexMut}, fmt, str::FromStr};

use arrayvec::ArrayVec;

use crate::util::{Element, Error, Result};

use super::{Field, BOARD_FIELDS, Vec2, Direct, BOARD_SIZE, Move, Doubled, Team};

// Ported from https://github.com/software-challenge/backend/blob/a3145a91749abb73ca5ffd426fd2a77d9a90967a/plugin/src/main/kotlin/sc/plugin2023/Board.kt

/// The 8x8 game board, a two-dimensional grid of ice floes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    fields: [Field; BOARD_FIELDS],
}

impl Default for Board {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Board {
    /// The empty board.
    pub const EMPTY: Self = Self { fields: [Field::EMPTY; BOARD_FIELDS] };

    /// Creates a new board with the given fields.
    pub const fn new(fields: [Field; BOARD_FIELDS]) -> Self {
        Self { fields }
    }

    /// Checks whether the given coordinates are in bounds.
    pub fn in_bounds(coords: impl Into<Vec2<Doubled>>) -> bool {
        let doubled: Vec2<Doubled> = coords.into();
        let direct: Vec2<Direct> = doubled.into();
        // We need to check doubled.x separately, since (-1, ...) would otherwise get mapped to (0, ...)
        // I.e. we also have to be careful not to convert back to direct coordinates anywhere in the
        // 'Into-chain' since we lose information there and may cause (-1, ...) to incorrectly pass the bounds-check.
        doubled.x >= 0
            && direct.x >= 0
            && direct.x < BOARD_SIZE as i32
            && direct.y >= 0
            && direct.y < BOARD_SIZE as i32
    }

    /// Converts coordinates to an index.
    fn index_for(coords: impl Into<Vec2<Doubled>>) -> usize {
        let direct: Vec2<Direct> = coords.into().into();
        direct.y as usize * BOARD_SIZE + direct.x as usize
    }

    /// Converts an index to coordinates.
    fn coords_for(index: usize) -> Vec2<Direct> {
        Vec2::new((index % BOARD_SIZE) as i32, (index / BOARD_SIZE) as i32)
    }

    /// Optionally fetches the field at the given position.
    pub fn get(&self, coords: impl Into<Vec2<Doubled>> + Copy) -> Option<Field> {
        if Self::in_bounds(coords.into()) {
            Some(self[coords])
        } else {
            None
        }
    }

    /// Fetches the possible moves from a given position.
    pub fn possible_moves_from<'a>(&'a self, coords: impl Into<Vec2<Doubled>>) -> impl Iterator<Item=Move> + 'a {
        let doubled: Vec2<Doubled> = coords.into();
        Vec2::<Doubled>::DIRECTIONS
            .into_iter()
            .flat_map(move |v| (1..BOARD_SIZE as i32)
                .map(move |n| Move::sliding(doubled, n * v))
                .take_while(|c| self.get(c.to()).unwrap_or_default().fish() > 0))
    }

    /// Fetches an iterator over the fields with coordinates.
    pub fn fields(&self) -> impl Iterator<Item=(Vec2<Doubled>, Field)> {
        self.fields
            .into_iter()
            .enumerate()
            .map(|(i, f)| (Self::coords_for(i).into(), f))
    }

    /// Fetches the penguins on the board.
    pub fn penguins(&self) -> impl Iterator<Item=(Vec2<Doubled>, Team)> {
        self.fields()
            .filter_map(|(c, f)| f.penguin().map(|p| (c, p)))
    }
}

impl<V> Index<V> for Board where V: Copy + Into<Vec2<Doubled>> {
    type Output = Field;

    fn index(&self, index: V) -> &Field {
        &self.fields[Self::index_for(index)]
    }
}

impl<V> IndexMut<V> for Board where V: Copy + Into<Vec2<Doubled>> {
    fn index_mut(&mut self, index: V) -> &mut Field {
        &mut self.fields[Self::index_for(index)]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                write!(f, "{}", self.fields[y * BOARD_SIZE + x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            fields: s.lines()
                .filter(|l| !l.is_empty())
                .flat_map(|l| l.chars().map(|c| c.try_into()))
                .collect::<Result<ArrayVec<Field, BOARD_FIELDS>>>()?
                .into_inner()
                .map_err(|e| Error::from(format!("Board has wrong number of fields: {:?}", e)))?
        })
    }
}

impl TryFrom<&Element> for Board {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self {
            fields: elem.childs_by_name("list")
                .flat_map(|c| c.childs_by_name("field").map(|c| c.try_into()))
                .collect::<Result<ArrayVec<Field, BOARD_FIELDS>>>()?
                .into_inner()
                .map_err(|e| Error::from(format!("Board has wrong number of fields: {:?}", e)))?
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use indoc::indoc;

    use crate::{util::Element, game::{Board, Team, Vec2, Field, Direct}};

    #[test]
    fn test_from_xml() {
        assert_eq!(Board::try_from(&Element::from_str(indoc! {r#"
            <board>
                <list>
                    <field>3</field>
                    <field>2</field>
                    <field>1</field>
                    <field>1</field>
                    <field>4</field>
                    <field>3</field>
                    <field>2</field>
                    <field>3</field>
                </list>
                <list>
                    <field>3</field>
                    <field>2</field>
                    <field>2</field>
                    <field>3</field>
                    <field>1</field>
                    <field>1</field>
                    <field>2</field>
                    <field>1</field>
                </list>
                <list>
                    <field>1</field>
                    <field>2</field>
                    <field>2</field>
                    <field>1</field>
                    <field>1</field>
                    <field>2</field>
                    <field>1</field>
                    <field>1</field>
                </list>
                <list>
                    <field>2</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                </list>
                <list>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>2</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                </list>
                <list>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>ONE</field>
                    <field>1</field>
                </list>
                <list>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                </list>
                <list>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                </list>
            </board>
        "#}).unwrap()).unwrap(), Board::new([
            3.into(), 2.into(), 1.into(), 1.into(), 4.into(), 3.into(), 2.into(), 3.into(),
            3.into(), 2.into(), 2.into(), 3.into(), 1.into(), 1.into(), 2.into(), 1.into(),
            1.into(), 2.into(), 2.into(), 1.into(), 1.into(), 2.into(), 1.into(), 1.into(),
            2.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(),
            1.into(), 1.into(), 1.into(), 1.into(), 2.into(), 1.into(), 1.into(), 1.into(),
            1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), Team::One.into(), 1.into(),
            1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(),
            1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(),
        ]));
    }

    #[test]
    fn test_display_roundtrip() {
        let mut board = Board::EMPTY;
        board[Vec2::<Direct>::new(2, 2)] = Field::with_fish(3);
        board[Vec2::<Direct>::new(1, 0)] = Field::with_penguin(Team::One);
        board[Vec2::<Direct>::new(1, 1)] = Field::with_penguin(Team::Two);

        assert_eq!(board.to_string(), indoc! {r#"
            0R000000
            0B000000
            00300000
            00000000
            00000000
            00000000
            00000000
            00000000
        "#});

        assert_eq!(board.to_string().parse::<Board>().unwrap(), board);
    }
}
