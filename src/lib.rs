//! # Order data
//!
//! This module contains a definition of the `Order` type using a typestate
//! pattern. Orders can be in one of four states:
//!
//! * `Pending`: The order has been created but not yet submitted for packaging.
//! * `Packaging`: The order is being packaged in a warehouse.
//! * `InDelivery`: The order is being delivered to the customer.
//! * `Completed`: The order has been processed.
//!
//! By using the typestate pattern, we can ensure that we cannot run any operation
//! that wouldn't make sense given the state of the order. For example, we cannot
//! change the state of an order from `Pending` to `Delivered` because we don't
//! know if the order has been packaged yet.
//!
//! ## Example
//!
//! With the typestate pattern, we restrict what methods are available on a given
//! struct based on its internal state.
//!
//! Here's an example of a normal order flow:
//!
//! ```rust
//! use rust_typestate::{Product, Order, User};
//!
//! // Create a new order
//! let product = Product::new("product-123", "T-Shirt", 15.99);
//! let user = User::new("user-123");
//! let order = Order::new(user, vec![product]);
//!
//! // Submit the order for packaging
//! let order = order.submit();
//!
//! // Send the order
//! let order = order.ship("tracking-123");
//!
//! // Mark the order as completed
//! let order = order.complete();
//! ```
//!
//! However, we cannot change the state of an order from `Pending` to `Completed`:
//!
//! ```compile_fail
//! use rust_typestate::{Product, Order, User};
//!
//! // Create a new order
//! let product = Product::new("product-123", "T-Shirt", 15.99);
//! let user = User::new("user-123");
//! let order = Order::new(user, vec![product]);
//!
//! // Mark the order as completed
//! // This will not compile, as the `complete` method is not implemented for
//! // `Order::Pending`.
//! let order = order.complete();
//! ```

use std::cmp::PartialEq;

#[derive(Clone, Debug)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
}

impl Product {
    pub fn new(id: impl AsRef<str>, name: impl AsRef<str>, price: f64) -> Product {
        Product {
            id: id.as_ref().to_string(),
            name: name.as_ref().to_string(),
            price,
        }
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
}

impl User {
    pub fn new(id: impl AsRef<str>) -> User {
        User {
            id: id.as_ref().to_string(),
        }
    }
}

/// A struct that represents an order
pub struct Order<S: OrderState> {
    pub id: String,
    pub user: User,
    pub products: Vec<Product>,
    /// The current state of the order
    ///
    /// Because `InDelivery` has concrete data, we cannot use a zero-sized type
    /// here. If we didn't store any data, we could use `std::marker::PhantomData`
    /// instead.
    pub state: S,
}

/// State of the order
pub trait OrderState {}

/// State of an order that was just created
#[derive(PartialEq, Clone, Debug)]
pub struct Pending;
/// Order being packaged
#[derive(PartialEq, Clone, Debug)]
pub struct Packaging;
/// Order being shipped
#[derive(PartialEq, Clone, Debug)]
pub struct InDelivery {
    pub tracking_id: String,
}
/// Order has been delivered
#[derive(PartialEq, Clone, Debug)]
pub struct Completed;

impl OrderState for Pending {}
impl OrderState for Packaging {}
impl OrderState for InDelivery {}
impl OrderState for Completed {}

impl Order<Pending> {
    /// Create a new order
    pub fn new(user: User, products: Vec<Product>) -> Self {
        Order {
            id: String::new(),
            user,
            products,
            state: Pending,
        }
    }

    /// Submit the order for packaging
    pub fn submit(self) -> Order<Packaging> {
        Order {
            id: self.id,
            user: self.user,
            products: self.products,
            state: Packaging,
        }
    }
}

impl Order<Packaging> {
    /// Ship the order
    pub fn ship(self, tracking_id: impl AsRef<str>) -> Order<InDelivery> {
        Order {
            id: self.id,
            user: self.user,
            products: self.products,
            state: InDelivery {
                tracking_id: tracking_id.as_ref().to_string(),
            },
        }
    }
}

impl Order<InDelivery> {
    /// Mark the order as completed
    pub fn complete(self) -> Order<Completed> {
        Order {
            id: self.id,
            user: self.user,
            products: self.products,
            state: Completed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_flow() {
        let user = User::new("user-1");
        let products = vec![
            Product::new("product-1", "Product 1", 10.0),
            Product::new("product-2", "Product 2", 10.0),
        ];
        let order = Order::new(user, products);
        assert_eq!(order.state, Pending);
        let order = order.submit();
        assert_eq!(order.state, Packaging);
        let order = order.ship("tracking-id");
        assert_eq!(
            order.state,
            InDelivery {
                tracking_id: "tracking-id".to_string()
            }
        );
        let order = order.complete();
        assert_eq!(order.state, Completed);
    }
}
