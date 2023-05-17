//! CPU-culling, using Union-Find
use std::collections::HashSet;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct BlockPos {
    x: i32,
    y: i16,
    z: i32,
}

impl BlockPos {
    pub fn up(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        }
    }

    pub fn down(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
            z: self.z,
        }
    }

    pub fn east(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        }
    }

    pub fn west(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
            z: self.z,
        }
    }

    pub fn south(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        }
    }

    pub fn north(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }
}

pub fn exposed(all: HashSet<BlockPos>) -> HashSet<BlockPos> {
    // TODO: convert this into Union-Find (crate named `petgraph` has its implementation)
    let mut res = all;
    // returning true implies to be kept, false implies to be removed
    res.retain(|it| {
        let remove = all.contains(&it.up()) && all.contains(&it.down()) && all.contains(&it.west()) && all.contains(&it.east()) && all.contains(&it.north()) && all.contains(&it.south());
        !remove
    });

    res
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
        let up = x.up();
        let down = x.down();
        let north = x.north();
        let south = x.south();
        let east = x.east();
        let west = x.west();

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

        assert_eq!(x.up().down(), x);
    }

    #[test]
    fn south_and_north_are_dual() {
        // TODO: should be chosen randomly
        let x = BlockPos {
            x: 99,
            y: 91,
            z: 0
        };

        assert_eq!(x.south().north(), x);
    }

    #[test]
    fn west_and_east_are_dual() {
        // TODO: should be chosen randomly
        let x = BlockPos {
            x: 128,
            y: 15,
            z: 19
        };

        assert_eq!(x.west().east(), x);
    }
}