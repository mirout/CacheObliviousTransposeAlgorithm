pub mod matrix {
    use std::fmt::{Display, Formatter};
    use std::fmt;
    use rand::RngCore;

    pub struct Matrix {
        n : usize,
        m : usize,
        matrix : Vec<Vec<i32>>,
    }

    impl Matrix {
        pub fn new(n : usize, m : usize) -> Matrix {
            Matrix {
                n,
                m,
                matrix: vec![vec![0; m]; n]
            }
        }

        pub fn new_random(n : usize, m : usize) -> Matrix {
            let mut matrix = vec![vec![0; m]; n];
            let mut rng = rand::thread_rng();
            for i in 0..n {
                for j in 0..m {
                    matrix[i][j] = rng.next_u32() as i32;
                }
            }

            Matrix {
                n,
                m,
                matrix,
            }
        }

        pub fn transpose(&self) -> Matrix {
            let mut result = Matrix::new(self.m, self.n);
            for i in 0..self.n {
                for j in 0..self.m {
                    result.matrix[j][i] = self.matrix[i][j];
                }
            }
            result
        }

        pub fn fast_transpose(&self) -> Matrix {
            let mut result: Matrix = Matrix::new(self.m, self.n);

            self.fast_transpose_with_recursion(0, 0, self.n, self.m, &mut result);

            result
        }

        fn fast_transpose_with_recursion(&self, x: usize, y: usize, dx: usize, dy: usize, out: &mut Matrix) {
            /*
            x, y - submatrix start
            dx, dy - submatrix size
            out - transposed matrix (size: MxN)
            */
            if dx == 1 && dy == 1 {
                out.matrix[y][x] = self.matrix[x][y];
            } else if dx >= dy {
                let mid = dx / 2;
                self.fast_transpose_with_recursion(x, y, mid, dy, out);
                self.fast_transpose_with_recursion(x+mid, y, dx - mid, dy, out);
            } else {
                let mid = dy / 2;
                self.fast_transpose_with_recursion(x, y, dx, mid, out);
                self.fast_transpose_with_recursion(x, y + mid, dx, dy - mid, out);
            }
        }
    }

    impl Display for Matrix {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.matrix)
        }
    }

    impl std::cmp::PartialEq for Matrix { //implementation trait to check for equality
        fn eq(&self, other: &Self) -> bool {
            self.matrix == other.matrix
        }
    }
}