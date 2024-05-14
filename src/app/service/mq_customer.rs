use std::sync::Arc;

use crate::{
    app::bootstrap::{
        constants::{MQ_SEND_EMAIL_QUEUE, MQ_SEND_EMAIL_TAG},
        AppState,
    },
    library::{error::AppResult, mailor::Email, mqer::Subscriber},
};

pub struct MqCustomer {}

impl MqCustomer {
    pub async fn serve(state: Arc<AppState>) {
        let customer = MqCustomer {};
        match customer.email_sender(state.clone()).await {
            Ok(()) => {}
            Err(e) => {
                tracing::error!("Email Sender Broken: {}", e)
            }
        };
    }
    pub async fn email_sender(&self, state: Arc<AppState>) -> AppResult<()> {
        tracing::debug!("customer started");
        let mqer = state.get_mq()?;
        let func = |message: String| {
            let result = serde_json::from_str::<Email>(&message)
                .map_err(|e| {
                    tracing::error!("Failed to parse email from message: {}", e)
                })
                .and_then(|email| {
                    let res = email.sync_send_text().map_err(|e| {
                        tracing::error!("Failed to send email: {}", e)
                    });
                    tracing::debug!("received:{:#?}", email);
                    res
                });
            if result.is_err() {
                tracing::error!("Failed to send email")
            }
        };
        let delegate = Subscriber::new(func, mqer.clone());
        Ok(mqer
            .basic_receive(MQ_SEND_EMAIL_QUEUE, MQ_SEND_EMAIL_TAG, delegate)
            .await?)
    }
}
