// ========================================================================
// RUST FOR SOLANA BLOCKCHAIN DEVELOPMENT CHEAT SHEET
// ========================================================================
//
// This cheat sheet provides examples of Rust concepts with a focus on
// Solana blockchain development. It is organized from basic to advanced
// concepts, with detailed explanations to help newcomers understand
// the language features.

// ========================================================================
// 1. BASIC CONCEPTS
// ========================================================================

// ----------------------------------------
// Printing and Basic Output
// ----------------------------------------

fn printing_examples() {
    // Basic print
    println!("Hello, Solana!");

    // Print with variables
    let sol_balance = 42.5;
    println!("Account balance: {} SOL", sol_balance);

    // Formatting options
    println!("Balance in lamports: {:.0}", sol_balance * 1_000_000_000.0); // No decimal
    println!("Hex pubkey: {:#x}", 0x1234ABCD); // Hex format with 0x prefix

    // Debug printing (useful for complex data structures)
    let token_amounts = vec![10, 20, 30];
    println!("Token balances: {:?}", token_amounts); // Debug print
    println!("Token balances (pretty): {:#?}", token_amounts); // Pretty debug print

    // Printing errors
    eprintln!("Error: Transaction failed"); // Prints to stderr

    // Print without newline
    print!("Processing transaction... ");
    print!("Done!\n"); // Need to add newline manually
}

// ----------------------------------------
// Variables and Mutability
// ----------------------------------------

fn variables_and_mutability() {
    // Immutable by default (important for blockchain state security)
    let wallet_address = "8ZgMzTUqx7Uq7LUwHEfUvVi3FKEG5BEWdv25TpNB2mKa";
    println!("Wallet address: {}", wallet_address);

    // This would cause an error:
    // wallet_address = "different_address"; // Cannot reassign immutable variable

    // For mutable variables, use 'mut'
    let mut token_balance = 100;
    token_balance += 50; // We can modify mutable variables
    println!("Updated balance: {}", token_balance);

    // Constants (known at compile time)
    const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
    println!("1 SOL = {} lamports", LAMPORTS_PER_SOL);

    // In Solana programs, state is often kept immutable for security:
    let program_id = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    println!("Solana Token Program ID: {}", program_id);
}

// ----------------------------------------
// Shadowing
// ----------------------------------------

fn shadowing_examples() {
    // Shadowing allows reusing variable names
    let balance = "50";
    println!("Balance as string: {}", balance);

    // Shadow the previous variable with a new type
    let balance = balance.parse::<u64>().unwrap();
    println!("Balance as number: {}", balance);

    // Shadow with an expression
    let balance = balance * 2;
    println!("Doubled balance: {}", balance);

    // Useful in Solana when deserializing from bytes to structured data
    let data = [0, 1, 2, 3];
    let data = convert_bytes_to_instruction(&data); // shadow with processed data

    // Shadowing vs mutability:
    // 1. Shadowing creates a new variable (can change type)
    // 2. Mutation modifies existing variable (cannot change type)
}

// Helper function for the shadowing example
fn convert_bytes_to_instruction(data: &[u8]) -> String {
    // Just a dummy function for the example
    format!("Instruction: {:?}", data)
}

// ========================================================================
// 2. DATA TYPES AND CONTROL FLOW
// ========================================================================

// ----------------------------------------
// Basic Data Types
// ----------------------------------------

fn basic_data_types() {
    // Integers (commonly used in Solana for amounts, timestamps, etc.)
    let lamports: u64 = 5_000_000_000; // Unsigned 64-bit integer (no negative values)
    let block_height: i64 = 123_456_789; // Signed 64-bit integer

    // In Solana, lamports and token amounts usually use u64
    // because negative balances don't make sense

    // Floating-point (rarely used in blockchain for precision reasons)
    let sol_amount: f64 = 5.0; // Avoid for financial calculations!

    // Boolean
    let is_signer: bool = true;
    let is_writable: bool = false;

    // Characters
    let token_symbol: char = 'S';

    // Compound types

    // Tuple (fixed-length collection of different types)
    let transaction_info: (u64, bool, &str) = (
        lamports,
        is_signer,
        "8ZgMzTUqx7Uq7LUwHEfUvVi3FKEG5BEWdv25TpNB2mKa",
    );
    println!(
        "Tx amount: {}, Signed: {}, Address: {}",
        transaction_info.0, transaction_info.1, transaction_info.2
    );

    // Destructuring tuples
    let (amount, signed, address) = transaction_info;
    println!("Destructured - Amount: {}, Signed: {}", amount, signed);
}

// ----------------------------------------
// Control Flow
// ----------------------------------------

fn control_flow_examples() {
    // If expressions (expressions return values)
    let lamports = 1_500_000_000;
    let sol_amount = if lamports >= 1_000_000_000 {
        lamports as f64 / 1_000_000_000.0
    } else {
        0.0
    };
    println!("SOL amount: {}", sol_amount);

    // Match expressions (exhaustive pattern matching)
    let instruction_type = 2;
    let operation = match instruction_type {
        0 => "Initialize",
        1 => "Transfer",
        2 => "Mint",
        3 => "Burn",
        _ => "Unknown operation", // _ is a catch-all pattern
    };
    println!("Operation: {}", operation);

    // Loops
    let mut counter = 0;
    // Infinite loop with break
    loop {
        counter += 1;
        if counter >= 5 {
            break;
        }
    }

    // While loop
    let mut block_height = 100;
    while block_height < 105 {
        println!("Processing block: {}", block_height);
        block_height += 1;
    }

    // For loop (most common in Solana for iterating through accounts)
    let token_accounts = vec!["Acc1", "Acc2", "Acc3"];
    for account in token_accounts {
        println!("Checking token account: {}", account);
    }

    // For loop with range
    for i in 0..5 {
        println!("Index: {}", i); // 0, 1, 2, 3, 4
    }

    // For loop with inclusive range
    for i in 0..=5 {
        println!("Index: {}", i); // 0, 1, 2, 3, 4, 5
    }

    // Pattern matching with enums (common in Solana for handling instruction types)
    let result = TokenInstruction::Transfer;
    match result {
        TokenInstruction::Initialize => println!("Initializing token"),
        TokenInstruction::Transfer => println!("Transferring tokens"),
        TokenInstruction::Mint => println!("Minting new tokens"),
        TokenInstruction::Burn => println!("Burning tokens"),
    }
}

// Enum for the control flow example
enum TokenInstruction {
    Initialize,
    Transfer,
    Mint,
    Burn,
}

// ----------------------------------------
// Complex Pattern Matching
// ----------------------------------------

fn complex_pattern_matching() {
    // Matching with multiple patterns
    let status_code = 404;
    match status_code {
        200 => println!("Success"),
        400 | 401 | 403 => println!("Client error"),
        500..=599 => println!("Server error"),
        _ => println!("Unknown status"),
    }

    // Matching with guards
    let lamports: i64 = 5_000_000_000;
    match lamports {
        amount if amount > 1_000_000_000 => println!("More than 1 SOL"),
        amount if amount > 500_000_000 => println!("More than 0.5 SOL"),
        _ => println!("Less than 0.5 SOL"),
    }

    // Destructuring in match
    let tx_details = (200, 5_000_000, true);
    match tx_details {
        (200, _, true) => println!("Successful signed transaction"),
        (200, _, false) => println!("Successful unsigned transaction"),
        (code, amount, _) if code != 200 && amount > 1_000_000 => {
            println!("Failed high-value transaction")
        }
        _ => println!("Other transaction status"),
    }

    // Complex pattern matching with Option<T>
    let maybe_pubkey: Option<&str> = Some("8ZgMzTUqx7Uq7LUwHEfUvVi3FKEG5BEWdv25TpNB2mKa");
    match maybe_pubkey {
        Some(pubkey) if pubkey.starts_with("8") => {
            println!("Valid public key starting with 8")
        }
        Some(pubkey) => println!("Other public key: {}", pubkey),
        None => println!("No public key provided"),
    }

    // If-let for simpler Option matching
    if let Some(pubkey) = maybe_pubkey {
        println!("Found public key: {}", pubkey);
    }

    // While-let for conditional loops
    let mut token_amounts = vec![100, 200, 300];
    while let Some(amount) = token_amounts.pop() {
        println!("Processing amount: {}", amount);
    }
}

// ========================================================================
// 3. MEMORY MANAGEMENT: STACK vs HEAP
// ========================================================================

fn stack_vs_heap_examples() {
    // STACK: Fixed size, fast access, follows LIFO (Last In, First Out)
    // - Primitive types (i32, bool, etc.)
    // - Fixed-size arrays
    // - References (&T)
    // - Function calls and local variables

    // HEAP: Dynamic size, slower access, no specific order
    // - Dynamic data (Vec, String, Box, etc.)
    // - Data whose size may change
    // - Data that needs to live beyond a function call

    // --------- STACK EXAMPLES ---------

    // These are all stored on the stack:
    let stack_int: u64 = 123456789; // Fixed size (8 bytes)
    let stack_bool: bool = true; // Fixed size (1 byte)
    let stack_array: [u8; 4] = [1, 2, 3, 4]; // Fixed size array (4 bytes)

    // Function call stack frames also go on the stack

    // --------- HEAP EXAMPLES ---------

    // String data is stored on the heap (the String struct itself is on the stack)
    let heap_string = String::from("Solana Public Key");

    // Vector - elements are contiguous in heap memory
    let heap_vector = vec![10, 20, 30, 40];

    // Box - single value on the heap
    let heap_box = Box::new(50);

    // --------- HEAP EXPLANATION ---------

    // In Solana, understanding heap vs stack is crucial because:
    // 1. Solana programs have limited compute budget
    // 2. Memory allocation on heap is more expensive
    // 3. Efficient memory use means lower transaction costs

    // When you create a String:
    let s = String::from("Hello Solana");
    // What happens:
    // 1. Memory is requested from heap allocator
    // 2. String struct on stack contains:
    //    - Pointer to data on heap
    //    - Length (12 in this case)
    //    - Capacity (how much memory is reserved)

    // When you use a vector:
    let v = vec![1, 2, 3];
    // Similarly:
    // 1. Contiguous memory allocated on heap
    // 2. Vec struct on stack contains:
    //    - Pointer to data
    //    - Length (3)
    //    - Capacity

    // Example of stack vs heap in Solana context
    {
        // Program ID is typically a fixed-size array on the stack
        let program_id: [u8; 32] = [0; 32]; // 32 bytes on stack

        // Account data is dynamic and goes on the heap
        let account_data: Vec<i32> = Vec::new(); // Pointer, len, cap on stack; actual data on heap

        // When working with actual Solana programs:
        // - Account inputs/references: stack
        // - Deserialized account data: heap
        // - Instruction data: heap
    }

    // Visualizing the box
    println!("Box value: {}", *heap_box); // Dereference to get the value
}

// ========================================================================
// 4. REFERENCES, BORROWING, AND OWNERSHIP
// ========================================================================

// ----------------------------------------
// Ownership Basics
// ----------------------------------------

fn ownership_basics() {
    // OWNERSHIP RULES:
    // 1. Each value has an owner
    // 2. There can only be one owner at a time
    // 3. When owner goes out of scope, value is dropped

    // Example: String
    {
        let wallet = String::from("8ZgMzTUqx7Uq7LUwHEfUvVi3FKEG5BEWdv25TpNB2mKa");
        // wallet is the owner of the string data

        // wallet goes out of scope here, memory is freed automatically
    }

    // Move semantics
    let token_name = String::from("Solana");
    let token_name2 = token_name; // Ownership moves to token_name2

    // This would fail because token_name no longer owns the data:
    // println!("Token: {}", token_name); // Error!
    println!("Token: {}", token_name2); // Works

    // For primitive types that implement Copy trait, values are copied instead of moved
    let lamports = 50_000_000;
    let lamports2 = lamports; // lamports is copied, not moved

    // Both work fine
    println!("Lamports1: {}", lamports);
    println!("Lamports2: {}", lamports2);

    // Function ownership
    let program_id = String::from("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    takes_ownership(program_id);
    // program_id is no longer valid here

    let new_id = gives_ownership();
    println!("New ID: {}", new_id);

    // Returning ownership
    let input_id = String::from("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
    let output_id = takes_and_gives_back(input_id);
    println!("Output ID: {}", output_id);
}

// Function that takes ownership
fn takes_ownership(program_id: String) {
    println!("Now I own: {}", program_id);
    // program_id goes out of scope and is dropped
}

// Function that gives ownership
fn gives_ownership() -> String {
    let id = String::from("MangoCzJ36AjZyKwVj3VnYU4GTonjfVEnJmvvWaxLac");
    id // Return and transfer ownership
}

// Function that takes and returns ownership
fn takes_and_gives_back(program_id: String) -> String {
    // Takes ownership and then returns it
    program_id
}

// ----------------------------------------
// Referencing, Dereferencing, and Pointers
// ----------------------------------------

fn references_and_borrowing() {
    // REFERENCE RULES:
    // 1. At any time, you can have EITHER:
    //    - One mutable reference (&mut T)
    //    - Any number of immutable references (&T)
    // 2. References must always be valid (no dangling references)

    // ------ IMMUTABLE REFERENCES ------

    // The & symbol creates a reference (a pointer that doesn't take ownership)
    let wallet = String::from("8ZgMzTUqx7Uq7LUwHEfUvVi3FKEG5BEWdv25TpNB2mKa");

    // Create a reference to wallet
    let wallet_ref = &wallet;

    // Both can be used - wallet_ref doesn't take ownership
    println!("Original: {}", wallet);
    println!("Reference: {}", wallet_ref);

    // Pass an immutable reference to a function
    calculate_address_hash(&wallet);
    println!("Wallet still valid: {}", wallet);

    // Multiple immutable references are allowed
    let wallet_ref2 = &wallet;
    let wallet_ref3 = &wallet;
    println!(
        "Multiple refs: {}, {}, {}",
        wallet, wallet_ref2, wallet_ref3
    );

    // ------ MUTABLE REFERENCES ------

    let mut balance = 100;

    // Create a mutable reference
    let balance_ref = &mut balance;

    // Modify the value through the reference
    *balance_ref += 50; // The * operator dereferences

    println!("Updated balance: {}", balance);

    // IMPORTANT: Cannot have other references while a mutable ref exists
    {
        let mut token_supply = String::from("1000000");

        // Create a mutable reference
        let supply_ref = &mut token_supply;

        // These would fail:
        // let supply_ref2 = &mut token_supply; // Error: cannot borrow as mutable more than once
        // let supply_ref3 = &token_supply;    // Error: cannot borrow as immutable while mutable borrow

        // Modify through mutable reference
        supply_ref.push_str(" SPL");
        println!("Modified supply: {}", supply_ref);
    } // supply_ref goes out of scope here

    // Example showing the alternating pattern
    let mut token_name = String::from("SOL");

    {
        let r1 = &token_name; // immutable borrow
        let r2 = &token_name; // another immutable borrow
        println!("Immutable refs: {} and {}", r1, r2);
        // r1 and r2 are no longer used after this point
    }

    // Now we can use a mutable reference
    let r3 = &mut token_name;
    r3.push_str("ANA");
    println!("Modified: {}", r3);

    // ------ THE * OPERATOR (DEREFERENCE) ------

    // * is used to access the value a reference points to
    let x = 5;
    let y = &x;
    assert_eq!(5, *y); // Use * to "follow" the reference to the value

    // With mutable references
    let mut num = 10;
    {
        let num_ref = &mut num;
        *num_ref = 15; // * is used to get to the actual value to modify it
    }
    println!("Modified num: {}", num);

    // Box<T> - a pointer to heap data that owns the data
    let heap_num = Box::new(42);
    println!("Box value: {}", *heap_num); // * dereferences the box

    // ------ REFERENCES IN SOLANA CONTEXT ------

    // In Solana programs, references are used extensively for:
    // 1. Account data (to avoid copying large data)
    // 2. Program inputs (passing by reference is more efficient)
    // 3. Multiple account checks without ownership transfer

    // Example: Typical Solana program function might look like:
    // process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> Result<(), Error>

    // Simulate a simple processing
    let program_id = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    let account_data = vec![1, 2, 3, 4];
    process_token_instruction(&program_id, &account_data);
}

// Function that borrows a reference
fn calculate_address_hash(address: &String) {
    // Using address without taking ownership
    println!("Processing address (len: {}): {}", address.len(), address);
    // address reference goes out of scope, but the String is still owned by the caller
}

// Simulate a Solana token instruction processing
fn process_token_instruction(program_id: &str, account_data: &Vec<u8>) {
    println!("Program {} processing data: {:?}", program_id, account_data);
    // Using references allows us to read the data without taking ownership
}

// ----------------------------------------
// Lifetime Parameters
// ----------------------------------------

fn lifetime_examples() {
    // Lifetime parameters ensure references remain valid

    // LIFETIME BASICS:

    // Problem: This function would fail without lifetime parameters
    // fn longest(s1: &str, s2: &str) -> &str { ... }

    // The compiler needs to know that the returned reference
    // will be valid for at least as long as the inputs

    // With lifetime parameters
    let string1 = String::from("solana");
    let string2 = String::from("blockchain");

    // 'a tells the compiler that the return value lives at least as long
    // as the shortest of the two input lifetimes
    let result = longest(&string1, &string2);
    println!("Longest string: {}", result);

    // ------ LIFETIME ELISION ------

    // Many common cases don't need explicit lifetime parameters
    // because the compiler follows these rules:

    // 1. Each reference parameter gets its own lifetime
    // 2. If there's exactly one input lifetime, it's assigned to all outputs
    // 3. If there's a &self or &mut self, its lifetime is assigned to all outputs

    // ------ LIFETIME IN STRUCTS ------

    // When a struct holds references, it needs lifetime parameters
    {
        // TokenAccount holds a reference, so it needs a lifetime parameter
        let owner_name = String::from("Alice");

        // This struct will be valid as long as the owner_name is valid
        let account = TokenAccount {
            amount: 100,
            owner: &owner_name,
        };

        println!("Account owner: {}", account.owner);
    } // Both owner_name and account go out of scope here

    // ------ STATIC LIFETIME ------

    // 'static means the reference can live for the entire program duration
    let static_str: &'static str = "I live forever in the program binary";
    println!("Static: {}", static_str);
}

// Function with lifetime parameter
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // 'a says that the return value will live at least as long
    // as both input parameters
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// Struct with lifetime parameter
struct TokenAccount<'a> {
    amount: u64,
    owner: &'a str, // This reference needs a lifetime parameter
}

// ========================================================================
// 5. STRINGS AND SLICES
// ========================================================================

fn string_and_slice_examples() {
    // ---- TWO STRING TYPES IN RUST ----

    // 1. &str - String slice (view into string data)
    //    - Fixed size
    //    - Immutable
    //    - Often stored in program binary (for literals)
    //    - Fat pointer (ptr + length)

    // 2. String - Owned string type
    //    - Growable
    //    - Mutable
    //    - Heap-allocated
    //    - Struct containing ptr, len, capacity

    // ----- STRING SLICES (&str) -----

    // String literals are &str (stored in program binary)
    let program_name: &str = "Solana Token Program";

    // String slice = reference to part of a string
    // Format: &str OR &String[start_idx..end_idx]

    let full_key = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    let prefix = &full_key[0..5]; // "Token"
    println!("Key prefix: {}", prefix);

    // Shorthand for starting at index 0
    let prefix_alt = &full_key[..5]; // Also "Token"

    // Shorthand for going to the end
    let suffix = &full_key[5..]; // "kegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"

    // Full slice (entire string)
    let full_slice = &full_key[..]; // Same as full_key

    println!("Variations: {}, {}, {}", prefix_alt, suffix, full_slice);

    // ----- OWNED STRINGS (String) -----

    // Create empty growable String
    let mut token_symbol = String::new();

    // Add data
    token_symbol.push_str("SO");
    token_symbol.push('L');
    println!("Token symbol: {}", token_symbol);

    // Create String from &str
    let blockchain = String::from("Solana");
    let blockchain_copy = "Solana".to_string();

    // Concatenation
    let greeting = String::from("Hello ");
    let audience = String::from("Solana Developers");

    // Using + operator (note: it takes ownership of the first String)
    let message = greeting + &audience; // greeting is moved here and can't be used after
    println!("Message: {}", message);

    // Better way for multiple strings: format! macro
    let part1 = "Solana";
    let part2 = "is";
    let part3 = "fast";
    let combined = format!("{} {} {}!", part1, part2, part3);
    println!("Formatted: {}", combined);

    // ----- WHY TWO STRING TYPES? -----

    // 1. Memory efficiency:
    //    - &str is just a view (no ownership, no allocation)
    //    - Good for function parameters when you don't need ownership

    // 2. Flexibility:
    //    - String when you need to own and modify
    //    - &str when you just need to read

    // 3. In Solana context:
    //    - Program IDs and fixed identifiers: often &str
    //    - Dynamic user inputs or constructed data: String

    // Function that just needs to read a string (use &str)
    fn print_program_id(id: &str) {
        println!("Program ID: {}", id);
    }

    // Both work with the function
    let id1 = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    let id2 = String::from("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

    print_program_id(id1);
    print_program_id(&id2); // Need & for String -> &str conversion

    // ----- STRINGS IN SOLANA CONTEXT -----

    // Account data is often serialized/deserialized
    let serialized_data = "01000000050000004e414d4500"; // hex representation

    // Extract field (in real code, use proper deserialization)
    let field_slice = &serialized_data[10..18]; // "4e414d45"
    println!("Encoded field: {}", field_slice);

    // Convert a &str to bytes (useful for Solana instruction data)
    let instruction = "transfer";
    let instruction_bytes = instruction.as_bytes();
    println!("Instruction as bytes: {:?}", instruction_bytes);
}

// ========================================================================
// 6. SLICES (GENERAL CONCEPT)
// ========================================================================

fn slice_examples() {
    // Slices = References to contiguous sequence of elements
    // - Type: &[T]
    // - Fat pointer (address + length)
    // - Allow borrowing part of a collection

    // ----- ARRAY SLICES -----

    // Array (fixed size, stack allocated)
    let token_amounts = [100, 200, 300, 400, 500];

    // Slice of the array (borrowing a section)
    let mid_amounts = &token_amounts[1..4]; // [200, 300, 400]
    println!("Mid amounts: {:?}", mid_amounts);

    // Process array slice
    println!("Sum of slice: {}", sum_of_amounts(mid_amounts));

    // ----- VECTOR SLICES -----

    // Vector (dynamic size, heap allocated)
    let accounts = vec!["Alice", "Bob", "Charlie", "Dave", "Eve"];

    // Create a slice
    let some_accounts = &accounts[2..]; // ["Charlie", "Dave", "Eve"]
    println!("Some accounts: {:?}", some_accounts);

    // Passing slice to function
    print_accounts(some_accounts);

    // ----- BYTE SLICES -----

    // Byte slice - commonly used in Solana for instruction data
    let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Create slices
    let header = &data[..2]; // [0, 1]
    let payload = &data[2..8]; // [2, 3, 4, 5, 6, 7]
    let checksum = &data[8..]; // [8, 9]

    println!(
        "Instruction parts - Header: {:?}, Payload: {:?}, Checksum: {:?}",
        header, payload, checksum
    );

    // ----- SLICES IN SOLANA CONTEXT -----

    // Solana instruction data is often passed as &[u8]
    let transaction_data: Vec<u8> = vec![2, 0, 0, 0, 100, 0, 0, 0, 50, 0, 0, 0];

    // Process different parts of the instruction
    process_instruction(&transaction_data);

    // In Solana, account data is often accessed via slices
    let account_data = vec![
        1, 0, 0, 0, 255, 255, 255, 255, 5, 0, 0, 0, 83, 79, 76, 47, 85, 83, 68, 67,
    ];

    // Get the instruction type (first byte)
    let instruction_type = account_data[0];

    // Get the token name (bytes 8-12 in this example)
    let name_length = account_data[8] as usize; // 5 bytes
    let name_slice = &account_data[9..9 + name_length];

    // Convert bytes to string (in real code, use proper error handling)
    let name = std::str::from_utf8(name_slice).unwrap_or("Invalid UTF-8");
    println!("Token pair name: {}", name); // "SOL/USDC"

    // Slices help efficiently process parts of serialized data
    // without needing to copy the entire dataset
}

// Helper function for sum calculation
fn sum_of_amounts(amounts: &[i32]) -> i32 {
    let mut sum = 0;
    for amount in amounts {
        sum += amount;
    }
    sum
}

// Function that takes a string slice
fn print_accounts(accounts: &[&str]) {
    println!("Accounts:");
    for (i, account) in accounts.iter().enumerate() {
        println!("  {}. {}", i + 1, account);
    }
}

// Simulate processing a Solana instruction from bytes
fn process_instruction(data: &[u8]) {
    if data.len() < 4 {
        println!("Invalid instruction: too short");
        return;
    }

    // In Solana, first 4 bytes often indicate instruction type
    let instruction_type = data[0];

    match instruction_type {
        0 => println!("Initialize instruction"),
        1 => println!("Create account instruction"),
        2 => {
            println!("Transfer instruction");
            if data.len() >= 12 {
                // In real Solana code, use proper deserialization libraries
                let amount = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                let recipient_id = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
                println!("  Amount: {}, Recipient ID: {}", amount, recipient_id);
            }
        }
        _ => println!("Unknown instruction type: {}", instruction_type),
    }
}

// ========================================================================
// 7. GENERICS AND TRAITS
// ========================================================================

// Generics allow you to write flexible, reusable code that works with
// different types while maintaining type safety.

// ----------------------------------------
// Basic Generics
// ----------------------------------------

// Generic function that works with any type
fn print_value<T: std::fmt::Debug>(value: T) {
    println!("Value: {:?}", value);
}

// Generic struct that can hold any type
struct Holder<T> {
    value: T,
}

// Generic implementation
impl<T> Holder<T> {
    fn new(value: T) -> Self {
        Holder { value }
    }

    fn get_value(&self) -> &T {
        &self.value
    }
}

// Function showing usage of generics
fn generic_examples() {
    // Using generic function with different types
    print_value(42);
    print_value("Solana");
    print_value(true);

    // Using generic struct with different types
    let int_holder = Holder::new(50);
    let string_holder = Holder::new(String::from("SPL Token"));

    println!("Int holder value: {}", int_holder.get_value());
    println!("String holder value: {}", string_holder.get_value());

    // Generic vector operations
    let numbers = vec![1, 2, 3, 4, 5];
    let first = get_first(&numbers);
    match first {
        Some(value) => println!("First value: {}", value),
        None => println!("Vector is empty"),
    }

    // In Solana, generics are often used for account serialization/deserialization
    // Example: Simulating a token account with generic owner type
    let token_account = TokenAccount2 {
        mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        owner: "8ZgMzTUqx7Uq7LUwHEfUvVi3FKEG5BEWdv25TpNB2mKa",
        amount: 100,
    };

    // Process the account
    process_token_account(&token_account);
}

// Generic function that works with any collection type
fn get_first<T: Copy>(collection: &[T]) -> Option<T> {
    if collection.is_empty() {
        None
    } else {
        Some(collection[0])
    }
}

// Generic struct for token account
struct TokenAccount2<'a> {
    mint: &'a str,
    owner: &'a str,
    amount: u64,
}

// Process the generic token account
fn process_token_account<'a>(account: &TokenAccount2<'a>) {
    println!("Processing token account:");
    println!("  Mint: {}", account.mint);
    println!("  Owner: {}", account.owner);
    println!("  Amount: {}", account.amount);
}

// ----------------------------------------
// Traits (similar to interfaces in other languages)
// ----------------------------------------

// Define a trait (interface)
pub trait Transaction {
    fn signature(&self) -> String;
    fn amount(&self) -> u64;
    fn verify(&self) -> bool;

    // Default implementation (can be overridden)
    fn is_valid(&self) -> bool {
        self.verify() && self.amount() > 0
    }
}

// Implement the trait for a specific type
struct TokenTransfer {
    from: String,
    to: String,
    amount_lamports: u64,
    sig: String,
}

// Implement Transaction trait for TokenTransfer
impl Transaction for TokenTransfer {
    fn signature(&self) -> String {
        self.sig.clone()
    }

    fn amount(&self) -> u64 {
        self.amount_lamports
    }

    fn verify(&self) -> bool {
        // In a real implementation, this would verify the signature
        // For this example, we'll just check if it starts with a specific prefix
        self.sig.starts_with("0x")
    }

    // We're using the default implementation for is_valid()
}

// Another type implementing the same trait
struct NFTTransfer {
    collection: String,
    token_id: u64,
    new_owner: String,
    signed_by: String,
    sig: String,
}

impl Transaction for NFTTransfer {
    fn signature(&self) -> String {
        self.sig.clone()
    }

    fn amount(&self) -> u64 {
        1 // NFTs are usually single tokens
    }

    fn verify(&self) -> bool {
        !self.sig.is_empty() && self.signed_by == "authority"
    }

    // Override the default implementation
    fn is_valid(&self) -> bool {
        self.verify() && self.token_id > 0 && !self.collection.is_empty()
    }
}

// Function showing usage of traits
fn trait_examples() {
    // Create instances of structs implementing Transaction
    let token_tx = TokenTransfer {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount_lamports: 5_000_000_000,
        sig: "0x123abc".to_string(),
    };

    let nft_tx = NFTTransfer {
        collection: "Solana Monkeys".to_string(),
        token_id: 42,
        new_owner: "Charlie".to_string(),
        signed_by: "authority".to_string(),
        sig: "valid_sig".to_string(),
    };

    // Use trait methods
    process_transaction(&token_tx);
    process_transaction(&nft_tx);

    // Trait bounds with generics (trait as a constraint)
    let transactions: Vec<Box<dyn Transaction>> = vec![Box::new(token_tx), Box::new(nft_tx)];

    validate_transactions(&transactions);
}

// Function that takes any type implementing Transaction trait
fn process_transaction(tx: &impl Transaction) {
    println!("Processing transaction:");
    println!("  Signature: {}", tx.signature());
    println!("  Amount: {}", tx.amount());
    println!("  Valid: {}", tx.is_valid());
}

// Function with trait bounds
fn validate_transactions(txs: &[Box<dyn Transaction>]) {
    println!("Validating {} transactions...", txs.len());

    for (i, tx) in txs.iter().enumerate() {
        println!("Transaction #{}: Valid = {}", i + 1, tx.is_valid());
    }
}

// ----------------------------------------
// Trait Objects for Runtime Polymorphism
// ----------------------------------------

fn trait_objects_example() {
    // In Solana, you might need to handle different account types

    // Create a vector of trait objects (dynamic dispatch)
    let accounts: Vec<Box<dyn Account>> = vec![
        Box::new(UserAccount {
            name: "Alice".to_string(),
            lamports: 50_000_000,
        }),
        Box::new(ProgramAccount {
            id: "TokenProg".to_string(),
            is_executable: true,
        }),
    ];

    // Process different account types uniformly
    for account in &accounts {
        account.display_info();
        println!("Rent-exempt: {}", account.is_rent_exempt());
    }

    // In actual Solana development, this pattern allows processing
    // different instruction or account types with unified code
}

// Account trait for the example
trait Account {
    fn lamports(&self) -> u64;
    fn display_info(&self);

    // Default implementation
    fn is_rent_exempt(&self) -> bool {
        self.lamports() >= 890_880 // Example minimum for rent exemption
    }
}

// User account implementation
struct UserAccount {
    name: String,
    lamports: u64,
}

impl Account for UserAccount {
    fn lamports(&self) -> u64 {
        self.lamports
    }

    fn display_info(&self) {
        println!(
            "User Account: {}, Balance: {} lamports",
            self.name, self.lamports
        );
    }
}

// Program account implementation
struct ProgramAccount {
    id: String,
    is_executable: bool,
}

impl Account for ProgramAccount {
    fn lamports(&self) -> u64 {
        // Program accounts typically have more lamports for rent exemption
        1_000_000
    }

    fn display_info(&self) {
        println!(
            "Program Account: {}, Executable: {}",
            self.id, self.is_executable
        );
    }
}

// ========================================================================
// 8. ARRAYS AND VECTORS
// ========================================================================

fn arrays_and_vectors() {
    // ---- ARRAYS ----

    // Arrays have fixed size known at compile time
    // Type is [T; size]

    // Initialize an array with values
    let token_ids: [u8; 4] = [1, 2, 3, 4];

    // Initialize an array with the same value repeated
    let zero_balances: [u64; 10] = [0; 10];

    // Access by index
    println!("First token ID: {}", token_ids[0]);

    // Get array length
    println!("Number of token IDs: {}", token_ids.len());

    // Arrays are stack allocated (fixed size)
    // Good for small, fixed-size collections like Solana public keys

    // Iterate over array values
    for id in token_ids {
        println!("Processing token ID: {}", id);
    }

    // Iterate with index
    for (i, balance) in zero_balances.iter().enumerate() {
        println!("Account {}: {} tokens", i, balance);
    }

    // ---- VECTORS ----

    // Vectors have dynamic size
    // Stored on the heap

    // Create an empty vector
    let mut token_balances: Vec<u64> = Vec::new();

    // Add elements
    token_balances.push(100);
    token_balances.push(200);
    token_balances.push(300);

    // Create vector with initial values
    let addresses = vec!["addr1", "addr2", "addr3"];

    // Access by index
    println!("Second balance: {}", token_balances[1]);

    // Safe access with get (returns Option<&T>)
    match token_balances.get(5) {
        Some(balance) => println!("Balance: {}", balance),
        None => println!("Index out of bounds"),
    }

    // Remove last element
    let last = token_balances.pop(); // Returns Option<T>
    println!("Removed balance: {:?}", last);

    // Check if empty
    println!("Is empty: {}", token_balances.is_empty());

    // Get length
    println!("Number of balances: {}", token_balances.len());

    // Replace an element
    if token_balances.len() > 0 {
        token_balances[0] = 150;
    }

    // Clear all elements
    token_balances.clear();

    // Reserve capacity (optimization when you know you'll add more elements)
    token_balances.reserve(10);

    // ---- VECTOR VS ARRAY COMPARISONS ----

    // 1. Memory Allocation
    // - Arrays: Stack (fixed size, faster for small data)
    // - Vectors: Heap (dynamic size, better for large or growing data)

    // 2. Size Flexibility
    // - Arrays: Fixed size determined at compile time
    // - Vectors: Can grow or shrink at runtime

    // 3. Performance
    // - Arrays: Slightly better performance when size is known
    // - Vectors: Small overhead for managing heap allocation

    // 4. Common Uses in Solana
    // - Arrays: For fixed-size data like public keys, signatures (32 bytes)
    // - Vectors: For dynamic data like instruction arrays, variable account data

    // Example: Fixed-size Solana public key as an array
    let pubkey: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ];

    // Example: Dynamic account list as a vector
    let mut accounts = Vec::new();
    accounts.push("Treasury");
    accounts.push("User1");
    accounts.push("User2");

    // ---- ADVANCED VECTOR OPERATIONS ----

    // Create a vector with capacity
    let mut instructions = Vec::with_capacity(5);

    // Add elements
    for i in 0..3 {
        instructions.push(format!("Instruction {}", i));
    }

    // Vector with different types using enums
    #[derive(Debug)]
    enum TokenAmount {
        Sol(f64),
        Spl(u64, String), // amount, token address
    }

    let mut balances = Vec::new();
    balances.push(TokenAmount::Sol(1.5));
    balances.push(TokenAmount::Spl(1000, "Token1".to_string()));

    // Process different types in a vector
    for balance in &balances {
        match balance {
            TokenAmount::Sol(amount) => println!("SOL balance: {}", amount),
            TokenAmount::Spl(amount, token) => println!("SPL token {}: {}", token, amount),
        }
    }

    // Sorting a vector
    let mut values = vec![5, 2, 8, 1, 9];
    values.sort();
    println!("Sorted values: {:?}", values);

    // Filter elements (creating a new vector)
    let high_values: Vec<i32> = values.iter().filter(|&&x| x > 5).copied().collect();
    println!("High values: {:?}", high_values);

    // Transform elements (map)
    let doubled: Vec<i32> = values.iter().map(|&x| x * 2).collect();
    println!("Doubled values: {:?}", doubled);
}

// ========================================================================
// 9. ITERATORS
// ========================================================================

fn iterator_examples() {
    // Iterators provide a way to process sequences of elements

    // ---- BASIC ITERATOR USAGE ----

    // Create a vector
    let token_balances = vec![100, 200, 300, 400, 500];

    // Get an iterator
    let mut iter = token_balances.iter();

    // Manually iterate
    println!("Manual iteration:");
    println!("  Next: {:?}", iter.next()); // Some(&100)
    println!("  Next: {:?}", iter.next()); // Some(&200)
    println!("  Next: {:?}", iter.next()); // Some(&300)

    // Using for loop (creates an iterator automatically)
    println!("For loop iteration:");
    for balance in &token_balances {
        println!("  Balance: {}", balance);
    }

    // ---- ITERATOR ADAPTORS ----

    // map - transform each element
    let doubled: Vec<i32> = token_balances.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // filter - keep elements that match predicate
    let high_balances: Vec<&i32> = token_balances.iter().filter(|&&x| x > 200).collect();
    println!("High balances: {:?}", high_balances);

    // enumerate - add indices
    for (i, balance) in token_balances.iter().enumerate() {
        println!("Index {}: {}", i, balance);
    }

    // chain - concatenate iterators
    let more_balances = vec![600, 700];
    let all_balances: Vec<&i32> = token_balances.iter().chain(more_balances.iter()).collect();
    println!("All balances: {:?}", all_balances);

    // zip - combine two iterators
    let accounts = vec!["Alice", "Bob", "Charlie", "Dave"];
    // let account_info: Vec<(&str, &i32)> = accounts
    //     .iter()
    //     .zip(token_balances.iter())
    //     .take(3) // Only take first 3 pairs
    //     .collect();

    // println!("Account info: {:?}", account_info);

    // ---- CONSUMING ADAPTORS ----

    // sum
    let total: i32 = token_balances.iter().sum();
    println!("Total balance: {}", total);

    // any - returns true if any element satisfies predicate
    let has_large_balance = token_balances.iter().any(|&x| x > 400);
    println!("Has large balance: {}", has_large_balance);

    // all - returns true if all elements satisfy predicate
    let all_positive = token_balances.iter().all(|&x| x > 0);
    println!("All positive: {}", all_positive);

    // find - returns first element that matches predicate
    if let Some(&balance) = token_balances.iter().find(|&&x| x > 300) {
        println!("First balance > 300: {}", balance);
    }

    // fold - accumulate value starting with initial value
    let sum_plus_1000 = token_balances.iter().fold(1000, |acc, &x| acc + x);
    println!("Sum + 1000: {}", sum_plus_1000);

    // ---- CREATING ITERATORS ----

    // From range
    let nums: Vec<i32> = (1..6).collect();
    println!("Range: {:?}", nums);

    // Repeat a value
    let zeroes: Vec<i32> = std::iter::repeat(0).take(5).collect();
    println!("Zeroes: {:?}", zeroes);

    // ---- ITERATOR EXAMPLES IN SOLANA CONTEXT ----

    // Simulating processing multiple token accounts
    struct TokenAcct {
        owner: String,
        amount: u64,
    }

    let token_accounts = vec![
        TokenAcct {
            owner: "Alice".to_string(),
            amount: 100,
        },
        TokenAcct {
            owner: "Bob".to_string(),
            amount: 200,
        },
        TokenAcct {
            owner: "Alice".to_string(),
            amount: 150,
        },
        TokenAcct {
            owner: "Charlie".to_string(),
            amount: 300,
        },
    ];

    // Find all accounts for a specific owner
    // let alice_accounts: Vec<&TokenAcct> = token_accounts
    //     .iter()
    //     .filter(|acct| acct.owner == "Alice")
    //     .collect();

    // println!("Alice's accounts: {} accounts found", alice_accounts.len());

    // Calculate total balance for each owner
    let mut owner_balances: std::collections::HashMap<String, u64> =
        std::collections::HashMap::new();

    for acct in &token_accounts {
        *owner_balances.entry(acct.owner.clone()).or_insert(0) += acct.amount;
    }

    println!("Owner balances: {:?}", owner_balances);

    // Find account with highest balance
    if let Some(max_acct) = token_accounts.iter().max_by_key(|acct| acct.amount) {
        println!(
            "Account with highest balance: {} with {} tokens",
            max_acct.owner, max_acct.amount
        );
    }
}

// ========================================================================
// 10. ERROR HANDLING
// ========================================================================

// Custom error type
#[derive(Debug)]
enum TokenError {
    InsufficientBalance,
    AccountNotFound,
    UnauthorizedSigner,
    InvalidAmount,
}

// Result type alias for convenience
type TokenResult<T> = Result<T, TokenError>;

// Function demonstrating basic error handling
fn error_handling_basics() {
    // ---- OPTION TYPE ----
    // Option<T> represents an optional value:
    // - Some(value) - contains a value
    // - None - no value

    // Example: Find an account
    let account_lookup = find_account("missing");

    // Handling Option with match
    match account_lookup {
        Some(account) => println!("Found account: {}", account),
        None => println!("Account not found"),
    }

    // Using if let for concise Option handling
    if let Some(account) = find_account("alice") {
        println!("Found account: {}", account);
    }

    // ---- RESULT TYPE ----
    // Result<T, E> represents success (Ok) or failure (Err)
    // - Ok(value) - successful result with value
    // - Err(error) - error of type E

    // Example: Transfer tokens
    let transfer_result = transfer_tokens("alice", "bob", 100);

    // Handling Result with match
    match transfer_result {
        Ok(tx_id) => println!("Transfer successful. Tx ID: {}", tx_id),
        Err(err) => println!("Transfer failed: {:?}", err),
    }

    // Using if let for concise Result handling
    if let Err(err) = transfer_tokens("alice", "bob", 999999) {
        println!("Expected error: {:?}", err);
    }

    // ---- PROPAGATING ERRORS ----

    // Example: Process a transaction (propagates errors from sub-functions)
    match process_transaction_with_result("alice", "bob", 50) {
        Ok(result) => println!("Transaction processed: {}", result),
        Err(err) => println!("Transaction processing failed: {:?}", err),
    }

    // ---- QUESTION MARK OPERATOR ----

    // The ? operator is shorthand for propagating errors
    match process_transaction_with_question_mark("alice", "bob", 50) {
        Ok(result) => println!("Transaction processed with ?: {}", result),
        Err(err) => println!("Transaction processing failed with ?: {:?}", err),
    }

    // ---- UNWRAP AND EXPECT ----

    // unwrap() - get value or panic if None/Err
    // Only use when you're sure it won't fail or in examples/tests
    let account = find_account("alice").unwrap(); // Panics if None
    println!("Unwrapped account: {}", account);

    // expect() - like unwrap but with custom error message
    let tx_id =
        transfer_tokens("alice", "bob", 25).expect("Transfer should succeed with valid parameters");
    println!("Expected successful tx: {}", tx_id);

    // ---- HANDLING MULTIPLE ERROR TYPES ----

    // When functions can fail in different ways, use Result with enum Error type
    run_complex_operation().unwrap_or_else(|err| {
        println!("Complex operation failed: {:?}", err);
        "default result".to_string()
    });
}

// Helper functions for error handling examples

fn find_account(id: &str) -> Option<String> {
    match id {
        "alice" => Some("Alice's Account".to_string()),
        "bob" => Some("Bob's Account".to_string()),
        _ => None,
    }
}

fn transfer_tokens(from: &str, to: &str, amount: u64) -> TokenResult<String> {
    // Simulate validations
    if from == "missing" {
        return Err(TokenError::AccountNotFound);
    }

    if amount > 1000 {
        return Err(TokenError::InsufficientBalance);
    }

    if amount == 0 {
        return Err(TokenError::InvalidAmount);
    }

    // Successful case
    Ok(format!("tx-{}-{}-{}", from, to, amount))
}

// Function that propagates errors manually
fn process_transaction_with_result(from: &str, to: &str, amount: u64) -> TokenResult<String> {
    // Check if accounts exist
    let from_account = match find_account(from) {
        Some(account) => account,
        None => return Err(TokenError::AccountNotFound),
    };

    let to_account = match find_account(to) {
        Some(account) => account,
        None => return Err(TokenError::AccountNotFound),
    };

    // Perform transfer
    let tx_id = match transfer_tokens(from, to, amount) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    Ok(format!(
        "Processed: {} -> {} ({}): {}",
        from_account, to_account, amount, tx_id
    ))
}

// Same function using ? operator for cleaner error propagation
fn process_transaction_with_question_mark(
    from: &str,
    to: &str,
    amount: u64,
) -> TokenResult<String> {
    // ? automatically returns the error if Result is Err
    let from_account = find_account(from).ok_or(TokenError::AccountNotFound)?;
    let to_account = find_account(to).ok_or(TokenError::AccountNotFound)?;

    // ? works with Result too
    let tx_id = transfer_tokens(from, to, amount)?;

    Ok(format!(
        "Processed: {} -> {} ({}): {}",
        from_account, to_account, amount, tx_id
    ))
}

// More complex error example
#[derive(Debug)]
enum ComplexError {
    Token(TokenError),
    Network(String),
    Serialization,
}

// Converting from TokenError to ComplexError
impl From<TokenError> for ComplexError {
    fn from(err: TokenError) -> Self {
        ComplexError::Token(err)
    }
}

fn run_complex_operation() -> Result<String, ComplexError> {
    // ? operator works with From implementations for error conversion
    let tx_id = transfer_tokens("alice", "bob", 100).map_err(ComplexError::from)?;

    // Simulate a network error
    if tx_id.contains("100") {
        return Err(ComplexError::Network("Timeout".to_string()));
    }

    Ok(format!("Operation complete: {}", tx_id))
}

// ----------------------------------------
// Error Handling in Solana Programs
// ----------------------------------------

// In Solana, error handling is crucial for secure program execution
fn solana_error_handling_examples() {
    println!("In Solana programs, error handling typically follows these patterns:");

    // 1. Define program errors as enums
    #[derive(Debug)]
    enum ProgramError {
        InvalidInstruction,
        NotRentExempt,
        InsufficientFunds,
        AlreadyInitialized,
        // etc.
    }

    // 2. Functions return Result types
    type ProgramResult<T> = Result<T, ProgramError>;

    // Example Solana-style function
    fn process_instruction(instruction: u8, accounts: Vec<&str>, data: &[u8]) -> ProgramResult<()> {
        // Validate instruction
        let instruction_type = match instruction {
            0 => "Initialize",
            1 => "Transfer",
            2 => "Mint",
            _ => return Err(ProgramError::InvalidInstruction),
        };

        println!("Processing {} instruction", instruction_type);

        // In real Solana programs, ? is used extensively for error propagation

        // Example checking account validation
        if accounts.is_empty() {
            return Err(ProgramError::InvalidInstruction);
        }

        // All successful
        Ok(())
    }

    // Example handling
    match process_instruction(1, vec!["Account1", "Account2"], &[0, 0, 0, 5]) {
        Ok(()) => println!("Instruction processed successfully"),
        Err(err) => println!("Error: {:?}", err),
    }
}

// ========================================================================
// 11. HASHMAPS
// ========================================================================

use std::collections::HashMap;

fn hashmap_examples() {
    // HashMap<K, V> provides key-value storage with O(1) average lookup

    // ---- BASIC HASHMAP OPERATIONS ----

    // Create a new empty HashMap
    let mut token_balances: HashMap<String, u64> = HashMap::new();

    // Insert key-value pairs
    token_balances.insert("Alice".to_string(), 100);
    token_balances.insert("Bob".to_string(), 200);
    token_balances.insert("Charlie".to_string(), 300);

    // Get value (returns Option<&V>)
    if let Some(balance) = token_balances.get("Alice") {
        println!("Alice's balance: {}", balance);
    }

    // Check if key exists
    if token_balances.contains_key("Dave") {
        println!("Dave found");
    } else {
        println!("Dave not found");
    }

    // Remove a key-value pair
    if let Some(removed_balance) = token_balances.remove("Bob") {
        println!("Removed Bob's balance: {}", removed_balance);
    }

    // Get number of entries
    println!("Number of accounts: {}", token_balances.len());

    // Iterate over key-value pairs (order is not guaranteed)
    for (name, balance) in &token_balances {
        println!("{} has {} tokens", name, balance);
    }

    // ---- ADVANCED HASHMAP OPERATIONS ----

    // Insert if key doesn't exist (entry API)
    token_balances.entry("Dave".to_string()).or_insert(0);

    // Update value based on current value
    *token_balances.entry("Dave".to_string()).or_insert(0) += 50;
    println!("Dave's balance: {}", token_balances["Dave"]);

    // Get or insert with a default
    let alice_balance = token_balances.entry("Alice".to_string()).or_insert(0);
    println!("Alice's balance: {}", alice_balance);

    // Create HashMap from vectors using zip and collect
    let keys = vec!["Token1", "Token2", "Token3"];
    let values = vec![5, 10, 15];

    let token_map: HashMap<&str, i32> = keys
        .iter()
        .zip(values.iter())
        .map(|(&k, &v)| (k, v))
        .collect();

    println!("Token map: {:?}", token_map);

    // ---- HASHMAPS IN SOLANA CONTEXT ----

    // Example: Track account state changes in a transaction
    let mut account_updates: HashMap<String, i64> = HashMap::new();

    // Simulate updates
    let ops = [
        ("Account1", 100),
        ("Account2", -50),
        ("Account1", 25),
        ("Account3", 75),
    ];

    // Process operations
    for (account, change) in ops {
        *account_updates.entry(account.to_string()).or_insert(0) += change;
    }

    println!("Final account states:");
    for (account, change) in &account_updates {
        println!("  {}: {:+}", account, change);
    }

    // Example: Mapping program IDs to program names
    let mut solana_programs: HashMap<&str, &str> = HashMap::new();

    solana_programs.insert(
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        "Token Program",
    );
    solana_programs.insert(
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        "Associated Token Account Program",
    );
    solana_programs.insert("11111111111111111111111111111111", "System Program");

    // Look up a program
    match solana_programs.get("11111111111111111111111111111111") {
        Some(name) => println!("Found program: {}", name),
        None => println!("Program not found"),
    }

    // Example: Count token occurrences
    let transaction_tokens = ["SOL", "USDC", "SOL", "BTC", "ETH", "USDC", "SOL", "USDT"];

    let mut token_counts: HashMap<&str, u32> = HashMap::new();

    for token in &transaction_tokens {
        let count = token_counts.entry(token).or_insert(0);
        *count += 1;
    }

    // Sort by count (highest first)
    let mut counts_vec: Vec<_> = token_counts.iter().collect();
    counts_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("Token frequency:");
    for (token, count) in counts_vec {
        println!("  {}: {}", token, count);
    }

    // Example: HashMap with custom key and error handling
    #[derive(Debug, Eq, PartialEq, Hash)]
    struct AccountKey {
        pubkey: String,
        is_signer: bool,
    }

    let mut account_data: HashMap<AccountKey, Vec<u8>> = HashMap::new();

    // Insert data
    account_data.insert(
        AccountKey {
            pubkey: "8ZgMzTUqx7Uq7LUwHEfUvVi3FKEG5BEWdv25TpNB2mKa".to_string(),
            is_signer: true,
        },
        vec![1, 2, 3, 4],
    );

    // Look up with error handling
    let key = AccountKey {
        pubkey: "8ZgMzTUqx7Uq7LUwHEfUvVi3FKEG5BEWdv25TpNB2mKa".to_string(),
        is_signer: true,
    };

    match account_data.get(&key) {
        Some(data) => println!("Found account data: {:?}", data),
        None => println!("Account not found"),
    }
}

// ========================================================================
// 12. MACROS
// ========================================================================

// Macros are expanded at compile time and can generate code

// ---- DECLARATIVE MACROS ----

// Simple macro to create a Solana account info for test/debug
#[macro_export]
macro_rules! debug_account {
    // Pattern with single expression for key
    ($key:expr) => {
        println!("Account: {}, Owner: System Program, Lamports: 0", $key);
    };

    // Pattern with key and lamports
    ($key:expr, $lamports:expr) => {
        println!(
            "Account: {}, Owner: System Program, Lamports: {}",
            $key, $lamports
        );
    };

    // Pattern with key, owner, and lamports
    ($key:expr, $owner:expr, $lamports:expr) => {
        println!(
            "Account: {}, Owner: {}, Lamports: {}",
            $key, $owner, $lamports
        );
    };
}

// Macro for token operations logging
// #[macro_export]
// macro_rules! log_token_op {
//     (transfer $amount:expr from $from:expr to $to:expr) => {
//         println!(
//             "TOKEN TRANSFER: {} tokens from {} to {}",
//             $amount, $from, $to
//         );
//     };

//     (mint $amount:expr to $to:expr) => {
//         println!("TOKEN MINT: {} tokens to {}", $amount, $to);
//     };

//     (burn $amount:expr from $from:expr) => {
//         println!("TOKEN BURN: {} tokens from {}", $amount, $from);
//     };
// }

// A macro that creates test instruction data
#[macro_export]
macro_rules! instruction_data {
    // Create transfer instruction data
    (transfer, $amount:expr) => {{
        let mut data = vec![0]; // 0 = transfer instruction
        data.extend_from_slice(&($amount as u64).to_le_bytes());
        data
    }};

    // Create mint instruction data
    (mint, $amount:expr, $decimals:expr) => {{
        let mut data = vec![1]; // 1 = mint instruction
        data.extend_from_slice(&($amount as u64).to_le_bytes());
        data.push($decimals);
        data
    }};
}

fn macro_examples() {
    // Using the debug_account macro
    debug_account!("8ZgMzTUqx7Uq7LUwHEfUvVi3FKEG5BEWdv25TpNB2mKa");
    debug_account!("vines1vzrYbzLMRdu58eller5JnZseqoCqQXwpv9GT", 50_000_000);
    debug_account!(
        "4Qkev8aNZcqzmXLY6oKWwR8ziedQrXG47kJuNHSfsjNj",
        "Token Program",
        1_000_000
    );

    // Using the log_token_op macro
    // log_token_op!(transfer 100 from "Alice" to "Bob");
    // log_token_op!(mint 1000 to "Treasury");
    // log_token_op!(burn 50 from "Alice");

    // Using the instruction_data macro
    let transfer_data = instruction_data!(transfer, 100);
    println!("Transfer instruction: {:?}", transfer_data);

    let mint_data = instruction_data!(mint, 1000, 9);
    println!("Mint instruction: {:?}", mint_data);

    // Common built-in macros in Rust

    // vec! - create a vector
    let accounts = vec!["Account1", "Account2", "Account3"];

    // println! and format! - for formatted output
    println!("Processing {} accounts", accounts.len());
    let message = format!("Found account {}", "Alice");

    // assert! and debug_assert! - for assertions
    let amount = 100;
    assert!(amount > 0, "Amount must be positive");

    // assert_eq! - assert equality
    assert_eq!(amount, 100, "Amount should be 100");

    // panic! - stop execution with error
    if false {
        // not actually panicking here
        panic!("Critical error encountered");
    }

    // In Solana programs, the solana_program crate provides macros like:
    // - msg! - for logging (similar to println! but works in Solana programs)
    // - sol_log_compute_units! - for logging compute unit consumption

    // Simulate Solana's msg! macro
    macro_rules! sol_msg {
        ($($arg:tt)*) => {
            println!("Program log: {}", format!($($arg)*));
        };
    }

    sol_msg!("Processing instruction type: {}", 2);
    sol_msg!("Account {} is a signer", "Alice");
}

// ========================================================================
// 13. MAIN ENTRYPOINT
// ========================================================================

fn main() {
    println!("\n==============================");
    println!("RUST FOR SOLANA BLOCKCHAIN DEVELOPMENT CHEAT SHEET");
    println!("==============================\n");

    println!("\n==============================");
    println!("1. BASIC CONCEPTS");
    println!("==============================\n");

    println!("\n--- Printing Examples ---\n");
    printing_examples();

    println!("\n--- Variables and Mutability ---\n");
    variables_and_mutability();

    println!("\n--- Shadowing Examples ---\n");
    shadowing_examples();

    println!("\n==============================");
    println!("2. DATA TYPES AND CONTROL FLOW");
    println!("==============================\n");

    println!("\n--- Basic Data Types ---\n");
    basic_data_types();

    println!("\n--- Control Flow Examples ---\n");
    control_flow_examples();

    println!("\n--- Complex Pattern Matching ---\n");
    complex_pattern_matching();

    println!("\n==============================");
    println!("3. MEMORY MANAGEMENT: STACK VS HEAP");
    println!("==============================\n");

    stack_vs_heap_examples();

    println!("\n==============================");
    println!("4. REFERENCES, BORROWING, AND OWNERSHIP");
    println!("==============================\n");

    println!("\n--- Ownership Basics ---\n");
    ownership_basics();

    println!("\n--- References and Borrowing ---\n");
    references_and_borrowing();

    println!("\n--- Lifetime Parameters ---\n");
    lifetime_examples();

    println!("\n==============================");
    println!("5. STRINGS AND SLICES");
    println!("==============================\n");

    string_and_slice_examples();

    println!("\n==============================");
    println!("6. SLICES (GENERAL CONCEPT)");
    println!("==============================\n");

    slice_examples();

    println!("\n==============================");
    println!("7. GENERICS AND TRAITS");
    println!("==============================\n");

    println!("\n--- Generic Examples ---\n");
    generic_examples();

    println!("\n--- Trait Examples ---\n");
    trait_examples();

    println!("\n--- Trait Objects Example ---\n");
    trait_objects_example();

    println!("\n==============================");
    println!("8. ARRAYS AND VECTORS");
    println!("==============================\n");

    arrays_and_vectors();

    println!("\n==============================");
    println!("9. ITERATORS");
    println!("==============================\n");

    iterator_examples();

    println!("\n==============================");
    println!("10. ERROR HANDLING");
    println!("==============================\n");

    error_handling_basics();

    println!("\n--- Solana Error Handling ---\n");
    solana_error_handling_examples();

    println!("\n==============================");
    println!("11. HASHMAPS");
    println!("==============================\n");

    hashmap_examples();

    println!("\n==============================");
    println!("12. MACROS");
    println!("==============================\n");

    macro_examples();

    println!("\n==============================");
    println!("CONGRATULATIONS!");
    println!("You've completed the Rust for Solana Blockchain Development Cheat Sheet!");
    println!("==============================\n");
}
