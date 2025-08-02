use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref DIST_MATRIX: HashMap<char, [u8; 2]> = {
        let mut m = HashMap::new();
        // Row 0 - QWERTY row
        m.insert('q', [0,0]);
        m.insert('w', [0,1]);
        m.insert('e', [0,2]);
        m.insert('r', [0,3]);
        m.insert('t', [0,4]);
        m.insert('y', [0,5]);
        m.insert('u', [0,6]);
        m.insert('i', [0,7]);
        m.insert('o', [0,8]);
        m.insert('p', [0,9]);

        // Row 1 - ASDF row
        m.insert('a', [1,0]);
        m.insert('s', [1,1]);
        m.insert('d', [1,2]);
        m.insert('f', [1,3]);
        m.insert('g', [1,4]);
        m.insert('h', [1,5]);
        m.insert('j', [1,6]);
        m.insert('k', [1,7]);
        m.insert('l', [1,8]);

        // Row 2 - ZXCV row
        m.insert('z', [2,0]);
        m.insert('x', [2,1]);
        m.insert('c', [2,2]);
        m.insert('v', [2,3]);
        m.insert('b', [2,4]);
        m.insert('n', [2,5]);
        m.insert('m', [2,6]);
        m
    };
}

pub fn get_keywords_at_dist(letter: &char, dist: &u8) -> Vec<char> {
    let mut result = Vec::new();

    let cur_letter_coord = match DIST_MATRIX.get(letter) {
        Some(coord) => coord,
        None => return result,
    };

    for (key, other_key_coord) in DIST_MATRIX.iter() {
        let row_diff = (cur_letter_coord[0] as i32 - other_key_coord[0] as i32).abs() as u8;
        let col_diff = (cur_letter_coord[1] as i32 - other_key_coord[1] as i32).abs() as u8;
        if row_diff + col_diff <= *dist {
            result.push(*key);
        }
    }
    result
}
