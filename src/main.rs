use num_traits::Float;
mod basket;
use basket::Basket;

fn solve<T: Float>(a: T, b: T) -> T {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
    let a = 3.0;
    let b = 4.0;
    println!("{}",solve(a,b));

    let basket = Basket::new(String::from("Basket"));
    let num_basket = Basket::new(10);

    println!("{:#?} {:#?}",basket,num_basket);

}
