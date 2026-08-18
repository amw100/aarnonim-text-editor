use crate::editor::terminal::Position;

#[derive(Clone, Copy, Default)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

impl From<Location> for Position {
    fn from(loc: Location) -> Self {
        Self {
            x: loc.column,
            y: loc.row,
        }
    }
}

impl Location {
    pub fn subtract(&self, other_location: &Self) -> Self {
        Self {
            column: self.column.saturating_sub(other_location.column),
            row: self.row.saturating_sub(other_location.row),
        }
    }
}
