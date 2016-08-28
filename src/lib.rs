#![cfg_attr(feature="flame_it", feature(plugin,custom_attribute))]
#![cfg_attr(feature="flame_it", plugin(flamer))]

#[cfg(feature="flame_it")]
extern crate flame;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate json;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate error_chain;

extern crate threadpool;
extern crate thread_id;

extern crate redis;
extern crate r2d2;
extern crate r2d2_redis;

extern crate rand;
extern crate random_choice;

extern crate libc;

extern crate chrono;

#[macro_use]
extern crate chan;
extern crate chan_signal;

mod server;
mod job_handler;
pub mod errors;
mod job;
mod utils;
mod worker;


pub use server::SidekiqServer;
pub use job_handler::{PrinterHandlerFactory, ErrorHandlerFactory, JobHandlerFactory,
                      PanicHandlerFactory, JobHandler, JobHandlerError};

pub use job::Job;
