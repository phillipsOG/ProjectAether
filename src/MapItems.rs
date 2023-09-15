#[derive(Clone)]
pub struct MapItem {
    pub map_width: usize,
    pub map_height: usize,
}

impl MapItem {
    pub(crate) fn new() -> Self {
        MapItem {

            map_width: 0,
            map_height: 0,
        }
    }
}