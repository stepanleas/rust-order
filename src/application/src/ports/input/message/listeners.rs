use domain::Customer;

pub trait CustomerMessageListener {
    fn customer_created(&self, customer: Customer) -> anyhow::Result<()>;
    fn customer_updated(&self, customer: Customer) -> anyhow::Result<()>;
}

pub trait ProductMessageListener {
    fn product_created(&self, product: domain::Product) -> anyhow::Result<()>;
    fn product_updated(&self, product: domain::Product) -> anyhow::Result<()>;
}
