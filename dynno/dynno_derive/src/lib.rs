use proc_macro::TokenStream;
use std::collections::HashMap;
use proc_macro2::Span;
use quote::quote;
use syn::{parse, parse_macro_input, DeriveInput};
use proc_macro_crate::{crate_name, FoundCrate};
use uuid::Uuid;

