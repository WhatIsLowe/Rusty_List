# rusty_list

`rusty_list` is a flexible Rust library that allows you to create and manipulate a custom list capable of storing values of different types. The library supports storing integers, strings, floating-point numbers, and can be easily extended to support additional types.

## Features

- Store multiple types in a single list.
- Insert elements at the end or beginning of the list.
- Replace elements at any position in the list.
- Retrieve elements by index with type safety.
- Iterate over the list items.
- Clear the list and retrieve its length.

## Usage

Add `rusty_list` to your `Cargo.toml`:

```toml
[dependencies]
rusty_list = { git = "https://github.com/WhatIsLowe/Rusty_List", branch = "main" }
```

## Example

```rust
use rusty_list::List;

fn main() {
    let mut my_list = List::new();

    // Insert different types into the list
    my_list.insert(42);                  // Insert integer
    my_list.insert("Hello".to_string()); // Insert string
    my_list.insert(3.14);                // Insert float

    // Insert an element at the beginning
    my_list.insert_at_beginning(100);

    // Replace an element at a specific index
    my_list.replace(2, 1337).unwrap();

    // Iterate and print all elements in the list
    for el in my_list.iter() {
        println!("{}", el);
    }

    // Get the number of elements in the list
    println!("List length: {}", my_list.len());

    // Clear the list
    my_list.clear();
    println!("List length after clear: {}", my_list.len());
}
```

## Methods

1. **`new() -> Self`:**
   - Creates a new, empty `List`.
   - **Example:**
     ```rust
     let list = List::new();
     ```

2. **`insert<T: Into<ListItem>>(&mut self, value: T)`:**
   - Inserts a value at the end of the list.
   - **Example:**
     ```rust
     let mut list = List::new();
     list.insert(42);
     ```

3. **`insert_at_beginning<T: Into<ListItem>>(&mut self, value: T)`:**
   - Inserts a value at the beginning of the list.
   - **Example:**
     ```rust
     let mut list = List::new();
     list.insert_at_beginning(42);
     ```

4. **`replace<T: Into<ListItem>>(&mut self, index: usize, value: T) -> Result<(), &'static str>`:**
   - Replaces the item at the specified index with a new value. Returns `Err` if the index is out of bounds.
   - **Example:**
     ```rust
     let mut list = List::new();
     list.insert(42);
     list.replace(0, 43).unwrap();
     assert_eq!(list.get::<i32>(0), Some(&43));
     ```

5. **`get<T: 'static>(&self, index: usize) -> Option<&T>`:**
   - Retrieves a reference to the item at the specified index if the type matches.
   - **Example:**
     ```rust
     let mut list = List::new();
     list.insert(42);
     assert_eq!(list.get::<i32>(0), Some(&42));
     ```

6. **`get_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T>`:**
   - Retrieves a mutable reference to the item at the specified index if the type matches.
   - **Example:**
     ```rust
     let mut list = List::new();
     list.insert(42);
     if let Some(value) = list.get_mut::<i32>(0) {
         *value += 1;
     }
     assert_eq!(list.get::<i32>(0), Some(&43));
     ```

7. **`iter(&self) -> ListIter<'_>`:**
   - Returns an iterator over the items in the list.
   - **Example:**
     ```rust
     let mut list = List::new();
     list.insert(42);
     for item in list.iter() {
         println!("{}", item);
     }
     ```

8. **`len(&self) -> usize`:**
   - Returns the number of items in the list.
   - **Example:**
     ```rust
     let mut list = List::new();
     list.insert(42);
     assert_eq!(list.len(), 1);
     ```

9. **`clear(&mut self)`:**
   - Clears the list, removing all items.
   - **Example:**
     ```rust
     let mut list = List::new();
     list.insert(42);
     list.clear();
     assert_eq!(list.len(), 0);
     ```

## Potential Use Cases

- **Dynamic Data Storage:** Store various types of data in a single structure without needing multiple vectors or lists.
- **Flexible Data Structures:** Useful for scenarios where the types of data are not known at compile time or can change during runtime.
- **Custom Iterators:** Iterate over heterogeneous data structures with ease.
