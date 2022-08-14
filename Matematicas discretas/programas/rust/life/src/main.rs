use nalgebra::matrix;

fn main() {
    let mut matriz = matrix![
        "0",".",".",".",".","0";
        ".",".","0","0",".",".";
        "0",".","0","0",".","0";
        ".","0",".",".","0",".";
        ".",".",".","0",".",".";
        "0",".",".",".","0",".";
    ];

    for i in 0..6 {
        for j in 0..6 {
            if check_live(matriz, i, j) {
                matriz[(i, j)] = ".";
            } else {
                matriz[(i, j)] = "0";
            }
        }
    }

    println!("{}", matriz);
}

fn check_live(matriz: i32, i: i32, j: i32) -> bool {
    todo!()
}
