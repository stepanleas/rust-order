use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::VarChar;
use diesel::{AsExpression, FromSqlRow, deserialize, serialize};
use domain::OrderStatus as DomainOrderStatus;
use std::io::Write;

#[derive(AsExpression, FromSqlRow, PartialEq, Debug, Clone, Copy)]
#[diesel(sql_type = VarChar)]
pub(crate) enum OrderStatus {
    Pending,
    Paid,
    Approved,
    Cancelling,
    Canceled,
}

impl Into<DomainOrderStatus> for OrderStatus {
    fn into(self) -> DomainOrderStatus {
        match self {
            OrderStatus::Pending => DomainOrderStatus::Pending,
            OrderStatus::Paid => DomainOrderStatus::Paid,
            OrderStatus::Approved => DomainOrderStatus::Approved,
            OrderStatus::Cancelling => DomainOrderStatus::Cancelling,
            OrderStatus::Canceled => DomainOrderStatus::Canceled,
        }
    }
}

impl From<DomainOrderStatus> for OrderStatus {
    fn from(value: DomainOrderStatus) -> Self {
        match value {
            DomainOrderStatus::Pending => OrderStatus::Pending,
            DomainOrderStatus::Paid => OrderStatus::Paid,
            DomainOrderStatus::Approved => OrderStatus::Approved,
            DomainOrderStatus::Cancelling => OrderStatus::Cancelling,
            DomainOrderStatus::Canceled => OrderStatus::Canceled,
        }
    }
}

impl ToSql<VarChar, Pg> for OrderStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let status_str = match self {
            OrderStatus::Pending => "pending",
            OrderStatus::Paid => "paid",
            OrderStatus::Approved => "approved",
            OrderStatus::Cancelling => "cancelling",
            OrderStatus::Canceled => "cancelled",
        };
        out.write_all(status_str.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<VarChar, Pg> for OrderStatus {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"pending" => Ok(OrderStatus::Pending),
            b"paid" => Ok(OrderStatus::Paid),
            b"approved" => Ok(OrderStatus::Approved),
            b"cancelling" => Ok(OrderStatus::Cancelling),
            b"cancelled" => Ok(OrderStatus::Canceled),
            v => Err(format!("Unknown order status: {:?}", String::from_utf8_lossy(v)).into()),
        }
    }
}
