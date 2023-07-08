use bottomless_pit::vectors::Vec2;

pub fn point_in_rect(rect_size: Vec2<f32>, pos: Vec2<f32>, point: Vec2<f32>) -> bool {
    if point.x < pos.x {
        return false
    }
    if point.y < pos.y {
        return false
    }
    if point.y > (pos.y + rect_size.y) {
        return false
    }
    if point.x > (pos.x + rect_size.x) {
        return false
    }

    true
}

pub fn rect_rect(r1_size: Vec2<f32>, r1_pos: Vec2<f32>, r2_size: Vec2<f32>, r2_pos: Vec2<f32>) -> bool {
    
    if r1_pos.x + r1_size.x >= r2_pos.x && r1_pos.x <= r2_pos.x + r2_size.x &&
       r1_pos.y + r1_size.y >= r2_pos.y && r1_pos.y <= r2_pos.y + r2_size.y {
        true
    } else {
        false
    }
}