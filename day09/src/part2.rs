use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

struct File {
    id: usize,
    length: usize,
    position: usize,
}

struct Space {
    length: usize,
    position: usize,
}

pub fn solve(input: &str) -> Result<usize, Error> {
    let map = input.trim_end().as_bytes();
    let mut files = Vec::with_capacity(map.len() / 2 + 1);
    let mut spaces = Vec::with_capacity(map.len() / 2);
    let mut position = 0;

    for (i, byte) in map.iter().enumerate() {
        let length = (*byte - b'0') as usize;
        if i & 1 == 0 {
            files.push(File {
                id: i / 2,
                length,
                position,
            });
        } else {
            spaces.push(Space { length, position });
        }
        position += length;
    }

    for file in files.iter_mut().rev() {
        if let Some(space) = spaces
            .iter_mut()
            .take_while(|space| space.position < file.position)
            .find(|space| space.length >= file.length)
        {
            file.position = space.position;
            space.position += file.length;
            space.length -= file.length;
        }
    }

    Ok(files
        .iter()
        .flat_map(|file| {
            (file.position..)
                .take(file.length)
                .map(|position| file.id * position)
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE).unwrap();
        assert_eq!(result, 2858);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT).unwrap();
        assert_eq!(result, expected);
    }
}
