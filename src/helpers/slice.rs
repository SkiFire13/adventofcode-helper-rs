pub trait SliceExt<T> {
    fn get2_mut(&mut self, i: usize, j: usize) -> (&mut T, &mut T);
}

impl<T> SliceExt<T> for [T] {
    fn get2_mut(&mut self, i: usize, j: usize) -> (&mut T, &mut T) {
        assert!(i != j && i < self.len() && j < self.len());
        if i < j {
            let (start, wj) = self.split_at_mut(j);
            (&mut start[i], &mut wj[0])
        } else {
            let (start, wi) = self.split_at_mut(i);
            (&mut wi[0], &mut start[j])
        }
    }
}
