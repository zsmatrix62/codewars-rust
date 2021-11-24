// https://www.codewars.com/kata/6135e4f40cffda0007ce356b/train/rust

fn bishop(start_pos: &str, end_pos: &str, num_moves: u8) -> bool {

    let (x0, y0) = get_coords(start_pos);
    let (x1, y1) = get_coords(end_pos);

    let is_same_position = x1 == x0 && y1 == y0;
    let is_same_color = (x0 + y0) % 2 == (x1 + y1) % 2;
    let is_same_line = (y0 - y1).abs() == (x0 - x1).abs();

    match num_moves {
        0 => is_same_position, // Same position
        1 => is_same_line, // Slope is a diagonal line
        _ => is_same_color, // Positions are the same color
    }

}

fn get_coords(pos: &str) -> (i32, i32) {
    let mut pos = pos.chars();
    let x = pos.next().unwrap() as i32 - 'a' as i32;
    let y = pos.next().unwrap() as i32 - '1' as i32;

    (x, y)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(bishop("a1", "b4", 2), true);
        assert_eq!(bishop("a1", "b5", 5), false);
        assert_eq!(bishop("f1", "f1", 0), true);
        assert_eq!(bishop("e6", "a1", 2), false);
    }
}

