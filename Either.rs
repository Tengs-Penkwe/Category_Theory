
#[derive(Debug)]
pub enum Either<T1, T2> {
    Left(T1),
    Right(T2),
}

impl<T1, T2> Either<T1, T2> {

    pub fn left(mut self, l: T1) -> Self {
        self = Either::Left(l);
        self
    }

    pub fn isLeft(&self) -> bool {
        match self {
            Either::Left(_) => true,
            _ => false,
        }
    }

    pub fn right(mut self, r: T2) -> Self {
        self = Either::Right(r);
        self
    }

    pub fn isRight(&self) -> bool {
        match self {
            Either::Right(_) => true,
            _ => false,
        }
    }
}

fn main () {
    let e: Either<i32, f32> = Either::Left(5);
    println!("{:?}", e.isRight());
    let e = e.right(5.0);
    println!("{:?}", e.isRight());
}
