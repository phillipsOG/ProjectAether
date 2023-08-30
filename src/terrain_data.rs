use crate::space::Space;
type Map = Vec<Vec<Space>>;

pub struct TerrainData {
    pub(crate) map: Map,
    pub(crate) height_increase: usize,
    pub(crate) width_increase: usize,
}

impl TerrainData {
    pub(crate) fn new() -> Self {
        TerrainData {
            map: vec![vec![]],
            height_increase: 0,
            width_increase: 0,
        }
    }
}
