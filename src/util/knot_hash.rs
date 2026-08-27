pub fn knot_round(list: &mut [u16], lengths: &[usize], position: &mut usize, skip: &mut usize) {
    let len = list.len();

    for &length in lengths {
        for i in 0..length / 2 {
            list.swap((*position + i) % len, (*position + length - 1 - i) % len);
        }
        *position = (*position + length + *skip) % len;
        *skip += 1;
    }
}

pub fn dense_hash(sparse: &[u16]) -> Vec<u8> {
    sparse
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0u16, |acc, &x| acc ^ x) as u8)
        .collect()
}

/// The full 256-element, 64-round knot hash used by AoC 2017 day 10 part 2
/// and day 14, returned as a lowercase hex string.
pub fn hex_hash(input: &str) -> String {
    let mut lengths: Vec<usize> = input.trim().bytes().map(|b| b as usize).collect();
    lengths.extend([17, 31, 73, 47, 23]);

    let mut list: Vec<u16> = (0..256).collect();
    let mut position = 0;
    let mut skip = 0;

    for _ in 0..64 {
        knot_round(&mut list, &lengths, &mut position, &mut skip);
    }

    dense_hash(&list)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_hash() {
        assert_eq!(hex_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(hex_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(hex_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(hex_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
