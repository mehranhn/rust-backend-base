pub trait ExternalSms: Send + Sync + 'static {
	async fn send_sms(phone: &str, message: &str) -> Result<(), anyhow::Error>;
}
