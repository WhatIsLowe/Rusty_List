# rusty_list

`rusty_list` is a custom Rust library that provides a generic list capable of storing elements of different types. It supports basic operations such as inserting elements, retrieving and modifying elements, and iterating over the list.

## Features

- Store elements of any type in a single list
- Insert elements at the end or at the beginning of the list
- Retrieve and modify elements by index
- Iterate over elements in the list

## Installation

To use `rusty_list` in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rusty_list = { git = "https://github.com/WhatIsLowe/Rusty_List", branch="main" }
```

## Usage
Here’s a quick guide on how to use rusty_list:

#### Create a New List

```rust
use rusty_list::List;

let mut list = List::new();
```

#### Insert Elements

```rust
list.insert(42);             // Insert an integer
list.insert("Leet".to_string()); // Insert a string

list.insert_at_beginning(99); // Insert an integer at the beginning
```

#### Retrieve Elements

```rust
if let Some(value) = list.get::<i32>(0) {
    println!("The value is: {}", value);
} else {
    println!("No value found at index 0 or type mismatch");
}
```

#### Modify Elements

```rust
if let Some(value) = list.get_mut::<i32>(0) {
    *value += 1; // Increment the value at index 0
} else {
    println!("No value found at index 0 or type mismatch");
}
```

#### Replace Elements

```rust
list.replace(1, 1337).unwrap();
```

#### Iterate Over Elements

```rust
for el in list.iter() {
    println!("{}", el);
}
```

## Methods

1. **`new()`**: 
   - Creates a new, empty `List`.

2. **`insert<T: 'static>(&mut self, value: T)`**: 
   - Inserts a value at the end of the list.
   - `T` must implement the `'static` lifetime, which means it must be valid for the entire duration of the program.

3. **`insert_at_beginning<T: 'static>(&mut self, value: T)`**: 
   - Inserts a value at the beginning of the list.
   - `T` must implement the `'static` lifetime.

4. **`get<T: 'static>(&self, index: usize) -> Option<&T>`**: 
   - Retrieves a reference to the item at the specified index, if the type matches.
   - Returns `Some(&T)` if the index is valid and the type matches, otherwise `None`.

5. **`get_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T>`**: 
   - Retrieves a mutable reference to the item at the specified index, if the type matches.
   - Returns `Some(&mut T)` if the index is valid and the type matches, otherwise `None`.

6. **`replace<T: 'static>(&mut self, index: usize, value: T) -> Result<(), &'static str>`**: 
   - Replaces the item at the specified index with a new value.
   - Returns `Ok(())` if the index is valid and the replacement was successful, otherwise `Err("Index must be in range!")`.

7. **`iter(&self) -> ListIter<'_>`**: 
   - Returns an iterator over the items in the list.
   - This allows you to iterate over the list using a `for` loop and access each item in the list.

