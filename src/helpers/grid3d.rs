#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid3D<T> {
    pub vec: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid3D<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn with_dimensions(width: usize, height: usize, depth: usize) -> Self
    where
        T: Default,
    {
        Self::with_dimensions_init(width, height, depth, |_, _, _| T::default())
    }

    pub fn with_dimensions_init<I>(width: usize, height: usize, depth: usize, mut init: I) -> Self
    where
        I: FnMut(usize, usize, usize) -> T,
    {
        let vec = itertools::iproduct!(0..width, 0..height, 0..depth)
            .map(|(x, y, z)| init(x, y, z))
            .collect();
        Self { vec, width, height }
    }

    pub fn h(&self) -> usize {
        self.height
    }

    pub fn w(&self) -> usize {
        self.width
    }

    pub fn d(&self) -> usize {
        self.depth()
    }

    pub fn depth(&self) -> usize {
        self.vec.len() / self.width / self.height
    }

    pub fn get(&self, (x, y, z): (usize, usize, usize)) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.vec.get(x + self.width * (y + self.height * z))
    }

    pub fn get_mut(&mut self, (x, y, z): (usize, usize, usize)) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.vec.get_mut(x + self.width * (y + self.height * z))
    }

    pub fn iget(&self, (x, y, z): (isize, isize, isize)) -> Option<&T> {
        if x < 0 || y < 0 || z < 0 {
            return None;
        }
        self.get((x as usize, y as usize, z as usize))
    }

    pub fn iget_mut(&mut self, (x, y, z): (isize, isize, isize)) -> Option<&mut T> {
        if x < 0 || y < 0 || z < 0 {
            return None;
        }
        self.get_mut((x as usize, y as usize, z as usize))
    }

    pub fn plus_neighbours(
        &self,
        (x, y, z): (usize, usize, usize),
    ) -> impl Iterator<Item = (usize, usize, usize)> {
        let iter = [
            (0, -1, 0),
            (-1, 0, 0),
            (0, 0, -1),
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
        ]
        .into_iter()
        .map(move |(dx, dy, dz)| (x as isize + dx, y as isize + dy, z as isize + dz));
        self.ifilter_in_bounds(iter)
    }

    pub fn ifilter_in_bounds(
        &self,
        iter: impl Iterator<Item = (isize, isize, isize)>,
    ) -> impl Iterator<Item = (usize, usize, usize)> {
        let (width, height, depth) = (self.w() as isize, self.h() as isize, self.d() as isize);
        iter.filter(move |&(x, y, z)| {
            0 <= x && x < width && 0 <= y && y < height && 0 <= z && z < depth
        })
        .map(|(x, y, z)| (x as usize, y as usize, z as usize))
    }

    pub fn map_ref<U>(&self, mut f: impl FnMut(&T, usize, usize, usize) -> U) -> Grid3D<U> {
        Grid3D {
            vec: self
                .vec
                .iter()
                .enumerate()
                .map(|(idx, elm)| {
                    let (x, rest) = (idx % self.width, idx / self.width);
                    let (y, z) = (rest % self.height, rest / self.height);
                    f(elm, x, y, z)
                })
                .collect(),
            width: self.width,
            height: self.height,
        }
    }

    pub fn to_set(&self, f: impl FnMut(&T, usize, usize, usize) -> bool) -> GridSet3D {
        GridSet3D(self.map_ref(f))
    }
}

impl Grid3D<bool> {
    pub fn into_set(self) -> GridSet3D {
        GridSet3D(self)
    }
}

impl<T> std::ops::Index<(usize, usize, usize)> for Grid3D<T> {
    type Output = T;
    fn index(&self, (x, y, z): (usize, usize, usize)) -> &Self::Output {
        let (w, h, d) = (self.w(), self.h(), self.d());
        self.get((x, y, z))
            .unwrap_or_else(|| index_out_of_bounds(w, h, d, x as isize, y as isize, z as isize))
    }
}

impl<T> std::ops::IndexMut<(usize, usize, usize)> for Grid3D<T> {
    fn index_mut(&mut self, (x, y, z): (usize, usize, usize)) -> &mut Self::Output {
        let (w, h, d) = (self.w(), self.h(), self.d());
        self.get_mut((x, y, z))
            .unwrap_or_else(|| index_out_of_bounds(w, h, d, x as isize, y as isize, z as isize))
    }
}

impl<T> std::ops::Index<(isize, isize, isize)> for Grid3D<T> {
    type Output = T;
    fn index(&self, (x, y, z): (isize, isize, isize)) -> &Self::Output {
        let (w, h, d) = (self.w(), self.h(), self.d());
        self.iget((x, y, z))
            .unwrap_or_else(|| index_out_of_bounds(w, h, d, x, y, z))
    }
}

impl<T> std::ops::IndexMut<(isize, isize, isize)> for Grid3D<T> {
    fn index_mut(&mut self, (x, y, z): (isize, isize, isize)) -> &mut Self::Output {
        let (w, h, d) = (self.w(), self.h(), self.d());
        self.iget_mut((x, y, z))
            .unwrap_or_else(|| index_out_of_bounds(w, h, d, x, y, z))
    }
}

#[cold]
fn index_out_of_bounds(w: usize, h: usize, d: usize, x: isize, y: isize, z: isize) -> ! {
    let (w, h, d) = (w as isize, h as isize, d as isize);

    if x >= w {
        panic!("Index out of bounds: the width was {w} but x was {x}");
    }
    if y >= h {
        panic!("Index out of bounds: the height was {h} but y was {y}");
    }
    if z >= d {
        panic!("Index out of bounds: the depth was {d} but z was {z}");
    }
    if x < 0 {
        panic!("Index out of bounds: x is {x} which is negative");
    }
    if y < 0 {
        panic!("Index out of bounds: y is {y} which is negative");
    }
    if z < 0 {
        panic!("Index out of bounds: z is {z} which is negative");
    }

    unreachable!();
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridSet3D(Grid3D<bool>);

impl GridSet3D {
    pub fn contains(&self, pos: (usize, usize, usize)) -> bool {
        self[pos]
    }

    pub fn icontains(&self, pos: (isize, isize, isize)) -> bool {
        self[pos]
    }

    pub fn insert(&mut self, pos: (usize, usize, usize)) -> bool {
        !std::mem::replace(&mut self[pos], true)
    }

    pub fn remove(&mut self, pos: (usize, usize, usize)) -> bool {
        std::mem::replace(&mut self[pos], false)
    }
}

impl std::ops::Deref for GridSet3D {
    type Target = Grid3D<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GridSet3D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
