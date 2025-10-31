use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FilterOrderBy<T> {
	column: T,
	is_asc: bool,
}
