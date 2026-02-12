use crate::ui::tui::Colour;

#[derive(Clone, PartialEq)]
pub struct Cell {
    pub char: char,
    pub colour: Colour,
    pub bold: bool
}

pub struct FrameBuffer {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Cell>,
}

impl FrameBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        let size = width as usize * height as usize;

        Self {
            width,
            height,
            cells: vec![
                Cell {
                    char: ' ',
                    colour: Colour::White,
                    bold: false
                };
                size
            ],
        }
    }

    #[inline]
    fn loc(&self, x: u16, y: u16) -> usize {
        (y * self.width + x) as usize
    }

    pub fn put(&mut self, x: u16, y: u16, cell: Cell) {
        let i = self.loc(x, y);
        self.cells[i] = cell;
    }

    pub(crate) fn get(&self, x: u16, y: u16) -> Cell {
        self.cells[self.loc(x, y)].clone()
    }
}
