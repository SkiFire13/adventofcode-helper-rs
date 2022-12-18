#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid<T> {
    pub vec: Vec<T>,
    pub width: usize,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            width: 0,
        }
    }

    pub fn with_dimensions(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self::with_dimensions_init(width, height, |_, _| T::default())
    }

    pub fn with_dimensions_init<I>(width: usize, height: usize, mut init: I) -> Self
    where
        I: FnMut(usize, usize) -> T,
    {
        let vec = itertools::iproduct!(0..width, 0..height)
            .map(|(x, y)| init(x, y))
            .collect();
        Self { vec, width }
    }

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

    pub fn into_iter(self) -> impl Iterator<Item = ((usize, usize), T)> {
        self.iter_by_row().zip(self.vec)
    }

    pub fn iter_by_row(&self) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.w(), self.h());
        (0..h).flat_map(move |y| (0..w).map(move |x| (x, y)))
    }

    pub fn iter_by_col(&self) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.w(), self.h());
        (0..w).flat_map(move |x| (0..h).map(move |y| (x, y)))
    }

    pub fn to_set(&self, f: impl FnMut(&T, usize, usize) -> bool) -> GridSet {
        GridSet(self.map_ref(f))
    }
}

impl Grid<bool> {
    pub fn into_set(self) -> GridSet {
        GridSet(self)
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let (w, h) = (self.w(), self.h());
        self.get((x, y))
            .unwrap_or_else(|| index_out_of_bounds(w, h, x as isize, y as isize))
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let (w, h) = (self.w(), self.h());
        self.get_mut((x, y))
            .unwrap_or_else(|| index_out_of_bounds(w, h, x as isize, y as isize))
    }
}

impl<T> std::ops::Index<(isize, isize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        let (w, h) = (self.w(), self.h());
        self.iget((x, y))
            .unwrap_or_else(|| index_out_of_bounds(w, h, x, y))
    }
}

impl<T> std::ops::IndexMut<(isize, isize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        let (w, h) = (self.w(), self.h());
        self.iget_mut((x, y))
            .unwrap_or_else(|| index_out_of_bounds(w, h, x, y))
    }
}

#[cold]
fn index_out_of_bounds(w: usize, h: usize, x: isize, y: isize) -> ! {
    let (w, h) = (w as isize, h as isize);

    if x >= w {
        panic!("Index out of bounds: the width was {w} but x was {x}");
    }
    if y >= h {
        panic!("Index out of bounds: the height was {h} but y was {y}");
    }
    if x < 0 {
        panic!("Index out of bounds: x is {x} which is negative");
    }
    if y < 0 {
        panic!("Index out of bounds: y is {y} which is negative");
    }

    unreachable!();
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridSet(Grid<bool>);

impl GridSet {
    pub fn contains(&self, pos: (usize, usize)) -> bool {
        self[pos]
    }

    pub fn icontains(&self, pos: (isize, isize)) -> bool {
        self[pos]
    }

    pub fn insert(&mut self, pos: (usize, usize)) -> bool {
        !std::mem::replace(&mut self[pos], true)
    }

    pub fn remove(&mut self, pos: (usize, usize)) -> bool {
        std::mem::replace(&mut self[pos], false)
    }

    pub fn count(&self) -> usize {
        self.iter_set().count()
    }

    pub fn iter_set(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.iter().filter(|&(_, &set)| set).map(|(pos, _)| pos)
    }
}

impl std::ops::Deref for GridSet {
    type Target = Grid<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GridSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
