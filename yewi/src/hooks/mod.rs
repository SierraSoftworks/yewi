//! Hooks that keep server and client renders in agreement.

mod id;
mod now;

pub use id::{reset_ids, use_id};
pub use now::{Now, NowProvider, NowProviderProps, Tick, use_now};
