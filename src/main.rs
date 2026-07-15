struct Customer {
    name: String,
    surname: String,
    balance: f64,
}

struct Product {
    name: String,
    price: f64,
    stock: u32,
}

impl Customer {
    fn buy_product(&mut self, product: &mut Product, quantity: u32) -> bool {
        let total_price = product.price * quantity as f64;

        if product.stock >= quantity && self.balance >= total_price {
            product.stock -= quantity;
            self.balance -= total_price;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut customer1 = Customer {
        name: String::from("Arvin"),
        surname: String::from("Pramuditya"),
        balance: 150.0,
    };

    let mut customer2 = Customer {
        name: String::from("John"),
        surname: String::from("Doe"),
        balance: 80.0,
    };

    let mut product = Product {
        name: String::from("Laptop Stand"),
        price: 20.0,
        stock: 10,
    };

    println!("=== Market Simulation ===");

    println!("\nCustomer 1 is buying 3 {}...", product.name);
    if customer1.buy_product(&mut product, 3) {
        println!("Purchase successful!");
    } else {
        println!("Purchase failed.");
    }

    println!(
        "{}'s balance: ${:.2}",
        customer1.name, customer1.balance
    );
    println!("Remaining stock: {}\n", product.stock);

    println!("Customer 2 is buying 8 {}...", product.name);
    if customer2.buy_product(&mut product, 8) {
        println!("Purchase successful!");
    } else {
        println!("Purchase failed.");
    }

    println!(
        "{}'s balance: ${:.2}",
        customer2.name, customer2.balance
    );
    println!("Remaining stock: {}", product.stock);
}