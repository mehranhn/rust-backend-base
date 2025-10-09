pub trait ExternalMemoryPhone {
	async fn phone_otp_get_and_delete_if_exists(
		&self, phone: &str,
	) -> Result<Option<u32>, anyhow::Error>;

	async fn phone_otp_set(&self, phone: &str, code: u32, ttl: u32) -> Result<(), anyhow::Error>;
}
