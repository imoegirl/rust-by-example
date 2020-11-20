fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (i, b) = pair;
    (b, i)
}

fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

pub fn run() {
    println!("p004_tuples >>>>>>>>");
    let t1 = (15, false);
    let rev_t1 = reverse(t1);
    println!("{:?} -> {:?}", t1, rev_t1);
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("matrix fmt: \n{:?}", matrix);
    println!("matrix display: \n{}", matrix);
    println!("matrix transposed: \n{}", transpose(matrix));
    println!("\n");
}
