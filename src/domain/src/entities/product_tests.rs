#[cfg(test)]
mod tests {
    use crate::entities::product::Product;
    use shared::domain::value_objects::{Money, ProductId};

    #[test]
    fn test_create_product() -> anyhow::Result<()> {
        let product_id = ProductId::new();
        let product = Product::new(product_id, "Laptop".to_string(), 5, Money::from_f64(99.99)?);

        assert_eq!(product.id(), product_id);
        assert_eq!(product.title(), "Laptop");
        assert_eq!(product.quantity(), 5);
        assert_eq!(product.price(), &Money::from_f64(99.99)?);

        Ok(())
    }
}
