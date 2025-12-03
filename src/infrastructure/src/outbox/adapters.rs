use application::outbox::OrderPaymentOutboxMessage;
use application::ports::output::repositories::PaymentOutboxRepository;

pub struct PaymentOutboxRepositoryAdapter {}

impl PaymentOutboxRepository for PaymentOutboxRepositoryAdapter {
    fn save(
        &self,
        _message: OrderPaymentOutboxMessage,
    ) -> anyhow::Result<OrderPaymentOutboxMessage> {
        todo!()
    }
}
