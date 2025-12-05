use bevy::prelude::{IVec2, Reflect, Vec3};

use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::{DynScale, Scale};

#[derive(Default)]
pub struct SubgridVecBuilder {
    chain: Vec<IVec2>,
}

impl SubgridVecBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, next: (i32, i32)) -> Self {
        let next = IVec2::new(next.0, next.1);
        self.chain.push(next);
        self
    }

    pub fn push_many<I: IntoIterator<Item = (i32, i32)>>(mut self, items: I) -> Self {
        self.chain.extend(items.into_iter().map(|xy| IVec2::new(xy.0, xy.1)));
        self
    }

    pub fn repeat(mut self, xy: (i32, i32), count: usize) -> Self {
        self.chain.extend(std::iter::repeat_n(IVec2::new(xy.0, xy.1), count));
        self
    }

    pub fn reverse(mut self) -> Self {
        self.chain.reverse();
        self
    }

    pub fn finish(self, subgrid_xy: (i32, i32)) -> SubgridVec {
        SubgridVec::try_from((self.chain, IVec2::new(subgrid_xy.0, subgrid_xy.1))).unwrap()
    }
}

#[derive(Default, Clone, PartialEq, Reflect)]
pub struct SubgridVec {
    pub(in super::super) grid_offset: GridVec,
    pub(in super::super) subgrid_offset: IVec2,
}
impl SubgridVec {
    pub fn build() -> SubgridVecBuilder {
        SubgridVecBuilder::new()
    }

    fn validate_grid_offset(grid_offset: &GridVec) {
        if grid_offset.scale == Scale::MIN {
            panic!("SubgridVec must be based on a scale no smaller than MIN+1, so there is room to represent the subgrid level as a virtual GridVec leaf");
        }
    }

    fn validate_subgrid_offset(subgrid_offset: &IVec2) {
        if subgrid_offset.x < -5 {
            panic!("X coordinate {} is too small. Range is (-5..5)", subgrid_offset.x);
        }
        if subgrid_offset.x >= 5 {
            panic!("X coordinate {} is too large. Range is (-5..5)", subgrid_offset.x);
        }
        if subgrid_offset.y < -5 {
            panic!("Y coordinate {} is too small. Range is (-5..5)", subgrid_offset.y);
        }
        if subgrid_offset.y >= 5 {
            panic!("Y coordinate {} is too large. Range is (-5..5)", subgrid_offset.y);
        }
    }

    pub fn new(grid_offset: GridVec, subgrid_offset: IVec2) -> Self {
        Self::validate_grid_offset(&grid_offset);
        Self::validate_subgrid_offset(&subgrid_offset);
        Self { grid_offset, subgrid_offset }
    }

    pub fn zoom_out(&mut self) {
        if self.grid_offset.parent.as_ref().unwrap().parent.is_none() {
            panic!("Cannot zoom out SubgridVec beyond the root GridVec");
        }

        let grid_extent = GridVec::new(self.grid_offset.clone(), self.subgrid_offset);

        let mut unit_extent = UnitVec {
            grid_offset: grid_extent,
            unit_offset: Vec3::ZERO,
        };

        unit_extent.zoom_out();

        self.grid_offset = (*unit_extent.grid_offset.parent.unwrap()).clone();
        self.subgrid_offset = unit_extent.grid_offset.xy;
    }
}
impl std::fmt::Debug for SubgridVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}: {})", self.grid_offset, self.subgrid_offset)
    }
}
impl std::ops::Add<IVec2> for SubgridVec {
    type Output = Self;

    fn add(mut self, rhs: IVec2) -> Self::Output {
        self.subgrid_offset += rhs;
        Self::validate_subgrid_offset(&self.subgrid_offset);
        self
    }
}
impl std::ops::AddAssign<IVec2> for SubgridVec {
    fn add_assign(&mut self, rhs: IVec2) {
        self.subgrid_offset += rhs;
        Self::validate_subgrid_offset(&self.subgrid_offset);
    }
}
impl std::ops::Sub<IVec2> for SubgridVec {
    type Output = Self;

    fn sub(mut self, rhs: IVec2) -> Self::Output {
        self.subgrid_offset -= rhs;
        Self::validate_subgrid_offset(&self.subgrid_offset);
        self
    }
}
impl std::ops::SubAssign<IVec2> for SubgridVec {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.subgrid_offset -= rhs;
        Self::validate_subgrid_offset(&self.subgrid_offset);
    }
}
impl std::ops::Add<SubgridVec> for SubgridVec {
    type Output = Self;

    fn add(self, rhs: SubgridVec) -> Self::Output {
        // === Phase 1: Build extended GridVec stacks from root to leaf ===
        fn build_stack(subgrid: &SubgridVec) -> Vec<(Scale, IVec2)> {
            let mut stack = Vec::new();
            let mut cursor = &subgrid.grid_offset;
            loop {
                stack.push((cursor.scale, cursor.xy));
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();

            // Append the phantom subgrid level (one scale down)
            let subgrid_scale = subgrid.grid_offset.scale.down().expect("No lower scale for subgrid");
            stack.push((subgrid_scale, subgrid.subgrid_offset));

            stack
        }

        let mut a_stack = build_stack(&self);
        let mut b_stack = build_stack(&rhs);
        let max_depth = a_stack.len().max(b_stack.len());

        // Pad shorter stack with (scale, ZERO)
        while a_stack.len() < max_depth {
            let (s, _) = b_stack[a_stack.len()];
            a_stack.push((s, IVec2::ZERO));
        }
        while b_stack.len() < max_depth {
            let (s, _) = a_stack[b_stack.len()];
            b_stack.push((s, IVec2::ZERO));
        }

        // === Phase 2: Raw sum top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0;
            let sum = a_stack[i].1 + b_stack[i].1;
            raw_stack.push((scale, sum));
        }

        // === Phase 3: Normalize with wrapping + carry ===
        let mut carry = IVec2::ZERO;
        for i in (0..raw_stack.len()).rev() {
            let (_scale, sum) = raw_stack[i];
            let wrapped_x = ((sum.x + carry.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((sum.y + carry.y + 5).rem_euclid(10)) - 5;
            let carry_x = (sum.x + carry.x - wrapped_x).div_euclid(10);
            let carry_y = (sum.y + carry.y - wrapped_y).div_euclid(10);

            raw_stack[i].1 = IVec2::new(wrapped_x, wrapped_y);
            carry = IVec2::new(carry_x, carry_y);
        }

        // === Phase 4: Build GridVec tree and extract SubgridVec ===
        let mut result: Option<GridVec> = None;
        for (_scale, xy) in raw_stack {
            result = Some(match result {
                Some(parent) => GridVec::new(parent, xy),
                None => GridVec::new_root(xy),
            });
        }

        let final_leaf = result.unwrap();
        let subgrid_offset = final_leaf.xy;
        let grid_offset = (*final_leaf.parent.unwrap()).clone();

        SubgridVec { grid_offset, subgrid_offset }
    }
}
impl std::ops::AddAssign<SubgridVec> for SubgridVec {
    fn add_assign(&mut self, rhs: SubgridVec) {
        *self = self.clone() + rhs;
    }
}
impl std::ops::Sub<SubgridVec> for SubgridVec {
    type Output = Self;

    fn sub(self, rhs: SubgridVec) -> Self::Output {
        // === Phase 1: Build extended GridVec stacks from root to leaf ===
        fn build_stack(subgrid: &SubgridVec) -> Vec<(Scale, IVec2)> {
            let mut stack = Vec::new();
            let mut cursor = &subgrid.grid_offset;
            loop {
                stack.push((cursor.scale, cursor.xy));
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();

            // Append the phantom subgrid level (one scale down)
            let subgrid_scale = subgrid.grid_offset.scale.down().expect("No lower scale for subgrid");
            stack.push((subgrid_scale, subgrid.subgrid_offset));

            stack
        }

        let mut a_stack = build_stack(&self);
        let mut b_stack = build_stack(&rhs);
        let max_depth = a_stack.len().max(b_stack.len());

        // Pad shorter stack with (scale, ZERO)
        while a_stack.len() < max_depth {
            let (s, _) = b_stack[a_stack.len()];
            a_stack.push((s, IVec2::ZERO));
        }
        while b_stack.len() < max_depth {
            let (s, _) = a_stack[b_stack.len()];
            b_stack.push((s, IVec2::ZERO));
        }

        // === Phase 2: Raw diff top-down ===
        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0;
            let diff = a_stack[i].1 - b_stack[i].1;
            raw_stack.push((scale, diff));
        }

        // === Phase 3: Normalize with wrapping + carry ===
        let mut carry = IVec2::ZERO;
        for i in (0..raw_stack.len()).rev() {
            let (_scale, diff) = raw_stack[i];
            let wrapped_x = ((diff.x + carry.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((diff.y + carry.y + 5).rem_euclid(10)) - 5;
            let carry_x = (diff.x + carry.x - wrapped_x).div_euclid(10);
            let carry_y = (diff.y + carry.y - wrapped_y).div_euclid(10);

            raw_stack[i].1 = IVec2::new(wrapped_x, wrapped_y);
            carry = IVec2::new(carry_x, carry_y);
        }

        // === Phase 4: Build GridVec tree and extract SubgridVec ===
        let mut result: Option<GridVec> = None;
        for (_scale, xy) in raw_stack {
            result = Some(match result {
                Some(parent) => GridVec::new(parent, xy),
                None => GridVec::new_root(xy),
            });
        }

        let final_leaf = result.unwrap();
        let subgrid_offset = final_leaf.xy;
        let grid_offset = (*final_leaf.parent.unwrap()).clone();

        SubgridVec { grid_offset, subgrid_offset }
    }
}
impl std::ops::SubAssign<SubgridVec> for SubgridVec {
    fn sub_assign(&mut self, rhs: SubgridVec) {
        *self = self.clone() - rhs;
    }
}
impl std::convert::TryFrom<(Vec<IVec2>, IVec2)> for SubgridVec {
    type Error = &'static str;

    fn try_from((stack, subgrid_offset): (Vec<IVec2>, IVec2)) -> Result<Self, Self::Error> {
        let grid_offset = GridVec::try_from(stack)?;
        Ok(SubgridVec::new(grid_offset, subgrid_offset))
    }
}
