use std::collections::HashMap;
use std::thread;
use std::time::Duration;

// Simulate a database with some product data
struct Database;

impl Database {
    fn get_prices(&self, product_id: u32) -> Vec<&str> {
        // Simulate a slow database query to fetch prices
        thread::sleep(Duration::from_secs(1));
        vec!["10", "20", "30"]
    }

    fn get_text(&self, product_id: u32) -> Vec<&'static str> {
        // Simulate a slow database query to fetch text
        thread::sleep(Duration::from_secs(1));
        vec!["Product A", "Product B", "Product C"]
    }
}

// Product struct for assembling and caching data
struct Product {
    product_id: u32,
    db: Database,
    data: Option<HashMap<&'static str, Vec<u32>>>,
}

impl Product {
    fn new(product_id: u32, db: Database) -> Product {
        Product {
            product_id,
            db,
            data: None,
        }
    }

    fn assemble_data(&mut self) {
        if self.data.is_none() {
            let prices = self.db.get_prices(self.product_id);
            let text = self.db.get_text(self.product_id);

            let mut data = HashMap::new();
            data.insert("prices", prices);
            data.insert("text", text);

            self.data = Some(data);
        }
    }
}

fn main() {
    let db = Database;

    // Simulate a cache (you can use a real cache system like Redis)
    let mut cache: HashMap<u32, Product> = HashMap::new();

    // Function to retrieve a product with caching
    fn get_product(product_id: u32, db: Database, cache: &mut HashMap<u32, Product>) -> &Product {
        if let Some(product) = cache.get(&product_id) {
            return product;
        }

        let product = Product::new(product_id, db);
        product.assemble_data();
        cache.insert(product_id, product);
        cache.get(&product_id).unwrap()
    }

    // Get product information
    let product1 = get_product(1, db, &mut cache);
    println!("Product 1 Data: {:?}", product1.data);

    // Get the same product again
    let product2 = get_product(1, db, &mut cache);
    println!("Product 2 Data: {:?}", product2.data);

    // Wait a bit to simulate a change in data
    thread::sleep(Duration::from_secs(2));

    // Get the product again after a change (refresh the cache)
    let product3 = get_product(1, db, &mut cache);
    println!("Product 3 Data: {:?}", product3.data);
}
