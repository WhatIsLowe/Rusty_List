use std::fmt;

/// A custom list that can store values of different types.
pub enum ListItem {
    Int(i32),
    Str(String),
    Float(f64),
    // Add other types as needed
}

pub struct List {
    items: Vec<ListItem>,
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
    /// assert_eq!(list.get::<i32>(0), Some(&42));
    /// ```
    pub fn insert<T: Into<ListItem>>(&mut self, value: T) {
        self.items.push(value.into());
    }

    /// Inserts a value at the beginning of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = rusty_list::List::new();
    /// list.insert_at_beginning(42);
    /// ```
    pub fn insert_at_beginning<T: Into<ListItem>>(&mut self, value: T) {
        self.items.insert(0, value.into());
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
    pub fn replace<T: Into<ListItem>>(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index < self.items.len() {
            self.items[index] = value.into();
            Ok(())
        } else {
            Err("Index out of range")
        }
    }

    /// Retrieves a reference to the item at the specified index if the type matches.
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
        match self.items.get(index)? {
            ListItem::Int(value) => value as &dyn std::any::Any,
            ListItem::Str(value) => value as &dyn std::any::Any,
            ListItem::Float(value) => value as &dyn std::any::Any,
        }
        .downcast_ref::<T>()
    }

    /// Retrieves a mutable reference to the item at the specified index if the type matches.
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
        match self.items.get_mut(index)? {
            ListItem::Int(value) => value as &mut dyn std::any::Any,
            ListItem::Str(value) => value as &mut dyn std::any::Any,
            ListItem::Float(value) => value as &mut dyn std::any::Any,
        }
        .downcast_mut::<T>()
    }

    /// Returns an iterator over the items in the list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = rusty_list::List::new();
    /// list.insert(42);
    /// for item in list.iter() {
    ///     println!("{}", item);
    /// }
    /// ```
    pub fn iter(&self) -> ListIter<'_> {
        ListIter {
            list: self,
            index: 0,
        }
    }

    /// Returns the number of items in the list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = rusty_list::List::new();
    /// list.insert(42);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Clears the list, removing all items.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = rusty_list::List::new();
    /// list.insert(42);
    /// list.clear();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

// Implementation of From for different types
impl From<i32> for ListItem {
    fn from(value: i32) -> Self {
        ListItem::Int(value)
    }
}

impl From<String> for ListItem {
    fn from(value: String) -> Self {
        ListItem::Str(value)
    }
}

impl From<f64> for ListItem {
    fn from(value: f64) -> Self {
        ListItem::Float(value)
    }
}

// Implementation of Display for ListItem
impl fmt::Display for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListItem::Int(val) => write!(f, "{val}"),
            ListItem::Str(val) => write!(f, "{val}"),
            ListItem::Float(val) => write!(f, "{val}"),
        }
    }
}

// Iterator for List
pub struct ListIter<'a> {
    list: &'a List,
    index: usize,
}

impl<'a> Iterator for ListIter<'a> {
    type Item = &'a ListItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.list.len() {
            let item = &self.list.items[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}
