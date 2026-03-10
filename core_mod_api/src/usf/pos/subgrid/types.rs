use crate::bevy::prelude::{IVec3, Reflect, Vec3};

use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::types::{GridXyz, LocalCell3, SubgridXyz};
use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::{DynScale, Scale};

#[derive(Default)]
pub struct SubgridVecBuilder {
    chain: Vec<GridXyz>,
}

impl SubgridVecBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, next: impl Into<LocalCell3>) -> Self {
        self.chain.push(GridXyz::from_local_cell3(next.into()));
        self
    }

    pub fn push_many<I, C>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<LocalCell3>,
    {
        self.chain
            .extend(items.into_iter().map(|xyz| GridXyz::from_local_cell3(xyz.into())));
        self
    }

    pub fn repeat(mut self, xyz: impl Into<LocalCell3>, count: usize) -> Self {
        let xyz = GridXyz::from_local_cell3(xyz.into());
        self.chain.extend(std::iter::repeat_n(xyz, count));
        self
    }

    pub fn reverse(mut self) -> Self {
        self.chain.reverse();
        self
    }

    pub fn finish(self, subgrid_xyz: impl Into<LocalCell3>) -> SubgridVec {
        SubgridVec::try_from((self.chain, SubgridXyz::from_local_cell3(subgrid_xyz.into()))).unwrap()
    }
}

#[derive(Default, Clone, PartialEq, Reflect)]
pub struct SubgridVec {
    pub(in super::super) grid_offset: GridVec,
    pub(in super::super) subgrid_offset: SubgridXyz,
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

    pub fn new(grid_offset: GridVec, subgrid_offset: SubgridXyz) -> Self {
        Self::validate_grid_offset(&grid_offset);
        subgrid_offset.assert_local();
        Self { grid_offset, subgrid_offset }
    }

    pub fn zoom_out(&mut self) {
        if self.grid_offset.parent.as_ref().unwrap().parent.is_none() {
            panic!("Cannot zoom out SubgridVec beyond the root GridVec");
        }

        let grid_extent = GridVec::new(
            self.grid_offset.clone(),
            GridXyz::new_local(self.subgrid_offset.x, self.subgrid_offset.y, self.subgrid_offset.z),
        );

        let mut unit_extent = UnitVec {
            grid_offset: grid_extent,
            unit_offset: Vec3::ZERO,
        };

        unit_extent.zoom_out();

        self.grid_offset = (*unit_extent.grid_offset.parent.unwrap()).clone();
        self.subgrid_offset = SubgridXyz::new_local(
            unit_extent.grid_offset.xyz.x,
            unit_extent.grid_offset.xyz.y,
            unit_extent.grid_offset.xyz.z,
        );
    }
}
impl std::fmt::Debug for SubgridVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}: {})", self.grid_offset, self.subgrid_offset)
    }
}
impl std::ops::Add<IVec3> for SubgridVec {
    type Output = Self;

    fn add(mut self, rhs: IVec3) -> Self::Output {
        self.subgrid_offset += rhs;
        self.subgrid_offset.assert_local();
        self
    }
}
impl std::ops::AddAssign<IVec3> for SubgridVec {
    fn add_assign(&mut self, rhs: IVec3) {
        self.subgrid_offset += rhs;
        self.subgrid_offset.assert_local();
    }
}
impl std::ops::Sub<IVec3> for SubgridVec {
    type Output = Self;

    fn sub(mut self, rhs: IVec3) -> Self::Output {
        self.subgrid_offset -= rhs;
        self.subgrid_offset.assert_local();
        self
    }
}
impl std::ops::SubAssign<IVec3> for SubgridVec {
    fn sub_assign(&mut self, rhs: IVec3) {
        self.subgrid_offset -= rhs;
        self.subgrid_offset.assert_local();
    }
}
impl std::ops::Add<SubgridVec> for SubgridVec {
    type Output = Self;

    fn add(self, rhs: SubgridVec) -> Self::Output {
        fn build_stack(subgrid: &SubgridVec) -> Vec<(Scale, IVec3)> {
            let mut stack = Vec::new();
            let mut cursor = &subgrid.grid_offset;
            loop {
                stack.push((cursor.scale, cursor.xyz.as_ivec3()));
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();

            let subgrid_scale = subgrid.grid_offset.scale.down().expect("No lower scale for subgrid");
            stack.push((subgrid_scale, subgrid.subgrid_offset.as_ivec3()));

            stack
        }

        let mut a_stack = build_stack(&self);
        let mut b_stack = build_stack(&rhs);
        let max_depth = a_stack.len().max(b_stack.len());

        while a_stack.len() < max_depth {
            let (s, _) = b_stack[a_stack.len()];
            a_stack.push((s, IVec3::ZERO));
        }
        while b_stack.len() < max_depth {
            let (s, _) = a_stack[b_stack.len()];
            b_stack.push((s, IVec3::ZERO));
        }

        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0;
            let sum = a_stack[i].1 + b_stack[i].1;
            raw_stack.push((scale, sum));
        }

        let mut carry = IVec3::ZERO;
        for i in (0..raw_stack.len()).rev() {
            let (_scale, sum) = raw_stack[i];
            let wrapped_x = ((sum.x + carry.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((sum.y + carry.y + 5).rem_euclid(10)) - 5;
            let wrapped_z = ((sum.z + carry.z + 5).rem_euclid(10)) - 5;
            let carry_x = (sum.x + carry.x - wrapped_x).div_euclid(10);
            let carry_y = (sum.y + carry.y - wrapped_y).div_euclid(10);
            let carry_z = (sum.z + carry.z - wrapped_z).div_euclid(10);

            raw_stack[i].1 = IVec3::new(wrapped_x, wrapped_y, wrapped_z);
            carry = IVec3::new(carry_x, carry_y, carry_z);
        }

        let mut result: Option<GridVec> = None;
        for (_scale, xyz) in raw_stack {
            result = Some(match result {
                Some(parent) => GridVec::new(parent, GridXyz::new_local(xyz.x, xyz.y, xyz.z)),
                None => GridVec::new_root(GridXyz::new_local(xyz.x, xyz.y, xyz.z)),
            });
        }

        let final_leaf = result.unwrap();
        let subgrid_offset = SubgridXyz::new_local(final_leaf.xyz.x, final_leaf.xyz.y, final_leaf.xyz.z);
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
        fn build_stack(subgrid: &SubgridVec) -> Vec<(Scale, IVec3)> {
            let mut stack = Vec::new();
            let mut cursor = &subgrid.grid_offset;
            loop {
                stack.push((cursor.scale, cursor.xyz.as_ivec3()));
                if let Some(p) = &cursor.parent {
                    cursor = p;
                } else {
                    break;
                }
            }
            stack.reverse();

            let subgrid_scale = subgrid.grid_offset.scale.down().expect("No lower scale for subgrid");
            stack.push((subgrid_scale, subgrid.subgrid_offset.as_ivec3()));

            stack
        }

        let mut a_stack = build_stack(&self);
        let mut b_stack = build_stack(&rhs);
        let max_depth = a_stack.len().max(b_stack.len());

        while a_stack.len() < max_depth {
            let (s, _) = b_stack[a_stack.len()];
            a_stack.push((s, IVec3::ZERO));
        }
        while b_stack.len() < max_depth {
            let (s, _) = a_stack[b_stack.len()];
            b_stack.push((s, IVec3::ZERO));
        }

        let mut raw_stack = Vec::with_capacity(max_depth);
        for i in 0..max_depth {
            let scale = a_stack[i].0;
            let diff = a_stack[i].1 - b_stack[i].1;
            raw_stack.push((scale, diff));
        }

        let mut carry = IVec3::ZERO;
        for i in (0..raw_stack.len()).rev() {
            let (_scale, diff) = raw_stack[i];
            let wrapped_x = ((diff.x + carry.x + 5).rem_euclid(10)) - 5;
            let wrapped_y = ((diff.y + carry.y + 5).rem_euclid(10)) - 5;
            let wrapped_z = ((diff.z + carry.z + 5).rem_euclid(10)) - 5;
            let carry_x = (diff.x + carry.x - wrapped_x).div_euclid(10);
            let carry_y = (diff.y + carry.y - wrapped_y).div_euclid(10);
            let carry_z = (diff.z + carry.z - wrapped_z).div_euclid(10);

            raw_stack[i].1 = IVec3::new(wrapped_x, wrapped_y, wrapped_z);
            carry = IVec3::new(carry_x, carry_y, carry_z);
        }

        let mut result: Option<GridVec> = None;
        for (_scale, xyz) in raw_stack {
            result = Some(match result {
                Some(parent) => GridVec::new(parent, GridXyz::new_local(xyz.x, xyz.y, xyz.z)),
                None => GridVec::new_root(GridXyz::new_local(xyz.x, xyz.y, xyz.z)),
            });
        }

        let final_leaf = result.unwrap();
        let subgrid_offset = SubgridXyz::new_local(final_leaf.xyz.x, final_leaf.xyz.y, final_leaf.xyz.z);
        let grid_offset = (*final_leaf.parent.unwrap()).clone();

        SubgridVec { grid_offset, subgrid_offset }
    }
}
impl std::ops::SubAssign<SubgridVec> for SubgridVec {
    fn sub_assign(&mut self, rhs: SubgridVec) {
        *self = self.clone() - rhs;
    }
}
impl std::convert::TryFrom<(Vec<GridXyz>, SubgridXyz)> for SubgridVec {
    type Error = &'static str;

    fn try_from((stack, subgrid_offset): (Vec<GridXyz>, SubgridXyz)) -> Result<Self, Self::Error> {
        let grid_offset = GridVec::try_from(stack)?;
        Ok(SubgridVec::new(grid_offset, subgrid_offset))
    }
}
impl std::convert::TryFrom<(Vec<IVec3>, IVec3)> for SubgridVec {
    type Error = &'static str;

    fn try_from((stack, subgrid_offset): (Vec<IVec3>, IVec3)) -> Result<Self, Self::Error> {
        let grid_offset = GridVec::try_from(stack)?;
        let Ok(local) = SubgridXyz::try_from(subgrid_offset) else {
            return Err("SubgridVec requires local subgrid coordinates in [-5..5)");
        };
        Ok(SubgridVec::new(grid_offset, local))
    }
}
