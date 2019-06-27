fn get_moves(towers: u32, left: &mut Vec<u32>, mid: &mut Vec<u32>, right: &mut Vec<u32>) -> u32 {
    if towers == 0 {
        return 0;
    }
    if left.len() == 0 || right.len() > 0 && left[0] < right[0] {
        let moves = 1 << (towers - 1);
        let mut right = right.clone();
        right.remove(0);
        return moves + get_moves(towers - 1, mid, left, &mut right);
    } else {
        let mut left = left.clone();
        left.remove(0);
        return get_moves(towers - 1, &mut left, right, mid);
    }
}

fn get_rods(moves: u32, towers: u32, left: &mut Vec<u32>, mid: &mut Vec<u32>, right: &mut Vec<u32>) {
    if towers > 0 {
        if (moves << 1) & (1 << towers) != 0{
            right.push(towers);
            get_rods(moves, towers - 1, mid, left, right);
        } else {
            left.push(towers);
            get_rods(moves, towers - 1, left, right, mid);
        }
    }
}


fn hanoi(towers: u32) {
    for i in 0..2u32.pow(towers) {
        let mut left = Vec::<u32>::new();
        let mut mid = Vec::<u32>::new();
        let mut right = Vec::<u32>::new();
        get_rods(i, towers, &mut left, &mut mid, &mut right);
        let moves = get_moves(towers, &mut left, &mut mid, &mut right);
        println!("{:2} moves -- {:?} {:?} {:?}", moves, left, mid, right);
    }
}

fn main() {
    hanoi(4);
}
