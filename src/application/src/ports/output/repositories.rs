use crate::outbox::OrderPaymentOutboxMessage;

pub trait PaymentOutboxRepository {
    fn save(&self, message: OrderPaymentOutboxMessage)
    -> anyhow::Result<OrderPaymentOutboxMessage>;
}
