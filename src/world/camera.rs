use crate::world::map::{MAP_H, MAP_W};

/// Top-left world coordinate of the viewport. Follows the player, clamps to the
/// map, and when the terminal is *larger* than the map on an axis it centers the
/// map instead (the renderer fills the outside with border scenery, so no bars).
pub fn viewport_origin(player: (i32, i32), view_w: i32, view_h: i32) -> (i32, i32) {
    (
        axis_origin(player.0, view_w, MAP_W),
        axis_origin(player.1, view_h, MAP_H),
    )
}

fn axis_origin(center: i32, view: i32, map: i32) -> i32 {
    if view >= map {
        (map - view) / 2
    } else {
        (center - view / 2).clamp(0, map - view)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_to_map_edges() {
        assert_eq!(axis_origin(0, 80, MAP_W), 0);
        assert_eq!(axis_origin(MAP_W, 80, MAP_W), MAP_W - 80);
        assert_eq!(axis_origin(120, 80, MAP_W), 80);
    }

    #[test]
    fn centers_map_when_view_is_larger() {
        // 300-wide terminal on a 240-wide map: origin goes negative, map centered.
        assert_eq!(axis_origin(10, 300, MAP_W), -30);
    }
}
