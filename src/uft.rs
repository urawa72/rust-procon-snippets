use cargo_snippet::snippet;

#[snippet("UFT")]
pub struct UFT {
    pub par: Vec<usize>,
    pub rank: Vec<usize>,
}

#[snippet("UFT")]
impl UFT {
    pub fn new(n: usize) -> Self {
        UFT {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let p = self.par[x];
            let pp = self.root(p);
            self.par[x] = pp;
            pp
        }
    }

    pub fn merge(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.par[x] = y;
        } else {
            self.par[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
}

#[test]
fn test_uft() {
    let mut uft = UFT::new(4);
    assert_eq!(0, uft.root(0));
    assert_eq!(1, uft.root(1));
    assert_eq!(2, uft.root(2));
    assert_eq!(3, uft.root(3));
    uft.merge(1, 3);
    assert_eq!(0, uft.root(0));
    assert_eq!(1, uft.root(1));
    assert_eq!(2, uft.root(2));
    assert_eq!(1, uft.root(3));
    uft.merge(2, 3);
    assert_eq!(0, uft.root(0));
    assert_eq!(1, uft.root(1));
    assert_eq!(1, uft.root(2));
    assert_eq!(1, uft.root(3));
    uft.merge(0, 2);
    assert_eq!(1, uft.root(0));
    assert_eq!(1, uft.root(1));
    assert_eq!(1, uft.root(2));
    assert_eq!(1, uft.root(3));
    uft.merge(0, 1);
    assert_eq!(1, uft.root(0));
    assert_eq!(1, uft.root(1));
    assert_eq!(1, uft.root(2));
    assert_eq!(1, uft.root(3));
}
