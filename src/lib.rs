//! Library for getting the types of Tuple Elements.
#![no_std]
#![deny(unsafe_code, clippy::unwrap_used)]

extern crate alloc;

use core::any::{TypeId, type_name};
use alloc::vec::Vec;

/// Represents a Type, with it's [TypeId] and Name.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Type{
    pub id: TypeId,
    pub name: &'static str
}

impl Type{
    pub fn of<T: 'static>() -> Self{
        Self{
            id: TypeId::of::<T>(),
            name: type_name::<T>()
        }
    }

    pub fn of_val<T: 'static>(_: &T) -> Self{
        Self{
            id: TypeId::of::<T>(),
            name: type_name::<T>()
        }
    }
}

/// Extension Trait that adds the [TupleExt::types],  [TupleExt::type_ids] and [TupleExt::type_names] functions along with methods with the prefix `self_`.
pub trait TupleExt{
    /// Returns the types that make up the tuple with [TupleExt] is implemented on.
    fn types() -> Vec<crate::Type>;
    /// Returns the [TypeId]s of the type with [TupleExt] is implemented on.
    fn type_ids() -> Vec<core::any::TypeId>;
    // Return the names of the types with [TupleExt] is implemented on as a Vector or `&'static str`s.
    fn type_names() -> Vec<&'static str>;

    /// Method version of [TupleExt::types].
    fn self_types(&self) -> Vec<Type> { Self::types() }
    /// Method version of [TupleExt::type_ids].
    fn self_type_ids(&self) -> Vec<TypeId> { Self::type_ids() }
    /// Method version of [TupleExt::type_names].
    fn self_type_names(&self) -> Vec<&'static str> { Self::type_names() }
}

/// Internal Macro for implementing [TupleExt] on tuples.
#[doc(hidden)]
macro_rules! impl_tuple_ext{
    [$($t:ident), *] => {
        impl<$($t: 'static), *> TupleExt for ($($t),*,) {
            fn types() -> Vec<crate::Type> { alloc::vec![$(crate::Type::of::<$t>()), *] }
            fn type_ids() -> Vec<core::any::TypeId> { alloc::vec![$(core::any::TypeId::of::<$t>()), *] }
            fn type_names() -> Vec<&'static str> { alloc::vec![$(core::any::type_name::<$t>()), *] }
        }
    }
}

impl TupleExt for () {
    fn types() -> Vec<crate::Type> { alloc::vec![] }
    fn type_ids() -> Vec<core::any::TypeId> { alloc::vec![] }
    fn type_names() -> Vec<&'static str> { alloc::vec![] }
}

impl_tuple_ext![T1];
impl_tuple_ext![T1, T2];
impl_tuple_ext![T1, T2, T3];
impl_tuple_ext![T1, T2, T3, T4];
impl_tuple_ext![T1, T2, T3, T4, T5];
impl_tuple_ext![T1, T2, T3, T4, T5, T6];
impl_tuple_ext![T1, T2, T3, T4, T5, T6, T7];

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use alloc::string::ToString as _;

    use super::*;

    #[test]
    fn test_type_of() {
        assert_eq!(Type::of::<i32>().name, "i32");
        assert_eq!(Type::of_val(&"hello".to_string()).name, "alloc::string::String");
    }

    #[test]
    fn test_tuple_ext() {
        let tuple = (1, 2.0, 3, 4, "test", 6, -8);
        assert_eq!(tuple.self_types().len(), 7);
        assert_eq!(tuple.self_type_ids().len(), 7);
        assert_eq!(tuple.self_type_names().len(), 7);
    }

    #[test]
    fn test_tuple_ext_different_types() {
        let tuple = (1, "hello", 3.14, true);
        let types = tuple.self_types();
        let type_names: Vec<&str> = types.iter().map(|t| t.name).collect();
        let type_ids: Vec<TypeId> = types.iter().map(|t| t.id).collect();

        assert_eq!(types.len(), 4);
        assert_eq!(type_names, alloc::vec!["i32", "&str", "f64", "bool"]);
        assert_eq!(type_ids.len(), 4);
    }

    #[test]
    fn test_type_of_struct() {
        #[derive(Debug)]
        struct Person {
            name: alloc::string::String,
            age: u8,
        }

        let person: Person = Person {
            name: "Proton".to_string(),
            age: 14,
        };

        assert!(Type::of_val(&person).name.ends_with("test_type_of_struct::Person"));
    }
}

