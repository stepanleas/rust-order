#[cfg(test)]
mod tests {
    use crate::entities::order_item::OrderItem;
    use shared::domain::value_objects::{Money, OrderId, OrderItemId, ProductId};

    #[test]
    fn test_create_order_item() -> anyhow::Result<()> {
        let order_item_id = OrderItemId::new();
        let order_id = OrderId::new();
        let product_id = ProductId::new();

        let order_item = OrderItem::new(
            order_item_id,
            order_id,
            product_id,
            2,
            Money::from_f64(30.0)?,
            Money::from_f64(60.0)?,
        );

        assert_eq!(order_item_id, order_item.id());
        assert_eq!(order_id, order_item.order_id());
        assert_eq!(product_id, order_item.product_id());
        assert_eq!(2, order_item.quantity());
        assert_eq!(&Money::from_f64(30.0)?, order_item.price());
        assert_eq!(&Money::from_f64(60.0)?, order_item.sub_total());
        assert!(order_item.is_price_valid());

        Ok(())
    }

    #[test]
    fn test_create_order_item_with_not_valid_price() -> anyhow::Result<()> {
        let order_item_id = OrderItemId::new();
        let order_id = OrderId::new();
        let product_id = ProductId::new();

        let order_item = OrderItem::new(
            order_item_id,
            order_id,
            product_id,
            2,
            Money::from_f64(30.0)?,
            Money::from_f64(55.0)?,
        );

        assert_eq!(order_item_id, order_item.id());
        assert_eq!(order_id, order_item.order_id());
        assert_eq!(product_id, order_item.product_id());
        assert_eq!(2, order_item.quantity());
        assert_eq!(&Money::from_f64(30.0)?, order_item.price());
        assert_eq!(&Money::from_f64(55.0)?, order_item.sub_total());
        assert!(!order_item.is_price_valid());

        Ok(())
    }
}
