// 4*1 + 0    -> 4*1 -> 4
// 4*3 + 4    -> 4*4 -> 16
// 4*5 + 16   -> 4*9 -> 36
// 4*7 + 36   -> 4*16 -> 64
// 4*9 + 64   -> 4*25 -> 100
// 4*11 + 100 -> 4*36 -> 144

// Ring size
// 1 -> 1

// 2 -> 4

// 3 -> 8   +7
// 1 -> 1

// 4 -> 12  +8
// 2 -> 4

// 5 -> 16  +8
// 3 -> 8   +7
// 1 -> 1

// 6 -> 20  +8
// 4 -> 12  +8
// 2 -> 4
fn get(i: u64, n: u64) -> String {
    if i == 0 {
        return "     ".to_string();
    }

    if n % 2 == 1 {
        let remapped_i = n*n-i;
        let layer = (remapped_i as f64 / 4.0).sqrt().floor();
        // let side;
        format!("{:3}:{}", remapped_i, layer)
    } else {
        "".to_string()
    }
}


// 1 2 3
// 8 0 4
// 7 6 5

// 8 7 6
// 1   5
// 2 3 4

fn test(arr: Vec<Vec<u64>>) {
    println!("");
    let n = arr.len() as u64;
    for row in arr {
        for cell in row {
            print!("{} ", get(cell, n));
        }
        println!("");
    }
    println!("");
}

fn main() {
    test(vec![
        vec![0]
    ]);

    test(vec![
        vec![1, 2],
        vec![0, 3]
    ]);

    test(vec![
        vec![1, 2, 3],
        vec![8, 0, 4],
        vec![7, 6, 5]
    ]);

    test(vec![
        vec![ 1,  2,  3,  4],
        vec![12, 13, 14,  5],
        vec![11,  0, 15,  6],
        vec![10,  9,  8,  7]
    ]);

    test(vec![
        vec![ 1,  2,  3,  4,  5],
        vec![16, 17, 18, 19,  6],
        vec![15, 24,  0, 20,  7],
        vec![14, 23, 22, 21,  8],
        vec![13, 12, 11, 10,  9]
    ]);
}