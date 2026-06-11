fn main() {
    println!("--- 🍔 Welcome to the Smart Drive-Thru 🍔 ---");

    // --- STACK ALLOCATION (Fast Memory) ---
    // Storing a fixed-size integer on the Stack. 
    // Marked as mutable ('mut') to allow runtime updates.
    let mut order_id = 100;

    // --- HEAP ALLOCATION (Dynamic Memory) ---
    // Creating a dynamic, heap-allocated String that can grow or shrink at runtime.
    let mut customer_name = String::from("");
    
    // Demonstrating dynamic growth by pushing data onto the heap memory structure.
    customer_name.push_str("\nAdey");
    customer_name.push_str("\nWani");
    customer_name.push_str("\nTemi");

    println!("\n Current Order ID: {}", order_id);
    println!("Customer at the window: {}", customer_name);

    println!("\n--- Processing the Loop ---");

    // --- CONTROL FLOW & BOUNDS ---
    // A fixed-range loop (1 to 3) using an exclusive upper bound (1..4).
    // Mutates the stack variable using the compound assignment operator (+=).
    for count in 1..4 {
        order_id += 1;
        println!("Processing order #{}... Next ID is: {}", count, order_id);
    }

    // --- MEMORY MANAGEMENT CHECK ---
    // Explicit tracking of Rust's unique compile-time memory safety mechanism.
    let cleanup_method = String::from("Ownership");

    // Validating final program state and confirming compile-time resource cleanup.
    if order_id == 103 && cleanup_method == "Ownership" {
        println!("\n Enjoy your order sir!");
    } else {
        println!("\nThe drive-thru machine is jammed. Check back again!");
    }
} // <-- At this closing bracket, 'customer_name' goes out of scope.
  // Rust automatically frees the Heap memory instantly via Ownership—no Garbage Collector needed!
