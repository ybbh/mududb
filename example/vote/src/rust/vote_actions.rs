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

    const TABLE_VOTE_ACTIONS: &str = "vote_actions";
    const COLUMN_ACTION_ID: &str = "action_id";
    const COLUMN_USER_ID: &str = "user_id";
    const COLUMN_VOTE_ID: &str = "vote_id";
    const COLUMN_ACTION_TIME: &str = "action_time";
    const COLUMN_IS_WITHDRAWN: &str = "is_withdrawn";

    #[derive(Debug, Clone)]
    pub struct VoteActions {
        action_id: Option<String>,
        user_id: Option<String>,
        vote_id: Option<String>,
        action_time: Option<i32>,
        is_withdrawn: Option<i32>,
    }

    impl VoteActions {
        pub fn new(
            action_id: Option<String>,
            user_id: Option<String>,
            vote_id: Option<String>,
            action_time: Option<i32>,
            is_withdrawn: Option<i32>,
        ) -> Self {
            let s = Self {
                action_id,
                user_id,
                vote_id,
                action_time,
                is_withdrawn,
            };
            s
        }

        pub fn set_action_id(&mut self, action_id: String) {
            self.action_id = Some(action_id);
        }

        pub fn get_action_id(&self) -> &Option<String> {
            &self.action_id
        }

        pub fn set_user_id(&mut self, user_id: String) {
            self.user_id = Some(user_id);
        }

        pub fn get_user_id(&self) -> &Option<String> {
            &self.user_id
        }

        pub fn set_vote_id(&mut self, vote_id: String) {
            self.vote_id = Some(vote_id);
        }

        pub fn get_vote_id(&self) -> &Option<String> {
            &self.vote_id
        }

        pub fn set_action_time(&mut self, action_time: i32) {
            self.action_time = Some(action_time);
        }

        pub fn get_action_time(&self) -> &Option<i32> {
            &self.action_time
        }

        pub fn set_is_withdrawn(&mut self, is_withdrawn: i32) {
            self.is_withdrawn = Some(is_withdrawn);
        }

        pub fn get_is_withdrawn(&self) -> &Option<i32> {
            &self.is_withdrawn
        }
    }

    impl Datum for VoteActions {
        fn dat_type() -> &'static DatType {
            lazy_static! {
                static ref DAT_TYPE: DatType = entity_utils::entity_dat_type::<VoteActions>();
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

    impl DatumDyn for VoteActions {
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

    impl Entity for VoteActions {
        fn new_empty() -> Self {
            let s = Self {
                action_id: None,
                user_id: None,
                vote_id: None,
                action_time: None,
                is_withdrawn: None,
            };
            s
        }
        fn tuple_desc() -> &'static TupleFieldDesc {
            lazy_static! {
                static ref TUPLE_DESC: TupleFieldDesc = TupleFieldDesc::new(vec![
                    AttrActionId::datum_desc().clone(),
                    AttrUserId::datum_desc().clone(),
                    AttrVoteId::datum_desc().clone(),
                    AttrActionTime::datum_desc().clone(),
                    AttrIsWithdrawn::datum_desc().clone(),
                ]);
            }
            &TUPLE_DESC
        }

        fn object_name() -> &'static str {
            TABLE_VOTE_ACTIONS
        }

        fn get_field_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_ACTION_ID => attr_field_access::attr_get_binary::<_>(&self.action_id),
                COLUMN_USER_ID => attr_field_access::attr_get_binary::<_>(&self.user_id),
                COLUMN_VOTE_ID => attr_field_access::attr_get_binary::<_>(&self.vote_id),
                COLUMN_ACTION_TIME => attr_field_access::attr_get_binary::<_>(&self.action_time),
                COLUMN_IS_WITHDRAWN => attr_field_access::attr_get_binary::<_>(&self.is_withdrawn),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_binary<B: AsRef<[u8]>>(&mut self, column: &str, binary: B) -> RS<()> {
            match column {
                COLUMN_ACTION_ID => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.action_id,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_USER_ID => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.user_id, binary.as_ref())?;
                }
                COLUMN_VOTE_ID => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.vote_id, binary.as_ref())?;
                }
                COLUMN_ACTION_TIME => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.action_time,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_IS_WITHDRAWN => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.is_withdrawn,
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
                COLUMN_ACTION_ID => attr_field_access::attr_get_value::<_>(&self.action_id),
                COLUMN_USER_ID => attr_field_access::attr_get_value::<_>(&self.user_id),
                COLUMN_VOTE_ID => attr_field_access::attr_get_value::<_>(&self.vote_id),
                COLUMN_ACTION_TIME => attr_field_access::attr_get_value::<_>(&self.action_time),
                COLUMN_IS_WITHDRAWN => attr_field_access::attr_get_value::<_>(&self.is_withdrawn),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_value<B: AsRef<DatValue>>(&mut self, column: &str, value: B) -> RS<()> {
            match column {
                COLUMN_ACTION_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.action_id, value)?;
                }
                COLUMN_USER_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.user_id, value)?;
                }
                COLUMN_VOTE_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.vote_id, value)?;
                }
                COLUMN_ACTION_TIME => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.action_time, value)?;
                }
                COLUMN_IS_WITHDRAWN => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.is_withdrawn, value)?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
        }
    }

    pub struct AttrActionId {}

    impl AttrValue<String> for AttrActionId {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTE_ACTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_ACTION_ID
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
            TABLE_VOTE_ACTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_USER_ID
        }
    }

    pub struct AttrVoteId {}

    impl AttrValue<String> for AttrVoteId {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTE_ACTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_VOTE_ID
        }
    }

    pub struct AttrActionTime {}

    impl AttrValue<i32> for AttrActionTime {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTE_ACTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_ACTION_TIME
        }
    }

    pub struct AttrIsWithdrawn {}

    impl AttrValue<i32> for AttrIsWithdrawn {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTE_ACTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_IS_WITHDRAWN
        }
    }
} // end mod object
