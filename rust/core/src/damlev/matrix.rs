
pub struct DistMatrix {
    size: usize,
    raw: Vec<usize>,
}


impl DistMatrix {
    pub fn new(size: usize) -> Self {
        let raw = vec![0; size * size];
        let mut matrix = Self { size, raw };
        matrix.init();
        matrix
    }

    pub fn grow(&mut self, size: usize) {
        if size <= self.size {
            return;
        }
        let size = size + size / 2;
        self.raw.resize(size * size, 0);
        self.size = size;
        self.init();
    }

    pub fn init(&mut self) {
        if self.size == 0 { return; }
        unsafe {
            for i in 0..self.size {
                self.set_unchecked(i, 0, self.size);
                self.set_unchecked(0, i, self.size);
            }
            for i in 1..self.size {
                self.set_unchecked(i, 1, i - 1);
                self.set_unchecked(1, i, i - 1);
            }
        }
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, i: usize, j: usize) -> usize {
        *self.raw.get_unchecked(i * self.size + j)
    }

    #[inline]
    pub unsafe fn set_unchecked(&mut self, i: usize, j: usize, val: usize) {
        *self.raw.get_unchecked_mut(i * self.size + j) = val;
    }

    #[inline]
    pub fn get(&self, i: usize, j: usize) -> usize {
        self.raw[i * self.size + j]
    }
}
