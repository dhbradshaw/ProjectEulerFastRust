use std::cmp::max;
use std::cmp::min;

#[derive(Debug, PartialEq)]
pub struct IJ {
    pub i: usize,
    pub j: usize
}
impl IJ {
    pub fn new(i: usize, j: usize) -> IJ {
        IJ{i, j}
    }
    pub fn clone(&self) -> IJ {
        IJ::new(self.i, self.j)
    }
    pub fn down(&self) -> IJ {
        IJ::new(self.i + 1, self.j)
    }
    pub fn right(&self) -> IJ {
        IJ::new(self.i, self.j + 1)
    }
    pub fn ascend(&self) -> IJ {
        IJ::new(self.i - 1, self.j + 1)
    }
    pub fn descend(&self) -> IJ {
        IJ::new(self.i + 1, self.j + 1)
    }
}

pub struct AbstractSquareMatrix {
    pub order: usize
}

impl AbstractSquareMatrix {
    fn rows(&self) -> Vec<Vec<IJ>> {
        (0..self.order).map(|i| {
            (0..self.order).map(|j|{
                IJ{i, j}
            }).collect()
        }).collect()
    }
    fn columns(&self) -> Vec<Vec<IJ>> {
        (0..self.order).map(|i| {
            (0..self.order).map(|j|{
                IJ{i:j, j:i}
            }).collect()
        }).collect()
    }
    fn climbs(&self) -> Vec<Vec<IJ>> {
        let out = self.order;
        let scan_count: usize = 2 * out - 1;
        let mut first = IJ::new(0,0);
        let mut scans = Vec::new();
        for scan in 0..scan_count {
            println!("{:?}", first);
            let mut ij = first.clone();
            let mut scan_vec = vec![ij.clone()];
            while ij.j < (out - 1) {
                ij = ij.ascend();
                println!("{:?}", ij);
                scan_vec.push(ij.clone());
            }
            if first.i < (out - 1) {
                first = first.down();
            } else {
                first = first.right();
            }
        }
        return scans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn rows_2() {
        let asm = AbstractSquareMatrix{order:2};
        let rows = asm.rows();
        assert_eq!(
            rows.len(),
            2
        );
        assert_eq!(
            rows[0].len(),
            2
        );
        assert_eq!(rows[0][0], IJ{i: 0, j: 0});
        assert_eq!(rows[0][1], IJ{i: 0, j: 1});
    }
    #[test]
    fn cols_2() {
        let asm = AbstractSquareMatrix{order:2};
        let cols = asm.columns();
        assert_eq!(
            cols.len(),
            2
        );
        assert_eq!(
            cols[0].len(),
            2
        );
        assert_eq!(cols[0][0], IJ{i: 0, j: 0});
        assert_eq!(cols[0][1], IJ{i: 1, j: 0});
    }
    // #[test]
    // fn climbs_1() {
    //     let asm = AbstractSquareMatrix{order: 1};
    //     let climbs = asm.climbs();
    //     for scan in climbs {
    //         for item in scan {
    //             println!("i: {}, j: {}", item.i, item.j);
    //         }
    //         println!();
    //     }
    // }
    // #[test]
    // fn climbs_2() {
    //     let asm = AbstractSquareMatrix{order: 2};
    //     let climbs = asm.climbs();
    //     for scan in climbs {
    //         for item in scan {
    //             println!("i: {}, j: {}", item.i, item.j);
    //         }
    //         println!();
    //     }
    // }
    // #[test]
    // fn climbs_3() {
    //     let asm = AbstractSquareMatrix{order: 3};
    //     let climbs = asm.climbs();
    //     for scan in climbs {
    //         for item in scan {
    //             println!("i: {}, j: {}", item.i, item.j);
    //         }
    //         println!();
    //     }
    // }
    #[test]
    fn climbs_3() {
        let asm = AbstractSquareMatrix{order: 3};
        asm.climbs();
    }
}
