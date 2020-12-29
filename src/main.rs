use std::time::SystemTime;

use cache_oblivious::matrix::Matrix;

fn main() {
    for _k in 0..4 {

        let mut result_naive = Vec::new();
        let mut result_fast = Vec::new();

        for n in (1000..35001).step_by(1000) {
            println!("Matrix size {0}x{0}", n);

            let a = Matrix::new_random(n, n);

            let start = SystemTime::now();
            a.transpose();
            result_naive.push(SystemTime::now().duration_since(start).unwrap().as_millis());

            let start = SystemTime::now();
            a.fast_transpose();
            result_fast.push(SystemTime::now().duration_since(start).unwrap().as_millis());
        }

        for i in result_naive {
            println!("{}", i);
        }

        println!("----------------");

        for i in result_fast {
            println!("{}", i);
        }

        println!("!---------------!");
    }
}
