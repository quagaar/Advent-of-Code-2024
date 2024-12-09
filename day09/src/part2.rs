struct File {
    id: usize,
    length: u8,
    position: usize,
}

struct Space {
    length: u8,
    position: usize,
}

pub fn solve(input: &str) -> usize {
    let map = input.trim_end().as_bytes();
    let mut files = Vec::with_capacity(map.len() / 2 + 1);
    let mut spaces = Vec::with_capacity(map.len() / 2);
    let mut position = 0;

    for (i, byte) in map.iter().enumerate() {
        let length = *byte - b'0';
        if i & 1 == 0 {
            files.push(File {
                id: i / 2,
                length,
                position,
            });
        } else {
            spaces.push(Space { length, position });
        }
        position += length as usize;
    }

    for file in files.iter_mut().rev() {
        if let Some(space) = spaces
            .iter_mut()
            .take_while(|space| space.position < file.position)
            .find(|space| space.length >= file.length)
        {
            file.position = space.position;
            space.position += file.length as usize;
            space.length -= file.length;
        }
    }

    files
        .iter()
        .flat_map(|file| {
            (file.position..)
                .take(file.length as usize)
                .map(|position| file.id * position)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 2858);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
