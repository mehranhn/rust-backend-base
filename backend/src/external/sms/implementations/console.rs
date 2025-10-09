use crate::external::sms::traits::ExternalSms;

struct ExternalSmsConsole;

impl ExternalSms for ExternalSmsConsole {
	async fn send_sms(phone: &str, message: &str) -> Result<(), anyhow::Error> {
		tracing::info!("Sent '{0}' To {1}", message, phone);
		Ok(())
	}
}
