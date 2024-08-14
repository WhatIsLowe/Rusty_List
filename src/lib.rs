use std::any::Any;

/// A custom list that can store values of different types.
pub struct List {
    items: Vec<Box<dyn Any>>,
}

impl List {
    /// Creates a new, empty `List`.
    ///
    /// # Examples
    ///
    /// ```
    /// let list = rusty_list::List::new();
    /// ```
    pub fn new() -> Self {
        List { items: Vec::new() }
    }

    /// Inserts a value at the end of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = rusty_list::List::new();
    /// list.insert(42);
    /// ```
    pub fn insert<T: 'static>(&mut self, value: T) {
        self.items.push(Box::new(value));
    }

    /// Inserts a value at the beginning of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = rusty_list::List::new();
    /// list.insert_at_beginning(42);
    /// ```
    pub fn insert_at_beginning<T: 'static>(&mut self, value: T) {
        self.items.insert(0, Box::new(value));
    }

    /// Retrieves a reference to the item at the specified index.
    ///
    /// Returns `None` if the index is out of bounds or the type doesn't match.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = rusty_list::List::new();
    /// list.insert(42);
    /// assert_eq!(list.get::<i32>(0), Some(&42));
    /// ```
    pub fn get<T: 'static>(&self, index: usize) -> Option<&T> {
        self.items.get(index)?.downcast_ref::<T>()
    }

    /// Retrieves a mutable reference to the item at the specified index.
    ///
    /// Returns `None` if the index is out of bounds or the type doesn't match.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = rusty_list::List::new();
    /// list.insert(42);
    /// if let Some(value) = list.get_mut::<i32>(0) {
    ///     *value += 1;
    /// }
    /// assert_eq!(list.get::<i32>(0), Some(&43));
    /// ```
    pub fn get_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)?.downcast_mut::<T>()
    }

    /// Replaces the item at the specified index with a new value.
    ///
    /// Returns `Err` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = rusty_list::List::new();
    /// list.insert(42);
    /// list.replace(0, 43).unwrap();
    /// assert_eq!(list.get::<i32>(0), Some(&43));
    /// ```
    pub fn replace<T: 'static>(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index < self.items.len() {
            self.items[index] = Box::new(value);
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
}

/// An iterator over the items of a `List`.
pub struct ListIter<'a> {
    iter: std::slice::Iter<'a, Box<dyn Any>>,
}

impl<'a> Iterator for ListIter<'a> {
    type Item = &'a dyn Any;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.as_ref())
    }
}

impl List {
    /// Returns an iterator over the items in the list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = rusty_list::List::new();
    /// list.insert(42);
    /// for item in list.iter() {
    ///     if let Some(value) = item.downcast_ref::<i32>() {
    ///         println!("{}", value);
    ///     }
    /// }
    /// ```
    pub fn iter(&self) -> ListIter<'_> {
        ListIter {
            iter: self.items.iter(),
        }
    }
}
