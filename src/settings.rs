pub const INDICES: &[u32] = &[0, 1, 2, 0, 2, 3];
pub const GRID: (u32, u32) = (600, 600);
// ([GRID.0, GRID.1][(GRID.0 < GRID.1) as usize]) calculates the min of GRID
// this method is used, since the function min is not permitted during compile time
pub const SIZE: f32 = 2000.0/([GRID.0, GRID.1][(GRID.0 < GRID.1) as usize]) as f32;
pub const INSTCOUNT: usize = (GRID.0 * GRID.1) as usize;