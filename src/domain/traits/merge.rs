


pub trait Mergable {
    fn merge(&self, other: &Self) -> Self;
}
