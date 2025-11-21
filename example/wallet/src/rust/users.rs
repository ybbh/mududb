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
    const COLUMN_NAME: &str = "name";
    const COLUMN_PHONE: &str = "phone";
    const COLUMN_EMAIL: &str = "email";
    const COLUMN_PASSWORD: &str = "password";
    const COLUMN_CREATED_AT: &str = "created_at";
    const COLUMN_UPDATED_AT: &str = "updated_at";

    #[derive(Debug, Clone)]
    pub struct Users {
        user_id: Option<i32>,
        name: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        password: Option<String>,
        created_at: Option<i32>,
        updated_at: Option<i32>,
    }

    impl Users {
        pub fn new(
            user_id: Option<i32>,
            name: Option<String>,
            phone: Option<String>,
            email: Option<String>,
            password: Option<String>,
            created_at: Option<i32>,
            updated_at: Option<i32>,
        ) -> Self {
            let s = Self {
                user_id,
                name,
                phone,
                email,
                password,
                created_at,
                updated_at,
            };
            s
        }

        pub fn set_user_id(&mut self, user_id: i32) {
            self.user_id = Some(user_id);
        }

        pub fn get_user_id(&self) -> &Option<i32> {
            &self.user_id
        }

        pub fn set_name(&mut self, name: String) {
            self.name = Some(name);
        }

        pub fn get_name(&self) -> &Option<String> {
            &self.name
        }

        pub fn set_phone(&mut self, phone: String) {
            self.phone = Some(phone);
        }

        pub fn get_phone(&self) -> &Option<String> {
            &self.phone
        }

        pub fn set_email(&mut self, email: String) {
            self.email = Some(email);
        }

        pub fn get_email(&self) -> &Option<String> {
            &self.email
        }

        pub fn set_password(&mut self, password: String) {
            self.password = Some(password);
        }

        pub fn get_password(&self) -> &Option<String> {
            &self.password
        }

        pub fn set_created_at(&mut self, created_at: i32) {
            self.created_at = Some(created_at);
        }

        pub fn get_created_at(&self) -> &Option<i32> {
            &self.created_at
        }

        pub fn set_updated_at(&mut self, updated_at: i32) {
            self.updated_at = Some(updated_at);
        }

        pub fn get_updated_at(&self) -> &Option<i32> {
            &self.updated_at
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
                name: None,
                phone: None,
                email: None,
                password: None,
                created_at: None,
                updated_at: None,
            };
            s
        }
        fn tuple_desc() -> &'static TupleFieldDesc {
            lazy_static! {
                static ref TUPLE_DESC: TupleFieldDesc = TupleFieldDesc::new(vec![
                    AttrUserId::datum_desc().clone(),
                    AttrName::datum_desc().clone(),
                    AttrPhone::datum_desc().clone(),
                    AttrEmail::datum_desc().clone(),
                    AttrPassword::datum_desc().clone(),
                    AttrCreatedAt::datum_desc().clone(),
                    AttrUpdatedAt::datum_desc().clone(),
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
                COLUMN_NAME => attr_field_access::attr_get_binary::<_>(&self.name),
                COLUMN_PHONE => attr_field_access::attr_get_binary::<_>(&self.phone),
                COLUMN_EMAIL => attr_field_access::attr_get_binary::<_>(&self.email),
                COLUMN_PASSWORD => attr_field_access::attr_get_binary::<_>(&self.password),
                COLUMN_CREATED_AT => attr_field_access::attr_get_binary::<_>(&self.created_at),
                COLUMN_UPDATED_AT => attr_field_access::attr_get_binary::<_>(&self.updated_at),
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
                COLUMN_NAME => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.name, binary.as_ref())?;
                }
                COLUMN_PHONE => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.phone, binary.as_ref())?;
                }
                COLUMN_EMAIL => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.email, binary.as_ref())?;
                }
                COLUMN_PASSWORD => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.password,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_CREATED_AT => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.created_at,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_UPDATED_AT => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.updated_at,
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
                COLUMN_USER_ID => attr_field_access::attr_get_value::<_>(&self.user_id),
                COLUMN_NAME => attr_field_access::attr_get_value::<_>(&self.name),
                COLUMN_PHONE => attr_field_access::attr_get_value::<_>(&self.phone),
                COLUMN_EMAIL => attr_field_access::attr_get_value::<_>(&self.email),
                COLUMN_PASSWORD => attr_field_access::attr_get_value::<_>(&self.password),
                COLUMN_CREATED_AT => attr_field_access::attr_get_value::<_>(&self.created_at),
                COLUMN_UPDATED_AT => attr_field_access::attr_get_value::<_>(&self.updated_at),
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
                COLUMN_NAME => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.name, value)?;
                }
                COLUMN_PHONE => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.phone, value)?;
                }
                COLUMN_EMAIL => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.email, value)?;
                }
                COLUMN_PASSWORD => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.password, value)?;
                }
                COLUMN_CREATED_AT => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.created_at, value)?;
                }
                COLUMN_UPDATED_AT => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.updated_at, value)?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
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
            TABLE_USERS
        }

        fn attr_name() -> &'static str {
            COLUMN_USER_ID
        }
    }

    pub struct AttrName {}

    impl AttrValue<String> for AttrName {
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
            COLUMN_NAME
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

    pub struct AttrEmail {}

    impl AttrValue<String> for AttrEmail {
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
            COLUMN_EMAIL
        }
    }

    pub struct AttrPassword {}

    impl AttrValue<String> for AttrPassword {
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
            COLUMN_PASSWORD
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
            TABLE_USERS
        }

        fn attr_name() -> &'static str {
            COLUMN_CREATED_AT
        }
    }

    pub struct AttrUpdatedAt {}

    impl AttrValue<i32> for AttrUpdatedAt {
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
            COLUMN_UPDATED_AT
        }
    }
} // end mod object
