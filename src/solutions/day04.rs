const SOUTH_EAST: u8 = 1;
const SOUTH: u8 = 2;
const SOUTH_WEST: u8 = 4;
const EAST: u8 = 8;
const WEST: u8 = 16;
const NORTH_EAST: u8 = 32;
const NORTH: u8 = 64;
const NORTH_WEST: u8 = 128;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut haystack = Haystack::new(input.lines().map(|l| l.chars().collect()).collect());

    let mut xmas_count = 0;
    for y in 0..haystack.nrows as isize {
        for x in 0..haystack.ncols as isize {
            if haystack.haystack[y as usize][x as usize] == 'X' {
                if haystack.check(x - 1, y, 'M', WEST) {
                    xmas_count += 1;
                }
                if haystack.check(x - 1, y - 1, 'M', NORTH_WEST) {
                    xmas_count += 1;
                }
                if haystack.check(x, y - 1, 'M', NORTH) {
                    xmas_count += 1;
                }
                if haystack.check(x + 1, y - 1, 'M', NORTH_EAST) {
                    xmas_count += 1;
                }
                haystack.directions[y as usize][x as usize] =
                    SOUTH_WEST | SOUTH | SOUTH_EAST | EAST;
            } else if haystack.haystack[y as usize][x as usize] == 'S' {
                if haystack.check(x - 1, y, 'A', EAST) {
                    xmas_count += 1;
                }
                if haystack.check(x - 1, y - 1, 'A', SOUTH_EAST) {
                    xmas_count += 1;
                }
                if haystack.check(x, y - 1, 'A', SOUTH) {
                    xmas_count += 1;
                }
                if haystack.check(x + 1, y - 1, 'A', SOUTH_WEST) {
                    xmas_count += 1;
                }
                haystack.directions[y as usize][x as usize] =
                    NORTH_EAST | NORTH | NORTH_WEST | WEST;
            } else if haystack.haystack[y as usize][x as usize] == 'M' {
                if haystack.check(x - 1, y, 'A', WEST) {
                    haystack.directions[y as usize][x as usize] |= WEST;
                }
                if haystack.check(x - 1, y - 1, 'A', NORTH_WEST) {
                    haystack.directions[y as usize][x as usize] |= NORTH_WEST;
                }
                if haystack.check(x, y - 1, 'A', NORTH) {
                    haystack.directions[y as usize][x as usize] |= NORTH;
                }
                if haystack.check(x + 1, y - 1, 'A', NORTH_EAST) {
                    haystack.directions[y as usize][x as usize] |= NORTH_EAST;
                }
                if haystack.check(x - 1, y, 'X', EAST) {
                    haystack.directions[y as usize][x as usize] |= EAST;
                }
                if haystack.check(x - 1, y - 1, 'X', SOUTH_EAST) {
                    haystack.directions[y as usize][x as usize] |= SOUTH_EAST;
                }
                if haystack.check(x, y - 1, 'X', SOUTH) {
                    haystack.directions[y as usize][x as usize] |= SOUTH;
                }
                if haystack.check(x + 1, y - 1, 'X', SOUTH_WEST) {
                    haystack.directions[y as usize][x as usize] |= SOUTH_WEST;
                }
            } else if haystack.haystack[y as usize][x as usize] == 'A' {
                if haystack.check(x - 1, y, 'S', WEST) {
                    haystack.directions[y as usize][x as usize] |= WEST;
                }
                if haystack.check(x - 1, y - 1, 'S', NORTH_WEST) {
                    haystack.directions[y as usize][x as usize] |= NORTH_WEST;
                }
                if haystack.check(x, y - 1, 'S', NORTH) {
                    haystack.directions[y as usize][x as usize] |= NORTH;
                }
                if haystack.check(x + 1, y - 1, 'S', NORTH_EAST) {
                    haystack.directions[y as usize][x as usize] |= NORTH_EAST;
                }
                if haystack.check(x - 1, y, 'M', EAST) {
                    haystack.directions[y as usize][x as usize] |= EAST;
                }
                if haystack.check(x - 1, y - 1, 'M', SOUTH_EAST) {
                    haystack.directions[y as usize][x as usize] |= SOUTH_EAST;
                }
                if haystack.check(x, y - 1, 'M', SOUTH) {
                    haystack.directions[y as usize][x as usize] |= SOUTH;
                }
                if haystack.check(x + 1, y - 1, 'M', SOUTH_WEST) {
                    haystack.directions[y as usize][x as usize] |= SOUTH_WEST;
                }
            }
        }
    }

    // Part 2
    let mut part2 = 0;
    for y in 0..haystack.nrows {
        for x in 0..haystack.ncols {
            if haystack.is_xmas(x, y) {
                part2 += 1;
            }
        }
    }

    (xmas_count.to_string(), part2.to_string())
}

struct Haystack {
    haystack: Vec<Vec<char>>,
    directions: Vec<Vec<u8>>,
    nrows: usize,
    ncols: usize,
}

impl Haystack {
    fn new(data: Vec<Vec<char>>) -> Self {
        Self {
            nrows: data.len(),
            ncols: data[0].len(),
            directions: (0..data.len())
                .map(|_| (0..data[0].len()).map(|_| 0).collect())
                .collect(),
            haystack: data,
        }
    }
    fn check(&self, x: isize, y: isize, val: char, dir: u8) -> bool {
        if 0 <= y && y < self.nrows as isize && 0 <= x && x < self.ncols as isize {
            return self.haystack[y as usize][x as usize] == val
                && self.directions[y as usize][x as usize] & dir == dir;
        } else {
            return false;
        }
    }
    fn is_xmas(&self, x: usize, y: usize) -> bool {
        if self.haystack[y][x] == 'A' && 0 < y && y + 1 < self.nrows && 0 < x && x + 1 < self.ncols
        {
            return ((self.haystack[y - 1][x - 1] == 'M' && self.haystack[y + 1][x + 1] == 'S')
                || (self.haystack[y - 1][x - 1] == 'S' && self.haystack[y + 1][x + 1] == 'M'))
                && ((self.haystack[y - 1][x + 1] == 'M' && self.haystack[y + 1][x - 1] == 'S')
                    || (self.haystack[y - 1][x + 1] == 'S' && self.haystack[y + 1][x - 1] == 'M'));
        } else {
            return false;
        }
    }
}
