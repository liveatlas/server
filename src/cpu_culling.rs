//! CPU-culling, using Union-Find
use std::collections::HashSet;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct BlockPos {
    x: i32,
    y: i16,
    z: i32,
}

// NOTE for IntelliJ user: "method is private" on `overflowing_add` method is false-positive
impl BlockPos {
    pub fn up(self) -> Option<Self> {
        let (y, carry) = self.y.overflowing_add(1);
        (!carry).then_some(y).map(|y| {
            Self {
                x: self.x,
                y,
                z: self.z,
            }
        })
    }

    pub fn down(self) -> Option<Self> {
        let (y, carry) = self.y.overflowing_sub(1);
        (!carry).then_some(y).map(|y| {
            Self {
                x: self.x,
                y,
                z: self.z,
            }
        })
    }

    pub fn east(self) -> Option<Self> {
        let (x, carry) = self.x.overflowing_add(1);
        (!carry).then_some(x).map(|x| {
            Self {
                x,
                y: self.y,
                z: self.z,
            }
        })
    }

    pub fn west(self) -> Option<Self> {
        let (x, carry) = self.x.overflowing_sub(1);
        (!carry).then_some(x).map(|x| {
            Self {
                x,
                y: self.y,
                z: self.z,
            }
        })
    }

    pub fn south(self) -> Option<Self> {
        let (z, carry) = self.z.overflowing_add(1);
        (!carry).then_some(z).map(|z| {
            Self {
                x: self.x,
                y: self.y,
                z,
            }
        })
    }

    pub fn north(self) -> Option<Self> {
        let (z, carry) = self.z.overflowing_sub(1);
        (!carry).then_some(z).map(|z| {
            Self {
                x: self.x,
                y: self.y,
                z,
            }
        })

    }
}

pub fn exposed(res: HashSet<BlockPos>) -> HashSet<BlockPos> {
    // TODO: convert this into Union-Find (crate named `petgraph` has its implementation)
    // returning true implies to be kept, false implies to be removed
    res.iter().filter(|it| {
        let remove = it.up().map(|p| res.contains(&p)).unwrap_or(false)
            && it.down().map(|p| res.contains(&p)).unwrap_or(false)
            && it.west().map(|p| res.contains(&p)).unwrap_or(false)
            && it.east().map(|p| res.contains(&p)).unwrap_or(false)
            && it.north().map(|p| res.contains(&p)).unwrap_or(false)
            && it.south().map(|p| res.contains(&p)).unwrap_or(false);
        !remove
    }).copied().collect()
}

#[cfg(test)]
mod tests {
    use maplit::hashset;
    use crate::cpu_culling::{BlockPos, exposed};

    #[test]
    fn empty_returns_empty() {
        assert!(exposed(hashset![]).is_empty());
    }

    #[test]
    fn single_returns_single() {
        let x = BlockPos {
            x: 42,
            y: 35,
            z: 96,
        };

        let result = exposed(hashset![x]);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&x));
    }

    #[test]
    fn three_dimension_cross() {
        let x = BlockPos {
            x: 21,
            y: 34,
            z: 59
        };
        let up = x.up().unwrap();
        let down = x.down().unwrap();
        let north = x.north().unwrap();
        let south = x.south().unwrap();
        let east = x.east().unwrap();
        let west = x.west().unwrap();

        let result = exposed(hashset![x, up, down, north, south, east, west]);
        assert_eq!(result.len(), 6);
        for x in &[up, down, north, south, east, west] {
            assert!(result.contains(x));
        }
    }

    #[test]
    fn up_and_down_are_dual() {
        // TODO: should be chosen randomly
        let x = BlockPos {
            x: 39,
            y: 26,
            z: 16
        };

        assert_eq!(x.up().unwrap().down().unwrap(), x);
    }

    #[test]
    fn south_and_north_are_dual() {
        // TODO: should be chosen randomly
        let x = BlockPos {
            x: 99,
            y: 91,
            z: 0
        };

        assert_eq!(x.south().unwrap().north().unwrap(), x);
    }

    #[test]
    fn west_and_east_are_dual() {
        // TODO: should be chosen randomly
        let x = BlockPos {
            x: 128,
            y: 15,
            z: 19
        };

        assert_eq!(x.west().unwrap().east().unwrap(), x);
    }
}