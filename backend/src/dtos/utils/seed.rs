pub struct SeedDto<'a> {
	pub super_admin_username: &'a str,
	pub super_admin_hashed_password: &'a [u8; 32],
}
