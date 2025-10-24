// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Uuid,
        #[max_length = 255]
        user_name -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
diesel::joinable!(orders -> customers (customer_id));

diesel::allow_tables_to_appear_in_same_query!(customers, order_items, orders,);
