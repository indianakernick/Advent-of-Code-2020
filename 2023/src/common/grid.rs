#[derive(Clone, Copy)]
pub struct Grid<'a, Tile = u8> {
    tiles: &'a [Tile],
    stride: usize,
    width: i32,
    height: i32,
}

impl<'a, Tile> Grid<'a, Tile> {
    pub fn from_input(input: &str) -> Self {
        let bytes = input.as_bytes();
        let width = bytes.iter().position(|b| *b == b'\n').unwrap();
        let stride = width + 1;
        let height = (bytes.len() + 1) / stride;
        let tiles = unsafe { std::mem::transmute::<&[u8], &[Tile]>(bytes) };

        Self {
            tiles,
            stride,
            width: width as i32,
            height: height as i32,
        }
    }

    pub fn valid(&self, (x, y): (i32, i32)) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    pub fn index_to_pos(&self, index: usize) -> (i32, i32) {
        ((index % self.stride) as i32, (index / self.stride) as i32)
    }

    pub fn pos_to_index(&self, (x, y): (i32, i32)) -> usize {
        y as usize * self.stride + x as usize
    }

    pub fn get_stride(&self) -> usize {
        self.stride
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}

impl<'a, Tile> Grid<'a, Tile>
    where Tile: PartialEq
{
    pub fn pos_of(&self, tile: Tile) -> Option<(i32, i32)> {
        self.tiles
            .iter()
            .position(|t| *t == tile)
            .map(|index| self.index_to_pos(index))
    }
}

impl<'a, Tile> Grid<'a, Tile>
    where Tile: Copy
{
    pub fn get(&self, pos: (i32, i32)) -> Tile {
        debug_assert!(self.valid(pos));
        self.tiles[self.pos_to_index(pos)]
    }

    pub fn to_vec(&self) -> Vec<Tile> {
        self.tiles.to_vec()
    }
}
