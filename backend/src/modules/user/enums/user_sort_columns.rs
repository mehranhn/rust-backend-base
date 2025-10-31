use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum UserSortColumns {
	Id,
	CreatedAt,
	UpdatedAt,
	Email,
	Phone,
	FisrtName,
	LastName,
}
