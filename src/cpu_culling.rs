//! CPU-culling, using Union-Find

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct BlockPos {
    ///
    repr: u64,
}

// NOTE for IntelliJ user: "method is private" on `overflowing_add` method is false-positive
impl BlockPos {
    const X_ABS_MAX: u32 = 0x3FF_FFFF;
    const Y_ABS_MAX: u16 = 0x000_0FFF;
    const Z_ABS_MAX: u32 = 0x3FF_FFFF;

    /// Create position. Returns [Err] if value is out of range.
    ///
    /// # Panics
    /// Never.
    pub fn from_xyz(x: i32, y: i16, z: i32) -> Result<Self, ()> {
        let is_in_domain = x.abs() as u64 <= Self::X_ABS_MAX as u64 && y.abs() as u16 <= Self::Y_ABS_MAX && z.abs() as u64 <= Self::Z_ABS_MAX as u64;

        is_in_domain.then_some(Self::compute_repr(x, y, z)).map(|repr| Self { repr }).ok_or(())
    }

    #[inline]
    fn compute_repr(x: i32, y: i16, z: i32) -> u64 {
        (((x as i64 & Self::X_ABS_MAX as i64) << 38) | ((z as i64 & Self::Z_ABS_MAX as i64) << 12) | (y & Self::Y_ABS_MAX as i16) as i64) as u64
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

    pub fn up(self) -> Self {
        Self::from_xyz(self.x(), self.y() + 1, self.z()).unwrap()
    }

    pub fn down(self) -> Self {
        Self::from_xyz(self.x(), self.y() - 1, self.z()).unwrap()
    }

    pub fn east(self) -> Self {
        Self::from_xyz(self.x() + 1, self.y(), self.z()).unwrap()
    }

    pub fn west(self) -> Self {
        Self::from_xyz(self.x() - 1, self.y(), self.z()).unwrap()
    }

    pub fn south(self) -> Self {
        Self::from_xyz(self.x(), self.y(), self.z() + 1).unwrap()
    }

    pub fn north(self) -> Self {
        Self::from_xyz(self.x(), self.y(), self.z() - 1).unwrap()
    }
}

pub fn exposed(full_blocks: &[BlockPos]) -> Vec<&BlockPos> {
    let pos_to_index = full_blocks;
    let mut index_to_occurrence = vec![0u8; pos_to_index.len()];
    for pos in pos_to_index {
        let (u, d, w, e, n, s) = (
            pos.up(), pos.down(), pos.west(),
            pos.east(), pos.north(), pos.south()
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
    use crate::cpu_culling::{BlockPos, exposed};

    #[test]
    fn empty_returns_empty() {
        assert!(exposed(&[]).is_empty());
    }

    #[test]
    fn single_returns_single() {
        let x = BlockPos::from_xyz(42, 35, 96).unwrap();

        let y = [x];
        let result = exposed(&y);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&&x));
    }

    #[test]
    fn three_dimension_cross() {
        let x = BlockPos::from_xyz(21, 34, 59).unwrap();

        let up = x.up();
        let down = x.down();
        let north = x.north();
        let south = x.south();
        let east = x.east();
        let west = x.west();

        let y = [x, up, down, north, south, east, west];
        let result = exposed(&y);
        assert_eq!(result.len(), 6);
        for x in &[up, down, north, south, east, west] {
            assert!(result.contains(&&x));
        }
    }

    #[test]
    fn up_and_down_are_dual() {
        // TODO: should be chosen randomly
        let x = BlockPos::from_xyz(39, 26, 16).unwrap();

        assert_eq!(x.up().down(), x);
    }

    #[test]
    fn south_and_north_are_dual() {
        // TODO: should be chosen randomly
        let x = BlockPos::from_xyz(99, 91, 0).unwrap();

        assert_eq!(x.south().north(), x);
    }

    #[test]
    fn west_and_east_are_dual() {
        // TODO: should be chosen randomly
        let x = BlockPos::from_xyz(128, 15, 19).unwrap();

        assert_eq!(x.west().east(), x);
    }
}
