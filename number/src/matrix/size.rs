#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct MatrixDimension {
    pub width: usize,
    pub height: usize,
}
#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct MatrixIndex {
    pub row: usize,
    pub col: usize,
}
impl MatrixDimension {
    pub fn width(self) -> usize {
        self.width
    }
    pub fn height(self) -> usize {
        self.height
    }
    pub fn size(self) -> usize {
        self.width() * self.height()
    }
    #[must_use]
    pub fn transpose(self) -> Self {
        Self::new(self.height, self.width)
    }
    pub fn index(self, index: MatrixIndex) -> Option<usize> {
        let n = index.row() * self.width() + index.col();
        (index.col < self.width && index.row < self.height).then_some(n)
    }
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}
impl MatrixIndex {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    pub fn row(self) -> usize {
        self.row
    }
    pub fn col(self) -> usize {
        self.col
    }
    pub fn from(dim: MatrixDimension, index: usize) -> Option<Self> {
        let row = (index / dim.width()).strict_cast();
        let col = index.rem_euclid(dim.width()).strict_cast();
        (row < dim.height).then_some(Self::new(row, col))
    }
}
