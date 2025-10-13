// @generated automatically by Diesel CLI.

diesel::table! {
    order_items (id) {
        id -> Uuid,
        order_id -> Uuid,
        product_id -> Uuid,
        quantity -> Int4,
        price -> Numeric,
        sub_total -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    orders (id) {
        id -> Uuid,
        customer_id -> Uuid,
        tracking_id -> Uuid,
        price -> Numeric,
        #[max_length = 10]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(order_items -> orders (order_id));

diesel::allow_tables_to_appear_in_same_query!(order_items, orders,);
