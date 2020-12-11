#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid<T> {
    pub vec: Vec<T>,
    pub width: usize,
}

impl<T> Grid<T> {
    pub fn from_input_chars(input: &str, mut f: impl FnMut(char, usize, usize) -> T) -> Self {
        Self {
            vec: input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (c, x, y)))
                .map(move |(c, x, y)| f(c, x, y))
                .collect(),
            width: input.lines().next().unwrap().len(),
        }
    }

    pub fn height(&self) -> usize {
        self.vec.len() / self.width
    }
    pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        if x >= self.width {
            return None;
        }
        self.vec.get(x + self.width * y)
    }
    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut T> {
        if x >= self.width {
            return None;
        }
        self.vec.get_mut(x + self.width * y)
    }
    pub fn iget(&self, (x, y): (isize, isize)) -> Option<&T> {
        if x < 0 || y < 0 {
            return None;
        }
        self.get((x as usize, y as usize))
    }
    pub fn iget_mut(&mut self, (x, y): (isize, isize)) -> Option<&mut T> {
        if x < 0 || y < 0 {
            return None;
        }
        self.get_mut((x as usize, y as usize))
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.get((x, y)).expect("Index out of bounds")
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.get_mut((x, y)).expect("Index out of bounds")
    }
}

impl<T> std::ops::Index<(isize, isize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        self.iget((x, y)).expect("Index out of bounds")
    }
}

impl<T> std::ops::IndexMut<(isize, isize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        self.iget_mut((x, y)).expect("Index out of bounds")
    }
}
