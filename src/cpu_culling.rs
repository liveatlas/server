//! CPU-culling, using Union-Find
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct BlockPos {
    repr: u64,
}

// NOTE for IntelliJ user: "method is private" on `overflowing_add` method is false-positive
impl BlockPos {
    pub fn from_xyz(x: i32, y: i16, z: i32) -> Option<Self> {
        const X_ABS_MAX: u64 = 0x3FF_FFFF;
        const Y_ABS_MAX: u16 = 0x000_0FFF;
        const Z_ABS_MAX: u64 = 0x3FF_FFFF;

        if x.abs() as u64 <= X_ABS_MAX && y.abs() as u16 <= Y_ABS_MAX && z.abs() as u64 <= Z_ABS_MAX {
            let repr = ((((x as i64) & 0x3FFFFFF) << 38) | (((z as i64) & 0x3FFFFFF) << 12) | ((y as i64) & 0xFFF)) as u64;
            Some(Self {
                repr
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn x(self) -> i32 {
        (self.repr >> 38) as i32
    }

    #[inline]
    pub fn y(self) -> i16 {
        ((self.repr << 52) >> 52) as i16
    }

    #[inline]
    pub fn z(self) -> i32 {
        ((self.repr << 26) >> 38) as i32
    }

    pub fn up(self) -> Option<Self> {
        let (y, carry) = self.y().overflowing_add(1);
        (!carry).then_some(y).and_then(|y| {
            Self::from_xyz(self.x(), y, self.z())
        })
    }

    pub fn down(self) -> Option<Self> {
        let (y, carry) = self.y().overflowing_sub(1);
        (!carry).then_some(y).and_then(|y| {
            Self::from_xyz(self.x(), y, self.z())
        })
    }

    pub fn east(self) -> Option<Self> {
        let (x, carry) = self.x().overflowing_add(1);
        (!carry).then_some(x).and_then(|x| {
            Self::from_xyz(x, self.y(), self.z())
        })
    }

    pub fn west(self) -> Option<Self> {
        let (x, carry) = self.x().overflowing_sub(1);
        (!carry).then_some(x).and_then(|x| {
            Self::from_xyz(x, self.y(), self.z())
        })
    }

    pub fn south(self) -> Option<Self> {
        let (z, carry) = self.z().overflowing_add(1);
        (!carry).then_some(z).and_then(|z| {
            Self::from_xyz(self.x(), self.y(), z)
        })
    }

    pub fn north(self) -> Option<Self> {
        let (z, carry) = self.z().overflowing_sub(1);
        (!carry).then_some(z).and_then(|z| {
            Self::from_xyz(self.x(), self.y(), z)
        })
    }
}

pub fn exposed(full_blocks: HashSet<BlockPos>) -> HashSet<BlockPos> {
    let pos_to_index = Vec::from_iter(full_blocks);
    let mut index_to_occurrence = vec![0u8; pos_to_index.len()];
    for pos in &pos_to_index {
        let (u, d, w, e, n, s) = (
            pos.up().unwrap(), pos.down().unwrap(), pos.west().unwrap(),
            pos.east().unwrap(), pos.north().unwrap(), pos.south().unwrap()
        );

        for p in [u, d, w, e, n, s] {
            if let Some((i, _)) = pos_to_index.iter().enumerate().find(|(_, x)| *x == &p) {
                index_to_occurrence[i] += 1;
            }
        }
    }

    index_to_occurrence.into_iter()
        .zip(pos_to_index)
        .filter_map(|(occurrence, pos)| {
            (occurrence != 6).then_some(pos)
        })
        .collect()
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
        let x = BlockPos::from_xyz(42, 35, 96).unwrap();

        let result = exposed(hashset![x]);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&x));
    }

    #[test]
    fn three_dimension_cross() {
        let x = BlockPos::from_xyz(21, 34, 59).unwrap();

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
        let x = BlockPos::from_xyz(39, 26, 16).unwrap();

        assert_eq!(x.up().unwrap().down().unwrap(), x);
    }

    #[test]
    fn south_and_north_are_dual() {
        // TODO: should be chosen randomly
        let x = BlockPos::from_xyz(99, 91, 0).unwrap();

        assert_eq!(x.south().unwrap().north().unwrap(), x);
    }

    #[test]
    fn west_and_east_are_dual() {
        // TODO: should be chosen randomly
        let x = BlockPos::from_xyz(128, 15, 19).unwrap();

        assert_eq!(x.west().unwrap().east().unwrap(), x);
    }
}
