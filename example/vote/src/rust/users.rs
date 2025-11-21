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

    const TABLE_USERS: &str = "users";
    const COLUMN_USER_ID: &str = "user_id";
    const COLUMN_PHONE: &str = "phone";

    #[derive(Debug, Clone)]
    pub struct Users {
        user_id: Option<String>,
        phone: Option<String>,
    }

    impl Users {
        pub fn new(user_id: Option<String>, phone: Option<String>) -> Self {
            let s = Self { user_id, phone };
            s
        }

        pub fn set_user_id(&mut self, user_id: String) {
            self.user_id = Some(user_id);
        }

        pub fn get_user_id(&self) -> &Option<String> {
            &self.user_id
        }

        pub fn set_phone(&mut self, phone: String) {
            self.phone = Some(phone);
        }

        pub fn get_phone(&self) -> &Option<String> {
            &self.phone
        }
    }

    impl Datum for Users {
        fn dat_type() -> &'static DatType {
            lazy_static! {
                static ref DAT_TYPE: DatType = entity_utils::entity_dat_type::<Users>();
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

    impl DatumDyn for Users {
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

    impl Entity for Users {
        fn new_empty() -> Self {
            let s = Self {
                user_id: None,
                phone: None,
            };
            s
        }
        fn tuple_desc() -> &'static TupleFieldDesc {
            lazy_static! {
                static ref TUPLE_DESC: TupleFieldDesc = TupleFieldDesc::new(vec![
                    AttrUserId::datum_desc().clone(),
                    AttrPhone::datum_desc().clone(),
                ]);
            }
            &TUPLE_DESC
        }

        fn object_name() -> &'static str {
            TABLE_USERS
        }

        fn get_field_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_USER_ID => attr_field_access::attr_get_binary::<_>(&self.user_id),
                COLUMN_PHONE => attr_field_access::attr_get_binary::<_>(&self.phone),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_binary<B: AsRef<[u8]>>(&mut self, column: &str, binary: B) -> RS<()> {
            match column {
                COLUMN_USER_ID => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.user_id, binary.as_ref())?;
                }
                COLUMN_PHONE => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.phone, binary.as_ref())?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
        }
        fn get_field_value(&self, column: &str) -> RS<Option<DatValue>> {
            match column {
                COLUMN_USER_ID => attr_field_access::attr_get_value::<_>(&self.user_id),
                COLUMN_PHONE => attr_field_access::attr_get_value::<_>(&self.phone),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_value<B: AsRef<DatValue>>(&mut self, column: &str, value: B) -> RS<()> {
            match column {
                COLUMN_USER_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.user_id, value)?;
                }
                COLUMN_PHONE => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.phone, value)?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
        }
    }

    pub struct AttrUserId {}

    impl AttrValue<String> for AttrUserId {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_USERS
        }

        fn attr_name() -> &'static str {
            COLUMN_USER_ID
        }
    }

    pub struct AttrPhone {}

    impl AttrValue<String> for AttrPhone {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_USERS
        }

        fn attr_name() -> &'static str {
            COLUMN_PHONE
        }
    }
} // end mod object
