use std::{error::Error as StdError, num::ParseIntError, string::FromUtf8Error};

use redis::RedisError;
use tokio::task::JoinError;

pub trait ToBoxedError: StdError + Send + 'static {}

impl ToBoxedError for tokio::io::Error {}
impl ToBoxedError for FromUtf8Error {}
impl ToBoxedError for JoinError {}
impl ToBoxedError for RedisError {}
impl ToBoxedError for ParseIntError {}
