trait Evaluable<T> {
    fn evaluate(&self) -> T;

    fn add(&self, other: Box<dyn Evaluable<T>>) -> Box<dyn Evaluable<T>>;
}

impl Evaluable<f64> for f64 {
    fn evaluate(&self) -> f64 {
        return *self;
    }

    fn add(&self, other: Box<dyn Evaluable<f64>>) -> Box<dyn Evaluable<f64>> {
        return Box::new(self + other.evaluate());
    }
}

#[cfg(test)]
mod evaluable_f64 {
    use super::*;

    #[test]
    fn evaluate() {
        assert_eq!(4f64, 4f64.evaluate());
    }

    #[test]
    fn add(){
        assert_eq!(20f64, 10f64.add(Box::new(10f64)).evaluate());
    }
}
