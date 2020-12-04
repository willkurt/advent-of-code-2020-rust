pub fn part_1() {
    let mt_map = load_map("./data/day_3.txt");
    println!("Trees = {}", mt_map.ct_trees_on_path(3, 1));
}

pub fn part_2() {
    let mt_map = load_map("./data/day_3.txt");
    let path_1 = mt_map.ct_trees_on_path(1, 1);
    let path_2 = mt_map.ct_trees_on_path(3, 1);
    let path_3 = mt_map.ct_trees_on_path(5, 1);
    let path_4 = mt_map.ct_trees_on_path(7, 1);
    let path_5 = mt_map.ct_trees_on_path(1, 2);
    println!("All paths: {}", path_1 * path_2 * path_3 * path_4 * path_5);
}

struct MtMap {
    map_data: Vec<Vec<char>>,
    depth: usize,
    width: usize,
}

impl MtMap {
    fn look_up(&self, x: usize, y: usize) -> Result<char, &'static str> {
        let x_wrapped = x % self.width;
        if y >= self.depth {
            Err("too deep!")
        } else {
            Ok(self.map_data[y][x_wrapped])
        }
    }

    fn ct_trees_on_path(&self, dx: usize, dy: usize) -> usize {
        let tree = '#';
        let mut x = 0;
        let mut y = 0;
        let mut tree_ct = 0;
        while let Ok(result_c) = self.look_up(x, y) {
            if result_c == tree {
                tree_ct += 1;
            }
            x += dx;
            y += dy;
        }
        tree_ct
    }
}

fn load_map(filename: &str) -> MtMap {
    let mut depth_c = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = crate::util::read_lines(filename) {
        for line in lines {
            if let Ok(row) = line {
                map.push(row.chars().collect());
                depth_c += 1;
            }
        }
    }
    let w = map[0].len();
    MtMap {
        map_data: map,
        depth: depth_c,
        width: w,
    }
}
