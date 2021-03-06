use std::cmp::{min, max};
use std::vec;
use cgmath::aabb::{Aabb, Aabb2};
// cgmath Vector shadows std::vec::Vector and breaks as_slice...
use cgVector = cgmath::vector::Vector;
use cgmath::vector::{Vec2};
use cgmath::point::{Point, Point2};
use rectutil::RectUtil;

pub static TILE_ALPHA: u8 = 0x80;

#[deriving(Clone)]
pub struct Tile {
    bounds: Aabb2<int>,
    data: ~[u8],
}

impl Tile {
    // Only supporting alpha channel for now.
    pub fn new_alpha(bounds: Aabb2<int>, data: ~[u8]) -> Tile {
        let bpp = 1;
        assert!(data.len() / bpp == bounds.volume() as uint);

        let mut ret = Tile {
            bounds: bounds,
            data: data
        };
        ret.crop();
        ret
    }

    // Split a large image into small tiles.
    pub fn new_alpha_set(
        tile_dim: &Vec2<int>, sheet_dim: &Vec2<int>,
        data: ~[u8], offset: &Vec2<int>) -> ~[Tile] {
        let mut ret = ~[];
        for r in range(0, sheet_dim.y / tile_dim.y) {
            for c in range(0, sheet_dim.x / tile_dim.x) {
                let mut tile_data = vec::from_elem((tile_dim.x * tile_dim.y) as uint, 0u8);
                let p1 : Point2<int> = Point::from_vec(offset);
                let p2 : Point2<int> = Point::from_vec(&offset.add_v(tile_dim));
                let bounds = Aabb2::new(p1, p2);
                for p in bounds.points() {
                    let data_offset = c * tile_dim.x + p.x - offset.x + sheet_dim.x *
                        (r * tile_dim.y + p.y - offset.y);
                    tile_data[bounds.scan_pos(&p)] = data[data_offset];
                }
                ret.push(Tile::new_alpha(bounds, tile_data));
            }
        }
        ret
    }

    #[inline]
    pub fn contains(&self, pos: &Point2<int>) -> bool {
        self.bounds.contains(pos)
    }

    #[inline]
    pub fn at(&self, pos: &Point2<int>) -> u8 {
        if self.contains(pos) {
            self.data[self.bounds.scan_pos(pos)]
        } else {
            0u8
        }
    }

    pub fn crop(&mut self) {
        let (mut min_x, mut min_y) = (self.bounds.max().x, self.bounds.max().y);
        let (mut max_x, mut max_y) = (self.bounds.min().x - 1, self.bounds.min().y - 1);
        for p in self.bounds.points() {
            if self.at(&p) != TILE_ALPHA {
                min_x = min(min_x, p.x);
                min_y = min(min_y, p.y);
                max_x = max(max_x, p.x + 1);
                max_y = max(max_y, p.y + 1);
            }
        }
        if min_x >= max_x || min_y >= max_y {
            // Empty area.
            self.data = ~[];
            self.bounds = RectUtil::new(0, 0, 0, 0);
            return;
        }
        let new_bounds : Aabb2<int> = RectUtil::new(min_x, min_y, max_x, max_y);
        if new_bounds != self.bounds {
            assert!(
                new_bounds.min().x > self.bounds.min().x ||
                new_bounds.min().y > self.bounds.min().y ||
                new_bounds.max().x < self.bounds.max().x ||
                new_bounds.max().y < self.bounds.max().y);
            let mut new_data = vec::from_elem(new_bounds.volume() as uint, 0u8);
            for p in new_bounds.points() {
                new_data[new_bounds.scan_pos(&p)] =
                    self.data[self.bounds.scan_pos(&p)];
            }
            self.data = new_data;
            self.bounds = new_bounds;
        }
    }
}

