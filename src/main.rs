fn main() {
    println!("--- 🍔 Welcome to the Smart Drive-Thru 🍔 ---");

    // starts at 100, increments each order
    let mut order_id = 100;

    // building the queue — using String so i can push names in
    let mut customer_name = String::from("");
    
    customer_name.push_str("\nAdey");
    customer_name.push_str("\nWani");
    customer_name.push_str("\nTemi");

    println!("\n Current Order ID: {}", order_id);
    println!("Customer at the window: {}", customer_name);

    println!("\n--- Processing the Loop ---");

    for count in 1..4 {
        order_id += 1;
        println!("Processing order #{}... Next ID is: {}", count, order_id);
    }

    // just using this to test the ownership concept — no GC, rust drops it at end of scope
    let cleanup_method = String::from("Ownership");

    if order_id == 103 && cleanup_method == "Ownership" {
        println!("\n Enjoy your order sir!");
    } else {
        println!("\nThe drive-thru machine is jammed. Check back again!");
    }
} // customer_name dropped here — heap freed automatically