pub mod implementations;

pub trait ExSmsBase: Send + Sync + 'static {
	fn send_sms(phone: &str, message: &str) -> impl Future<Output = Result<(), ()>> + Send;
}

pub trait ExSms: Send + Sync + 'static {}
