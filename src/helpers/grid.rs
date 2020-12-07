#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid<T> {
    pub vec: Vec<T>,
    pub width: usize,
}

impl<T> Grid<T> {
    pub fn height(&self) -> usize {
        self.vec.len() / self.width
    }
    pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        if x >= self.width {
            return None;
        }
        self.vec.get(x + self.width * y)
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.vec[x + self.width * y]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.vec[x + self.width * y]
    }
}
