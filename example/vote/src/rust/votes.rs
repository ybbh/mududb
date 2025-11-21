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

    const TABLE_VOTES: &str = "votes";
    const COLUMN_VOTE_ID: &str = "vote_id";
    const COLUMN_CREATOR_ID: &str = "creator_id";
    const COLUMN_TOPIC: &str = "topic";
    const COLUMN_VOTE_TYPE: &str = "vote_type";
    const COLUMN_MAX_CHOICES: &str = "max_choices";
    const COLUMN_END_TIME: &str = "end_time";
    const COLUMN_VISIBILITY_RULE: &str = "visibility_rule";

    #[derive(Debug, Clone)]
    pub struct Votes {
        vote_id: Option<String>,
        creator_id: Option<String>,
        topic: Option<String>,
        vote_type: Option<String>,
        max_choices: Option<i32>,
        end_time: Option<i32>,
        visibility_rule: Option<String>,
    }

    impl Votes {
        pub fn new(
            vote_id: Option<String>,
            creator_id: Option<String>,
            topic: Option<String>,
            vote_type: Option<String>,
            max_choices: Option<i32>,
            end_time: Option<i32>,
            visibility_rule: Option<String>,
        ) -> Self {
            let s = Self {
                vote_id,
                creator_id,
                topic,
                vote_type,
                max_choices,
                end_time,
                visibility_rule,
            };
            s
        }

        pub fn set_vote_id(&mut self, vote_id: String) {
            self.vote_id = Some(vote_id);
        }

        pub fn get_vote_id(&self) -> &Option<String> {
            &self.vote_id
        }

        pub fn set_creator_id(&mut self, creator_id: String) {
            self.creator_id = Some(creator_id);
        }

        pub fn get_creator_id(&self) -> &Option<String> {
            &self.creator_id
        }

        pub fn set_topic(&mut self, topic: String) {
            self.topic = Some(topic);
        }

        pub fn get_topic(&self) -> &Option<String> {
            &self.topic
        }

        pub fn set_vote_type(&mut self, vote_type: String) {
            self.vote_type = Some(vote_type);
        }

        pub fn get_vote_type(&self) -> &Option<String> {
            &self.vote_type
        }

        pub fn set_max_choices(&mut self, max_choices: i32) {
            self.max_choices = Some(max_choices);
        }

        pub fn get_max_choices(&self) -> &Option<i32> {
            &self.max_choices
        }

        pub fn set_end_time(&mut self, end_time: i32) {
            self.end_time = Some(end_time);
        }

        pub fn get_end_time(&self) -> &Option<i32> {
            &self.end_time
        }

        pub fn set_visibility_rule(&mut self, visibility_rule: String) {
            self.visibility_rule = Some(visibility_rule);
        }

        pub fn get_visibility_rule(&self) -> &Option<String> {
            &self.visibility_rule
        }
    }

    impl Datum for Votes {
        fn dat_type() -> &'static DatType {
            lazy_static! {
                static ref DAT_TYPE: DatType = entity_utils::entity_dat_type::<Votes>();
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

    impl DatumDyn for Votes {
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

    impl Entity for Votes {
        fn new_empty() -> Self {
            let s = Self {
                vote_id: None,
                creator_id: None,
                topic: None,
                vote_type: None,
                max_choices: None,
                end_time: None,
                visibility_rule: None,
            };
            s
        }
        fn tuple_desc() -> &'static TupleFieldDesc {
            lazy_static! {
                static ref TUPLE_DESC: TupleFieldDesc = TupleFieldDesc::new(vec![
                    AttrVoteId::datum_desc().clone(),
                    AttrCreatorId::datum_desc().clone(),
                    AttrTopic::datum_desc().clone(),
                    AttrVoteType::datum_desc().clone(),
                    AttrMaxChoices::datum_desc().clone(),
                    AttrEndTime::datum_desc().clone(),
                    AttrVisibilityRule::datum_desc().clone(),
                ]);
            }
            &TUPLE_DESC
        }

        fn object_name() -> &'static str {
            TABLE_VOTES
        }

        fn get_field_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_VOTE_ID => attr_field_access::attr_get_binary::<_>(&self.vote_id),
                COLUMN_CREATOR_ID => attr_field_access::attr_get_binary::<_>(&self.creator_id),
                COLUMN_TOPIC => attr_field_access::attr_get_binary::<_>(&self.topic),
                COLUMN_VOTE_TYPE => attr_field_access::attr_get_binary::<_>(&self.vote_type),
                COLUMN_MAX_CHOICES => attr_field_access::attr_get_binary::<_>(&self.max_choices),
                COLUMN_END_TIME => attr_field_access::attr_get_binary::<_>(&self.end_time),
                COLUMN_VISIBILITY_RULE => {
                    attr_field_access::attr_get_binary::<_>(&self.visibility_rule)
                }
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
                COLUMN_CREATOR_ID => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.creator_id,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_TOPIC => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.topic, binary.as_ref())?;
                }
                COLUMN_VOTE_TYPE => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.vote_type,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_MAX_CHOICES => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.max_choices,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_END_TIME => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.end_time,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_VISIBILITY_RULE => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.visibility_rule,
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
                COLUMN_CREATOR_ID => attr_field_access::attr_get_value::<_>(&self.creator_id),
                COLUMN_TOPIC => attr_field_access::attr_get_value::<_>(&self.topic),
                COLUMN_VOTE_TYPE => attr_field_access::attr_get_value::<_>(&self.vote_type),
                COLUMN_MAX_CHOICES => attr_field_access::attr_get_value::<_>(&self.max_choices),
                COLUMN_END_TIME => attr_field_access::attr_get_value::<_>(&self.end_time),
                COLUMN_VISIBILITY_RULE => {
                    attr_field_access::attr_get_value::<_>(&self.visibility_rule)
                }
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
                COLUMN_CREATOR_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.creator_id, value)?;
                }
                COLUMN_TOPIC => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.topic, value)?;
                }
                COLUMN_VOTE_TYPE => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.vote_type, value)?;
                }
                COLUMN_MAX_CHOICES => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.max_choices, value)?;
                }
                COLUMN_END_TIME => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.end_time, value)?;
                }
                COLUMN_VISIBILITY_RULE => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.visibility_rule, value)?;
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
            TABLE_VOTES
        }

        fn attr_name() -> &'static str {
            COLUMN_VOTE_ID
        }
    }

    pub struct AttrCreatorId {}

    impl AttrValue<String> for AttrCreatorId {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTES
        }

        fn attr_name() -> &'static str {
            COLUMN_CREATOR_ID
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
            TABLE_VOTES
        }

        fn attr_name() -> &'static str {
            COLUMN_TOPIC
        }
    }

    pub struct AttrVoteType {}

    impl AttrValue<String> for AttrVoteType {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTES
        }

        fn attr_name() -> &'static str {
            COLUMN_VOTE_TYPE
        }
    }

    pub struct AttrMaxChoices {}

    impl AttrValue<i32> for AttrMaxChoices {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTES
        }

        fn attr_name() -> &'static str {
            COLUMN_MAX_CHOICES
        }
    }

    pub struct AttrEndTime {}

    impl AttrValue<i32> for AttrEndTime {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTES
        }

        fn attr_name() -> &'static str {
            COLUMN_END_TIME
        }
    }

    pub struct AttrVisibilityRule {}

    impl AttrValue<String> for AttrVisibilityRule {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTES
        }

        fn attr_name() -> &'static str {
            COLUMN_VISIBILITY_RULE
        }
    }
} // end mod object
