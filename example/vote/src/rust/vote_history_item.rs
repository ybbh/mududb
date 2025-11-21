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

    const TABLE_VOTE_HISTORY_ITEM: &str = "vote_history_item";
    const COLUMN_VOTE_ID: &str = "vote_id";
    const COLUMN_TOPIC: &str = "topic";
    const COLUMN_ACTION_TIME: &str = "action_time";
    const COLUMN_IS_WITHDRAWN: &str = "is_withdrawn";
    const COLUMN_VOTE_ENDED: &str = "vote_ended";

    #[derive(Debug, Clone)]
    pub struct VoteHistoryItem {
        vote_id: Option<String>,
        topic: Option<String>,
        action_time: Option<i32>,
        is_withdrawn: Option<i32>,
        vote_ended: Option<i32>,
    }

    impl VoteHistoryItem {
        pub fn new(
            vote_id: Option<String>,
            topic: Option<String>,
            action_time: Option<i32>,
            is_withdrawn: Option<i32>,
            vote_ended: Option<i32>,
        ) -> Self {
            let s = Self {
                vote_id,
                topic,
                action_time,
                is_withdrawn,
                vote_ended,
            };
            s
        }

        pub fn set_vote_id(&mut self, vote_id: String) {
            self.vote_id = Some(vote_id);
        }

        pub fn get_vote_id(&self) -> &Option<String> {
            &self.vote_id
        }

        pub fn set_topic(&mut self, topic: String) {
            self.topic = Some(topic);
        }

        pub fn get_topic(&self) -> &Option<String> {
            &self.topic
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

        pub fn set_vote_ended(&mut self, vote_ended: i32) {
            self.vote_ended = Some(vote_ended);
        }

        pub fn get_vote_ended(&self) -> &Option<i32> {
            &self.vote_ended
        }
    }

    impl Datum for VoteHistoryItem {
        fn dat_type() -> &'static DatType {
            lazy_static! {
                static ref DAT_TYPE: DatType = entity_utils::entity_dat_type::<VoteHistoryItem>();
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

    impl DatumDyn for VoteHistoryItem {
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

    impl Entity for VoteHistoryItem {
        fn new_empty() -> Self {
            let s = Self {
                vote_id: None,
                topic: None,
                action_time: None,
                is_withdrawn: None,
                vote_ended: None,
            };
            s
        }
        fn tuple_desc() -> &'static TupleFieldDesc {
            lazy_static! {
                static ref TUPLE_DESC: TupleFieldDesc = TupleFieldDesc::new(vec![
                    AttrVoteId::datum_desc().clone(),
                    AttrTopic::datum_desc().clone(),
                    AttrActionTime::datum_desc().clone(),
                    AttrIsWithdrawn::datum_desc().clone(),
                    AttrVoteEnded::datum_desc().clone(),
                ]);
            }
            &TUPLE_DESC
        }

        fn object_name() -> &'static str {
            TABLE_VOTE_HISTORY_ITEM
        }

        fn get_field_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_VOTE_ID => attr_field_access::attr_get_binary::<_>(&self.vote_id),
                COLUMN_TOPIC => attr_field_access::attr_get_binary::<_>(&self.topic),
                COLUMN_ACTION_TIME => attr_field_access::attr_get_binary::<_>(&self.action_time),
                COLUMN_IS_WITHDRAWN => attr_field_access::attr_get_binary::<_>(&self.is_withdrawn),
                COLUMN_VOTE_ENDED => attr_field_access::attr_get_binary::<_>(&self.vote_ended),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_binary<B: AsRef<[u8]>>(&mut self, column: &str, binary: B) -> RS<()> {
            match column {
                COLUMN_VOTE_ID => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.vote_id, binary.as_ref())?;
                }
                COLUMN_TOPIC => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.topic, binary.as_ref())?;
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
                COLUMN_VOTE_ENDED => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.vote_ended,
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
                COLUMN_VOTE_ID => attr_field_access::attr_get_value::<_>(&self.vote_id),
                COLUMN_TOPIC => attr_field_access::attr_get_value::<_>(&self.topic),
                COLUMN_ACTION_TIME => attr_field_access::attr_get_value::<_>(&self.action_time),
                COLUMN_IS_WITHDRAWN => attr_field_access::attr_get_value::<_>(&self.is_withdrawn),
                COLUMN_VOTE_ENDED => attr_field_access::attr_get_value::<_>(&self.vote_ended),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_value<B: AsRef<DatValue>>(&mut self, column: &str, value: B) -> RS<()> {
            match column {
                COLUMN_VOTE_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.vote_id, value)?;
                }
                COLUMN_TOPIC => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.topic, value)?;
                }
                COLUMN_ACTION_TIME => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.action_time, value)?;
                }
                COLUMN_IS_WITHDRAWN => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.is_withdrawn, value)?;
                }
                COLUMN_VOTE_ENDED => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.vote_ended, value)?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
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
            TABLE_VOTE_HISTORY_ITEM
        }

        fn attr_name() -> &'static str {
            COLUMN_VOTE_ID
        }
    }

    pub struct AttrTopic {}

    impl AttrValue<String> for AttrTopic {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTE_HISTORY_ITEM
        }

        fn attr_name() -> &'static str {
            COLUMN_TOPIC
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
            TABLE_VOTE_HISTORY_ITEM
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
            TABLE_VOTE_HISTORY_ITEM
        }

        fn attr_name() -> &'static str {
            COLUMN_IS_WITHDRAWN
        }
    }

    pub struct AttrVoteEnded {}

    impl AttrValue<i32> for AttrVoteEnded {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTE_HISTORY_ITEM
        }

        fn attr_name() -> &'static str {
            COLUMN_VOTE_ENDED
        }
    }
} // end mod object
