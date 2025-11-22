// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }
// use std::cmp;

const APPLE_COST: u32 = 2;
const MAX_APPLES: u32 = 40;
const DISCOUNT: u32 = 1;

fn calculate_price_of_apples(n: u32) -> u32 {
    match n {
        _ if n>MAX_APPLES => (APPLE_COST-DISCOUNT)*n,
        _ => APPLE_COST*n
    }
    // match n.cmp(&MAX_APPLES) {
    //     cmp::Ordering::Greater => (APPLE_COST-DISCOUNT)*n,
    //     _ => APPLE_COST*n
    // }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
