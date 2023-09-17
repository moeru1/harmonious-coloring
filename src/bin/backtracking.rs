const MAX_X: usize = 4;
const MAX_Y: usize = 9;

const END_X: usize = 3;
const END_Y: usize = 8;

#[derive(Default)]
struct P {
    x: usize,
    y: usize,
}

fn main() {
    let s_x = 0;
    let s_y = 0;

    let m = [
        [0, 0, 0, 1, 1, 1, 0, 0, 0],
        [1, 1, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 1, 0, 0, 1, 0, 1, 0],
        [0, 0, 1, 1, 0, 1, 1, 1, 0],
    ];

    let mut path: [P; MAX_X + MAX_Y + 1] = Default::default();

    //    for x in 0..MAX_X {
    //        for y in 0..MAX_Y {
    //            print!("{} ",m[x][y]);
    //        }
    //        println!("");
    //    };
    //
    check_paths(&m, s_x, s_y, &mut path, 0);
    //present_path(&path);
}

fn check_paths(
    m: &[[i32; MAX_Y]; MAX_X],
    c_x: usize,
    c_y: usize,
    path: &mut [P; MAX_X + MAX_Y + 1],
    l: usize,
) {
    if !try_candidate(m, c_x, c_y) {
        return;
    }

    path[l] = P { x: c_x, y: c_y };

    if c_x == END_X && c_y == END_Y {
        present_path(&path[..=l]);
    } else {
        check_paths(m, c_x + 1, c_y + 1, path, l + 1);
        check_paths(m, c_x + 1, c_y, path, l + 1);
        check_paths(m, c_x, c_y + 1, path, l + 1);
    }
}

fn try_candidate(m: &[[i32; MAX_Y]; MAX_X], x: usize, y: usize) -> bool {
    return (x < MAX_X) && (y < MAX_Y) && (m[x][y] == 0);
}

fn present_path(path: &[P]) {
    for p in path {
        print!("({},{}) ", p.x, p.y);
    }
    print!("\n\n");
}
