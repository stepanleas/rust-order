#[cfg(test)]
mod tests {
    use crate::entities::product::Product;
    use shared::domain::value_objects::{Money, ProductId};

    #[test]
    fn test_create_product() -> anyhow::Result<()> {
        let product_id = ProductId::new();
        let product = Product::new(product_id, "Laptop".to_string(), 5, Money::from_f64(99.99)?);

        assert_eq!(product_id, product.id());
        assert_eq!("Laptop", product.title());
        assert_eq!(5, product.quantity());
        assert_eq!("99.99", product.price().to_string());

        Ok(())
    }
}
