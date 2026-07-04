use crate::gfx::atlas::TILE;
use crate::world::map::{MAP_H, MAP_W};

/// Top-left world coordinate of the viewport. Follows the player, clamps to the
/// map, and when the view is *larger* than the map on an axis it centers the
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

/// The same camera in *pixels*: follows a pixel-precise player position (the
/// step-glide), so the world scrolls smoothly instead of snapping a tile at a
/// time. `center` is the point to keep mid-screen, `view` the viewport size
/// in pixels; the returned origin is the world pixel at the top-left corner.
pub fn viewport_origin_px(center: (i32, i32), view_w: i32, view_h: i32) -> (i32, i32) {
    (
        axis_origin(center.0, view_w, MAP_W * TILE as i32),
        axis_origin(center.1, view_h, MAP_H * TILE as i32),
    )
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
        // 300-wide view on a 240-wide map: origin goes negative, map centered.
        assert_eq!(axis_origin(10, 300, MAP_W), -30);
    }

    #[test]
    fn pixel_camera_clamps_like_the_tile_one() {
        let map_px = MAP_W * TILE as i32;
        assert_eq!(viewport_origin_px((0, 0), 240, 240).0, 0);
        assert_eq!(viewport_origin_px((map_px, 0), 240, 240).0, map_px - 240);
    }
}
