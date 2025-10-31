use std::future::ready;

use crate::external::sms::{ExSms, ExSmsBase};

pub struct ExSmsConsole;

impl ExSmsBase for ExSmsConsole {
    fn send_sms(phone: &str, message: &str) -> impl Future<Output = Result<(), ()>> + Send {
		tracing::info!("Sent '{0}' To {1}", message, phone);
        ready(Ok(()))
    }
}
