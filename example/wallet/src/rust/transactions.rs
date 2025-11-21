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

    const TABLE_TRANSACTIONS: &str = "transactions";
    const COLUMN_TRANS_ID: &str = "trans_id";
    const COLUMN_TRANS_TYPE: &str = "trans_type";
    const COLUMN_FROM_USER: &str = "from_user";
    const COLUMN_TO_USER: &str = "to_user";
    const COLUMN_AMOUNT: &str = "amount";
    const COLUMN_CREATED_AT: &str = "created_at";

    #[derive(Debug, Clone)]
    pub struct Transactions {
        trans_id: Option<String>,
        trans_type: Option<String>,
        from_user: Option<i32>,
        to_user: Option<i32>,
        amount: Option<i32>,
        created_at: Option<i32>,
    }

    impl Transactions {
        pub fn new(
            trans_id: Option<String>,
            trans_type: Option<String>,
            from_user: Option<i32>,
            to_user: Option<i32>,
            amount: Option<i32>,
            created_at: Option<i32>,
        ) -> Self {
            let s = Self {
                trans_id,
                trans_type,
                from_user,
                to_user,
                amount,
                created_at,
            };
            s
        }

        pub fn set_trans_id(&mut self, trans_id: String) {
            self.trans_id = Some(trans_id);
        }

        pub fn get_trans_id(&self) -> &Option<String> {
            &self.trans_id
        }

        pub fn set_trans_type(&mut self, trans_type: String) {
            self.trans_type = Some(trans_type);
        }

        pub fn get_trans_type(&self) -> &Option<String> {
            &self.trans_type
        }

        pub fn set_from_user(&mut self, from_user: i32) {
            self.from_user = Some(from_user);
        }

        pub fn get_from_user(&self) -> &Option<i32> {
            &self.from_user
        }

        pub fn set_to_user(&mut self, to_user: i32) {
            self.to_user = Some(to_user);
        }

        pub fn get_to_user(&self) -> &Option<i32> {
            &self.to_user
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

    impl Datum for Transactions {
        fn dat_type() -> &'static DatType {
            lazy_static! {
                static ref DAT_TYPE: DatType = entity_utils::entity_dat_type::<Transactions>();
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

    impl DatumDyn for Transactions {
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

    impl Entity for Transactions {
        fn new_empty() -> Self {
            let s = Self {
                trans_id: None,
                trans_type: None,
                from_user: None,
                to_user: None,
                amount: None,
                created_at: None,
            };
            s
        }
        fn tuple_desc() -> &'static TupleFieldDesc {
            lazy_static! {
                static ref TUPLE_DESC: TupleFieldDesc = TupleFieldDesc::new(vec![
                    AttrTransId::datum_desc().clone(),
                    AttrTransType::datum_desc().clone(),
                    AttrFromUser::datum_desc().clone(),
                    AttrToUser::datum_desc().clone(),
                    AttrAmount::datum_desc().clone(),
                    AttrCreatedAt::datum_desc().clone(),
                ]);
            }
            &TUPLE_DESC
        }

        fn object_name() -> &'static str {
            TABLE_TRANSACTIONS
        }

        fn get_field_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_TRANS_ID => attr_field_access::attr_get_binary::<_>(&self.trans_id),
                COLUMN_TRANS_TYPE => attr_field_access::attr_get_binary::<_>(&self.trans_type),
                COLUMN_FROM_USER => attr_field_access::attr_get_binary::<_>(&self.from_user),
                COLUMN_TO_USER => attr_field_access::attr_get_binary::<_>(&self.to_user),
                COLUMN_AMOUNT => attr_field_access::attr_get_binary::<_>(&self.amount),
                COLUMN_CREATED_AT => attr_field_access::attr_get_binary::<_>(&self.created_at),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_binary<B: AsRef<[u8]>>(&mut self, column: &str, binary: B) -> RS<()> {
            match column {
                COLUMN_TRANS_ID => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.trans_id,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_TRANS_TYPE => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.trans_type,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_FROM_USER => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.from_user,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_TO_USER => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.to_user, binary.as_ref())?;
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
                COLUMN_TRANS_ID => attr_field_access::attr_get_value::<_>(&self.trans_id),
                COLUMN_TRANS_TYPE => attr_field_access::attr_get_value::<_>(&self.trans_type),
                COLUMN_FROM_USER => attr_field_access::attr_get_value::<_>(&self.from_user),
                COLUMN_TO_USER => attr_field_access::attr_get_value::<_>(&self.to_user),
                COLUMN_AMOUNT => attr_field_access::attr_get_value::<_>(&self.amount),
                COLUMN_CREATED_AT => attr_field_access::attr_get_value::<_>(&self.created_at),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_value<B: AsRef<DatValue>>(&mut self, column: &str, value: B) -> RS<()> {
            match column {
                COLUMN_TRANS_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.trans_id, value)?;
                }
                COLUMN_TRANS_TYPE => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.trans_type, value)?;
                }
                COLUMN_FROM_USER => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.from_user, value)?;
                }
                COLUMN_TO_USER => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.to_user, value)?;
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

    pub struct AttrTransId {}

    impl AttrValue<String> for AttrTransId {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_TRANSACTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_TRANS_ID
        }
    }

    pub struct AttrTransType {}

    impl AttrValue<String> for AttrTransType {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_TRANSACTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_TRANS_TYPE
        }
    }

    pub struct AttrFromUser {}

    impl AttrValue<i32> for AttrFromUser {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_TRANSACTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_FROM_USER
        }
    }

    pub struct AttrToUser {}

    impl AttrValue<i32> for AttrToUser {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_TRANSACTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_TO_USER
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
            TABLE_TRANSACTIONS
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
            TABLE_TRANSACTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_CREATED_AT
        }
    }
} // end mod object
