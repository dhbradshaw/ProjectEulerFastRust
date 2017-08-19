use std::cmp::max;
use std::cmp::min;

pub struct AbstractMatrix {
    pub order: usize,
}

impl AbstractMatrix {
    pub fn new(order: usize) -> AbstractMatrix {
        AbstractMatrix{order}
    }
    pub fn rows(&self) -> Vec<Vec<(usize, usize)>> {
        (0..self.order).map(
            |i| (0..self.order).map(|j| (i, j)).collect()
        ).collect()
    }
    pub fn columns(&self) -> Vec<Vec<(usize, usize)>> {
        (0..self.order).map(
            |j| (0..self.order).map(|i| (i, j)).collect()
        ).collect()
    }

    fn climb(&self, sum: usize) -> Vec<(usize, usize)> {
        let largest = self.order - 1;
        let smallest = 0;

        let mut i = min(largest, sum);
        let mut j = sum - i;
        let mut v = Vec::new();
        loop {
            v.push((i, j));
            if i <= smallest || j >= largest {
                break;
            }
            i = i - 1;
            j = j + 1;
        }
        v
    }
    pub fn climbs(&self) -> Vec<Vec<(usize, usize)>> {
        let largest = self.order - 1;
        (0..2 * largest + 1).map(|sum| self.climb(sum)).collect()
    }
    fn descend(&self, diff: i32) -> Vec<(usize, usize)> {
        let largest = self.order - 1;
        let smallest = 0;

        let mut i = max(smallest as i32, diff) as usize;
        let mut j = ((i as i32) - diff) as usize;
        let mut v = Vec::new();
        loop {
            v.push((i, j));
            if j >= largest || i >= largest {
                break;
            }
            i = i + 1;
            j = j + 1;
        }
        v
    }
    pub fn descends(&self) -> Vec<Vec<(usize, usize)>> {
        let largest = (self.order - 1) as i32;
        (-largest..largest + 1).map(|diff| self.descend(diff)).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn climb() {
        let matrix1 = AbstractMatrix::new(1);
        assert_eq!(matrix1.climb(0), vec![(0, 0)]);

        let matrix2 = AbstractMatrix::new(2);
        assert_eq!(matrix2.climb(0), vec![(0, 0)]);
        assert_eq!(matrix2.climb(1), vec![(1, 0), (0, 1)]);
        assert_eq!(matrix2.climb(2), vec![(1, 1)]);
    }
    #[test]
    fn climbs() {
        let matrix1 = AbstractMatrix::new(1);
        assert_eq!(matrix1.rows(), vec![vec![(0, 0)]]);

        let matrix2 = AbstractMatrix::new(2);
        assert_eq!(
            matrix2.climbs(),
            vec![
                vec![(0, 0)],
                vec![(1, 0), (0, 1)],
                vec![(1, 1)],
            ]
        );
    }
    #[test]
    fn descend() {
        let matrix1 = AbstractMatrix::new(1);
        assert_eq!(matrix1.descend(0), vec![(0, 0)]);
        let matrix2 = AbstractMatrix::new(2);
        assert_eq!(matrix2.descend(-1), vec![(0, 1)]);
        assert_eq!(matrix2.descend(0), vec![(0, 0), (1, 1)]);
        assert_eq!(matrix2.descend(1), vec![(1, 0)]);
    }
    #[test]
    fn descends() {
        let matrix1 = AbstractMatrix::new(1);
        assert_eq!(matrix1.descends(), vec![vec![(0, 0)]]);

        let matrix2 = AbstractMatrix::new(2);
        assert_eq!(
            matrix2.descends(),
            vec![
                vec![(0, 1)],
                vec![(0, 0), (1, 1)],
                vec![(1, 0)],
            ]
        );
    }
    #[test]
    fn rows() {
        let matrix1 = AbstractMatrix::new(1);
        assert_eq!(matrix1.rows(), vec![vec![(0, 0)]]);
        let matrix2 = AbstractMatrix::new(2);
        assert_eq!(matrix2.rows(), vec![vec![(0, 0), (0, 1)], vec![(1, 0), (1, 1)]]);
    }
    #[test]
    fn columns() {
        let matrix1 = AbstractMatrix::new(1);
        assert_eq!(matrix1.columns(), vec![vec![(0, 0)]]);
        let matrix2 = AbstractMatrix::new(2);
        assert_eq!(matrix2.columns(), vec![vec![(0, 0), (1, 0)], vec![(0, 1), (1, 1)]]);
    }
}
