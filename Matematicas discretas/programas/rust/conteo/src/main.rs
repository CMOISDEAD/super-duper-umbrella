fn main() {
    println!("{}", 2)
}

// Counting permutations using math formula.
// fn prod_count(n: i32, r: i32) -> i32 {
//     factorial(n) / factorial(n - r)
// }
//
// fn factorial(number: i32) -> i32 {
//     let mut fact = 1;
//
//     for i in 1..number + 1 {
//         fact *= i;
//     }
//
//     fact
// }

//  Permutation of 5 objects taking 4 at time.
// fn permutations(numbers: [i32; 5]) {
//     let mut counter: i32 = 1;
//
//     for i in numbers {
//         for j in numbers {
//             if j != i {
//                 for k in numbers {
//                     if k != j && k != i {
//                         for l in numbers {
//                             if l != k && l != j && l != i {
//                                 println!("{}: {}{}{}{}", counter, i, j, k, l );
//                                 counter += 1;
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
