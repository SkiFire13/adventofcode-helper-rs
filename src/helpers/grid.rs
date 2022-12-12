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

    pub fn h(&self) -> usize {
        self.height()
    }

    pub fn w(&self) -> usize {
        self.width
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

    pub fn plus_neighbours(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let iter = [(0, -1), (-1, 0), (1, 0), (0, 1)]
            .into_iter()
            .map(move |(dx, dy)| (x as isize + dx, y as isize + dy));
        self.ifilter_in_bounds(iter)
    }

    pub fn square_neighbours(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        let iter = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .into_iter()
        .map(move |(dx, dy)| (x as isize + dx, y as isize + dy));
        self.ifilter_in_bounds(iter)
    }

    pub fn ifilter_in_bounds(
        &self,
        iter: impl Iterator<Item = (isize, isize)>,
    ) -> impl Iterator<Item = (usize, usize)> {
        let (width, height) = (self.width as isize, self.height() as isize);
        iter.filter(move |&(x, y)| 0 <= x && x < width && 0 <= y && y < height)
            .map(|(x, y)| (x as usize, y as usize))
    }

    pub fn map_ref<U>(&self, mut f: impl FnMut(&T, usize, usize) -> U) -> Grid<U> {
        Grid {
            vec: self
                .vec
                .iter()
                .enumerate()
                .map(|(idx, elm)| {
                    let (x, y) = (idx % self.width, idx / self.width);
                    f(elm, x, y)
                })
                .collect(),
            width: self.width,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.vec
            .chunks_exact(self.width)
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, val)| ((x, y), val)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.vec
            .chunks_exact_mut(self.width)
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter_mut()
                    .enumerate()
                    .map(move |(x, val)| ((x, y), val))
            })
    }

    pub fn iter_by_row(&self) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.w(), self.h());
        (0..h).flat_map(move |y| (0..w).map(move |x| (x, y)))
    }

    pub fn iter_by_col(&self) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.w(), self.h());
        (0..w).flat_map(move |x| (0..h).map(move |y| (x, y)))
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
