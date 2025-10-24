use application::{OrderPaymentOutboxMessage, PaymentOutboxRepository};

pub struct PaymentOutboxRepositoryAdapter {}

impl PaymentOutboxRepository for PaymentOutboxRepositoryAdapter {
    fn save(
        &self,
        message: OrderPaymentOutboxMessage,
    ) -> anyhow::Result<OrderPaymentOutboxMessage> {
        todo!()
    }
}
