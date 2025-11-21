pub mod object {

    use lazy_static::lazy_static;
    use mudu::common::result::RS;
    use mudu::data_type::dat_binary::DatBinary;
    use mudu::data_type::dat_textual::DatTextual;
    use mudu::data_type::dat_type::DatType;
    use mudu::data_type::dat_type_id::DatTypeID;
    use mudu::data_type::dat_value::DatValue;
    use mudu::data_type::datum::{Datum, DatumDyn};
    use mudu::database::attr_field_access;
    use mudu::database::attr_value::AttrValue;
    use mudu::database::entity::Entity;
    use mudu::database::entity_utils;
    use mudu::tuple::datum_desc::DatumDesc;
    use mudu::tuple::tuple_field_desc::TupleFieldDesc;

    const TABLE_ORDERS: &str = "orders";
    const COLUMN_ORDER_ID: &str = "order_id";
    const COLUMN_USER_ID: &str = "user_id";
    const COLUMN_MERCH_ID: &str = "merch_id";
    const COLUMN_AMOUNT: &str = "amount";
    const COLUMN_CREATED_AT: &str = "created_at";

    #[derive(Debug, Clone)]
    pub struct Orders {
        order_id: Option<i32>,
        user_id: Option<i32>,
        merch_id: Option<i32>,
        amount: Option<i32>,
        created_at: Option<i32>,
    }

    impl Orders {
        pub fn new(
            order_id: Option<i32>,
            user_id: Option<i32>,
            merch_id: Option<i32>,
            amount: Option<i32>,
            created_at: Option<i32>,
        ) -> Self {
            let s = Self {
                order_id,
                user_id,
                merch_id,
                amount,
                created_at,
            };
            s
        }

        pub fn set_order_id(&mut self, order_id: i32) {
            self.order_id = Some(order_id);
        }

        pub fn get_order_id(&self) -> &Option<i32> {
            &self.order_id
        }

        pub fn set_user_id(&mut self, user_id: i32) {
            self.user_id = Some(user_id);
        }

        pub fn get_user_id(&self) -> &Option<i32> {
            &self.user_id
        }

        pub fn set_merch_id(&mut self, merch_id: i32) {
            self.merch_id = Some(merch_id);
        }

        pub fn get_merch_id(&self) -> &Option<i32> {
            &self.merch_id
        }

        pub fn set_amount(&mut self, amount: i32) {
            self.amount = Some(amount);
        }

        pub fn get_amount(&self) -> &Option<i32> {
            &self.amount
        }

        pub fn set_created_at(&mut self, created_at: i32) {
            self.created_at = Some(created_at);
        }

        pub fn get_created_at(&self) -> &Option<i32> {
            &self.created_at
        }
    }

    impl Datum for Orders {
        fn dat_type() -> &'static DatType {
            lazy_static! {
                static ref DAT_TYPE: DatType = entity_utils::entity_dat_type::<Orders>();
            }
            &DAT_TYPE
        }

        fn from_binary(binary: &[u8]) -> RS<Self> {
            entity_utils::entity_from_binary(binary)
        }

        fn from_value(value: &DatValue) -> RS<Self> {
            entity_utils::entity_from_value(value)
        }

        fn from_textual(textual: &str) -> RS<Self> {
            entity_utils::entity_from_textual(textual)
        }
    }

    impl DatumDyn for Orders {
        fn dat_type_id(&self) -> RS<DatTypeID> {
            entity_utils::entity_dat_type_id()
        }

        fn to_binary(&self, dat_type: &DatType) -> RS<DatBinary> {
            entity_utils::entity_to_binary(self, dat_type)
        }

        fn to_textual(&self, dat_type: &DatType) -> RS<DatTextual> {
            entity_utils::entity_to_textual(self, dat_type)
        }

        fn to_value(&self, dat_type: &DatType) -> RS<DatValue> {
            entity_utils::entity_to_value(self, dat_type)
        }

        fn clone_boxed(&self) -> Box<dyn DatumDyn> {
            entity_utils::entity_clone_boxed(self)
        }
    }

    impl Entity for Orders {
        fn new_empty() -> Self {
            let s = Self {
                order_id: None,
                user_id: None,
                merch_id: None,
                amount: None,
                created_at: None,
            };
            s
        }
        fn tuple_desc() -> &'static TupleFieldDesc {
            lazy_static! {
                static ref TUPLE_DESC: TupleFieldDesc = TupleFieldDesc::new(vec![
                    AttrOrderId::datum_desc().clone(),
                    AttrUserId::datum_desc().clone(),
                    AttrMerchId::datum_desc().clone(),
                    AttrAmount::datum_desc().clone(),
                    AttrCreatedAt::datum_desc().clone(),
                ]);
            }
            &TUPLE_DESC
        }

        fn object_name() -> &'static str {
            TABLE_ORDERS
        }

        fn get_field_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_ORDER_ID => attr_field_access::attr_get_binary::<_>(&self.order_id),
                COLUMN_USER_ID => attr_field_access::attr_get_binary::<_>(&self.user_id),
                COLUMN_MERCH_ID => attr_field_access::attr_get_binary::<_>(&self.merch_id),
                COLUMN_AMOUNT => attr_field_access::attr_get_binary::<_>(&self.amount),
                COLUMN_CREATED_AT => attr_field_access::attr_get_binary::<_>(&self.created_at),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_binary<B: AsRef<[u8]>>(&mut self, column: &str, binary: B) -> RS<()> {
            match column {
                COLUMN_ORDER_ID => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.order_id,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_USER_ID => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.user_id, binary.as_ref())?;
                }
                COLUMN_MERCH_ID => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.merch_id,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_AMOUNT => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.amount, binary.as_ref())?;
                }
                COLUMN_CREATED_AT => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.created_at,
                        binary.as_ref(),
                    )?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
        }
        fn get_field_value(&self, column: &str) -> RS<Option<DatValue>> {
            match column {
                COLUMN_ORDER_ID => attr_field_access::attr_get_value::<_>(&self.order_id),
                COLUMN_USER_ID => attr_field_access::attr_get_value::<_>(&self.user_id),
                COLUMN_MERCH_ID => attr_field_access::attr_get_value::<_>(&self.merch_id),
                COLUMN_AMOUNT => attr_field_access::attr_get_value::<_>(&self.amount),
                COLUMN_CREATED_AT => attr_field_access::attr_get_value::<_>(&self.created_at),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_value<B: AsRef<DatValue>>(&mut self, column: &str, value: B) -> RS<()> {
            match column {
                COLUMN_ORDER_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.order_id, value)?;
                }
                COLUMN_USER_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.user_id, value)?;
                }
                COLUMN_MERCH_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.merch_id, value)?;
                }
                COLUMN_AMOUNT => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.amount, value)?;
                }
                COLUMN_CREATED_AT => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.created_at, value)?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
        }
    }

    pub struct AttrOrderId {}

    impl AttrValue<i32> for AttrOrderId {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_ORDERS
        }

        fn attr_name() -> &'static str {
            COLUMN_ORDER_ID
        }
    }

    pub struct AttrUserId {}

    impl AttrValue<i32> for AttrUserId {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_ORDERS
        }

        fn attr_name() -> &'static str {
            COLUMN_USER_ID
        }
    }

    pub struct AttrMerchId {}

    impl AttrValue<i32> for AttrMerchId {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_ORDERS
        }

        fn attr_name() -> &'static str {
            COLUMN_MERCH_ID
        }
    }

    pub struct AttrAmount {}

    impl AttrValue<i32> for AttrAmount {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_ORDERS
        }

        fn attr_name() -> &'static str {
            COLUMN_AMOUNT
        }
    }

    pub struct AttrCreatedAt {}

    impl AttrValue<i32> for AttrCreatedAt {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_ORDERS
        }

        fn attr_name() -> &'static str {
            COLUMN_CREATED_AT
        }
    }
} // end mod object
