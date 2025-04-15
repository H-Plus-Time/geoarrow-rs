use wkt::wkt;

pub mod xy {
    use super::*;

    pub fn geoms() -> Vec<Option<wkt::types::MultiPolygon<i32>>> {
        vec![
            Some(wkt! { MULTIPOLYGON (((30 10, 40 40, 20 40, 10 20, 30 10))) }),
            Some(
                wkt! { MULTIPOLYGON (((30 20, 45 40, 10 40, 30 20)), ((15 5, 40 10, 10 20, 5 10, 15 5))) },
            ),
            Some(
                wkt! { MULTIPOLYGON (((40 40, 20 45, 45 30, 40 40)), ((20 35, 10 30, 10 10, 30 5, 45 20, 20 35), (30 20, 20 15, 20 25, 30 20))) },
            ),
            None,
            Some(wkt! { MULTIPOLYGON EMPTY }),
        ]
    }
}

pub mod xyz {
    use super::*;

    pub fn geoms() -> Vec<Option<wkt::types::MultiPolygon<i32>>> {
        vec![
            Some(wkt! { MULTIPOLYGON Z (((30 10 40, 40 40 80, 20 40 60, 10 20 30, 30 10 40))) }),
            Some(
                wkt! { MULTIPOLYGON Z (((30 20 50, 45 40 85, 10 40 50, 30 20 50)), ((15 5 20, 40 10 50, 10 20 30, 5 10 15, 15 5 20))) },
            ),
            Some(
                wkt! { MULTIPOLYGON Z (((40 40 80, 20 45 65, 45 30 75, 40 40 80)), ((20 35 55, 10 30 40, 10 10 20, 30 5 35, 45 20 65, 20 35 55), (30 20 50, 20 15 35, 20 25 45, 30 20 50))) },
            ),
            None,
            Some(wkt! { MULTIPOLYGON Z EMPTY }),
        ]
    }
}

pub mod xym {
    use super::*;

    pub fn geoms() -> Vec<Option<wkt::types::MultiPolygon<i32>>> {
        vec![
            Some(
                wkt! { MULTIPOLYGON M (((30 10 300, 40 40 1600, 20 40 800, 10 20 200, 30 10 300))) },
            ),
            Some(
                wkt! { MULTIPOLYGON M (((30 20 600, 45 40 1800, 10 40 400, 30 20 600)), ((15 5 75, 40 10 400, 10 20 200, 5 10 50, 15 5 75))) },
            ),
            Some(
                wkt! { MULTIPOLYGON M (((40 40 1600, 20 45 900, 45 30 1350, 40 40 1600)), ((20 35 700, 10 30 300, 10 10 100, 30 5 150, 45 20 900, 20 35 700), (30 20 600, 20 15 300, 20 25 500, 30 20 600))) },
            ),
            None,
            Some(wkt! { MULTIPOLYGON M EMPTY }),
        ]
    }
}

pub mod xyzm {
    use super::*;

    pub fn geoms() -> Vec<Option<wkt::types::MultiPolygon<i32>>> {
        vec![
            Some(
                wkt! { MULTIPOLYGON ZM (((30 10 40 300, 40 40 80 1600, 20 40 60 800, 10 20 30 200, 30 10 40 300))) },
            ),
            Some(
                wkt! { MULTIPOLYGON ZM (((30 20 50 600, 45 40 85 1800, 10 40 50 400, 30 20 50 600)), ((15 5 20 75, 40 10 50 400, 10 20 30 200, 5 10 15 50, 15 5 20 75))) },
            ),
            Some(
                wkt! { MULTIPOLYGON ZM (((40 40 80 1600, 20 45 65 900, 45 30 75 1350, 40 40 80 1600)), ((20 35 55 700, 10 30 40 300, 10 10 20 100, 30 5 35 150, 45 20 65 900, 20 35 55 700), (30 20 50 600, 20 15 35 300, 20 25 45 500, 30 20 50 600))) },
            ),
            None,
            Some(wkt! { MULTIPOLYGON ZM EMPTY }),
        ]
    }
}
