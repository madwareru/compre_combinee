use std::fmt::Debug;
use proc_monadde_macro_combinee::*;

define_cmonadde_macro!();

#[macro_export]
macro_rules! c_compre {
    ($ex:expr; $($id:ident <- $monad:expr),+) => {
        c_monadde! {
            $($monad => $id |>)+
            $ex
        }
    }
}

#[macro_export]
macro_rules! c_hx_do {
    ($($id:ident <- $monad:expr),+; $ex:expr) => {
        c_monadde! {
            $($monad => $id |>)+
            $ex
        }
    }
}