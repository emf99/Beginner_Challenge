// TODO: Define your module structure here
// Hint: You'll need to create modules for products, users, orders, and inventory
use std::collections::HashMap;

mod product {
    /// Reprezentuje pojedynczy produkt w sklepie
    pub struct Product {
        pub id: u32,
        pub name: String,
        pub price: f64,
        pub description: String,
    }

    impl Product {
        /// Tworzy nowy obiekt Product
        pub fn new(id: u32, name: &str, price: f64, description: &str) -> Self {
            Self {
                id,
                name: name.to_string(),
                price,
                description: description.to_string(),
            }
        }

        /// Wyświetla informacje o produkcie
        pub fn display(&self) {
            println!("Product ID: {}", self.id);
            println!("Name: {}", self.name);
            println!("Price: ${:.2}", self.price);
            println!("Description: {}", self.description);
        }
    }
}

mod user {
    /// Reprezentuje użytkownika
    pub struct User {
        pub id: u32,
        pub name: String,
        pub email: String,
        pub address: String,
    }

    impl User {
        /// Tworzy nowego użytkownika
        pub fn new(id: u32, name: &str, email: &str, address: &str) -> Self {
            Self {
                id,
                name: name.to_string(),
                email: email.to_string(),
                address: address.to_string(),
            }
        }

        /// Wyświetla dane użytkownika
        pub fn display(&self) {
            println!("User ID: {}", self.id);
            println!("Name: {}", self.name);
            println!("Email: {}", self.email);
            println!("Address: {}", self.address);
        }
    }
}

mod order {
    use crate::product::Product;
    use crate::user::User;

    /// Status zamówienia
    #[derive(Debug)]
    pub enum OrderStatus {
        Pending,
        Shipped,
        Delivered,
        Cancelled,
    }

    /// Reprezentuje zamówienie z listą produktów i użytkownikiem
    pub struct Order {
        pub id: u32,
        pub user: User,
        pub products: Vec<Product>,
        pub status: OrderStatus,
    }

    impl Order {
        /// Tworzy nowe zamówienie (domyślnie ze statusem Pending)
        pub fn new(id: u32, user: User, products: Vec<Product>) -> Self {
            Self {
                id,
                user,
                products,
                status: OrderStatus::Pending,
            }
        }

        /// Aktualizuje status zamówienia
        pub fn update_status(&mut self, new_status: OrderStatus) {
            self.status = new_status;
        }

        /// Wyświetla szczegóły zamówienia
        pub fn display(&self) {
            println!("Order ID: {}", self.id);
            self.user.display();
            println!("Products:");
            for p in &self.products {
                p.display();
            }
            println!("Status: {:?}", self.status);
        }
    }
}

mod inventory {
    use std::collections::HashMap;

    /// Zarządza stanem magazynowym: dodawanie, sprawdzanie i usuwanie produktów
    pub struct Inventory {
        stock: HashMap<u32, u32>, // product_id -> quantity
    }

    impl Inventory {
        /// Tworzy nowe, puste Inventory
        pub fn new() -> Self {
            Self {
                stock: HashMap::new(),
            }
        }

        /// Dodaje określoną ilość produktu do magazynu
        pub fn add_stock(&mut self, product_id: u32, quantity: u32) {
            *self.stock.entry(product_id).or_insert(0) += quantity;
        }

        /// Usuwa określoną ilość produktu; zwraca true, jeśli operacja się powiodła
        pub fn remove_stock(&mut self, product_id: u32, quantity: u32) -> bool {
            if let Some(count) = self.stock.get_mut(&product_id) {
                if *count >= quantity {
                    *count -= quantity;
                    return true;
                }
            }
            false
        }

        /// Zwraca aktualny stan magazynowy dla danego produktu
        pub fn check_stock(&self, product_id: u32) -> u32 {
            *self.stock.get(&product_id).unwrap_or(&0)
        }
    }
}


fn main() {
   
    use product::Product;
    use user::User;
    use order::{Order, OrderStatus};
    use inventory::Inventory;

    let p1 = Product::new(1, "Smartphone", 699.99, "Latest model with 5G");
    let p2 = Product::new(2, "Laptop", 1299.99, "High performance laptop");
    let p3 = Product::new(3, "Headphones", 199.99, "Noise cancelling headphones");

    // 2. Add products to inventory
    let mut inventory = Inventory::new();
    inventory.add_stock(p1.id, 10);
    inventory.add_stock(p2.id, 5);
    inventory.add_stock(p3.id, 15);

    // 3. Create a user
    let user = User::new(1, "Alice Smith", "alice.smith@example.com", "456 Elm St, Warsaw");

    // 4. Create an order for the user with some products
    //    (Here we move p1 and p3 into the order)
    let mut order = Order::new(1001, user, vec![p1, p3]);

    // Update inventory for the items purchased
    inventory.remove_stock(1, 1); // 1 Smartphone sold
    inventory.remove_stock(3, 2); // 2 Headphones sold

    // 5. Print order details
    order.display();

    // Print remaining stock levels
    println!("\nRemaining stock:");
    println!("Product 1: {} units", inventory.check_stock(1));
    println!("Product 2: {} units", inventory.check_stock(2));
    println!("Product 3: {} units", inventory.check_stock(3));
}

    // TODO: Use your modules to create a simple e-commerce workflow:
    // 1. Create some products
    // 2. Add products to inventory
    // 3. Create a user
    // 4. Create an order for the user with some products
    // 5. Print order details
