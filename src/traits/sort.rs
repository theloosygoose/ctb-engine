use crate::person::previews::{OffPreview, DefPreview};
pub trait Sortable {
    fn sort(&mut self);

    fn rev_sort(&mut self);
}

impl Sortable for Vec<OffPreview> {
    fn sort(&mut self) {
        let mut new_len: usize;

        let mut len = self.len();

        loop {
            new_len = 0;

            for i in 1..len {
                if self[i - 1].1 < self[i].1 {
                    self.swap(i - 1, i);
                    new_len = i;
                }
            }
            if new_len == 0 {
                break;
            }

            len = new_len;
        }
    }

    fn rev_sort(&mut self) {
        let mut new_len: usize;

        let mut len = self.len();

        loop {
            new_len = 0;

            for i in 1..len {
                if self[i - 1].1 > self[i].1 {
                    self.swap(i - 1, i);
                    new_len = i;
                }
            }
            if new_len == 0 {
                break;
            }

            len = new_len;
        }
    }
}

impl Sortable for Vec<DefPreview> {
    fn sort(&mut self) {
        let mut new_len: usize;

        let mut len = self.len();

        loop {
            new_len = 0;

            for i in 1..len {
                if self[i - 1].1 < self[i].1 {
                    self.swap(i - 1, i);
                    new_len = i;
                }
            }
            if new_len == 0 {
                break;
            }

            len = new_len;
        }
    }

    fn rev_sort(&mut self) {
        let mut new_len: usize;

        let mut len = self.len();

        loop {
            new_len = 0;

            for i in 1..len {
                if self[i - 1].1 > self[i].1 {
                    self.swap(i - 1, i);
                    new_len = i;
                }
            }
            if new_len == 0 {
                break;
            }

            len = new_len;
        }
    }
}
