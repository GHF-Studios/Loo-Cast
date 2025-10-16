pub mod grid;
pub mod subgrid;
pub mod unit;

#[macro_export]
macro_rules! grid_pos {
    ([$first:expr $(, $rest:expr)*]) => {
        {
            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            GridPos::try_from(stack).unwrap()
        }
    };
}

#[macro_export]
macro_rules! subgrid_pos {
    ([$first:expr $(, $rest:expr)*]: $sub:expr) => {
        {
            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            SubgridPos::try_from((stack, IVec2::from($sub))).unwrap()
        }
    };
}

#[macro_export]
macro_rules! unit_pos {
    ([$first:expr $(, $rest:expr)*]: $unit:expr) => {{
        {
            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            UnitPos::try_from((stack, Vec2::from($unit))).unwrap()
        }
    }};
}
