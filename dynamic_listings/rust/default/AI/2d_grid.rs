# #![allow(dead_code)]
#
# #[derive(Clone, Default)]
# struct Tile;
struct Grid2d {
    grid: Vec<Vec<Tile>>, // A 2D array of pointers
    width: usize,
    height: usize,
}

impl Grid2d {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            grid: vec![vec![Tile::default(); cols]; rows],
            width: rows,
            height: cols,
        }
    }

    pub fn cell(&self, row: usize, col: usize) -> Option<&Tile> {
        self.grid.get(row).and_then(|x| x.get(col))
    }

    pub fn adjacent_cells(&self, row: usize, col: usize) -> Vec<&Tile> {
        /* Returns a list of cells adjacent the ones we give
        REMEMBER: We index at 0 so the first row is 0, the last one is at
        "height - 1", same goes for columns */
        let mut to_return = Vec::new();
        if let Some(tile) = self.cell(row - 1, col) {
            // We are not on the first row, we can add the cell above
            to_return.push(tile);
        }
        if let Some(tile) = self.cell(row + 1, col) {
            // We are not on the last row, we can add the cell below
            to_return.push(tile);
        }
        if let Some(tile) = self.cell(row, col - 1) {
            // We are not on the first column, we can add the cell on the left
            to_return.push(tile);
        }
        if let Some(tile) = self.cell(row, col + 1) {
            // We are not on the last column, we can add the cell on the right
            to_return.push(tile);
        }
        /* If the checks went well, toReturn will have
        a list of the adjacent cells, if not it will be empty */
        to_return
    }
}
