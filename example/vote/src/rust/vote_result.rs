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

    const TABLE_VOTE_RESULT: &str = "vote_result";
    const COLUMN_VOTE_ID: &str = "vote_id";
    const COLUMN_TOPIC: &str = "topic";
    const COLUMN_VOTE_ENDED: &str = "vote_ended";
    const COLUMN_TOTAL_VOTES: &str = "total_votes";
    const COLUMN_OPTIONS: &str = "options";

    #[derive(Debug, Clone)]
    pub struct VoteResult {
        vote_id: Option<String>,
        topic: Option<String>,
        vote_ended: Option<i32>,
        total_votes: Option<i32>,
        options: Option<String>,
    }

    impl VoteResult {
        pub fn new(
            vote_id: Option<String>,
            topic: Option<String>,
            vote_ended: Option<i32>,
            total_votes: Option<i32>,
            options: Option<String>,
        ) -> Self {
            let s = Self {
                vote_id,
                topic,
                vote_ended,
                total_votes,
                options,
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

        pub fn set_vote_ended(&mut self, vote_ended: i32) {
            self.vote_ended = Some(vote_ended);
        }

        pub fn get_vote_ended(&self) -> &Option<i32> {
            &self.vote_ended
        }

        pub fn set_total_votes(&mut self, total_votes: i32) {
            self.total_votes = Some(total_votes);
        }

        pub fn get_total_votes(&self) -> &Option<i32> {
            &self.total_votes
        }

        pub fn set_options(&mut self, options: String) {
            self.options = Some(options);
        }

        pub fn get_options(&self) -> &Option<String> {
            &self.options
        }
    }

    impl Datum for VoteResult {
        fn dat_type() -> &'static DatType {
            lazy_static! {
                static ref DAT_TYPE: DatType = entity_utils::entity_dat_type::<VoteResult>();
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

    impl DatumDyn for VoteResult {
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

    impl Entity for VoteResult {
        fn new_empty() -> Self {
            let s = Self {
                vote_id: None,
                topic: None,
                vote_ended: None,
                total_votes: None,
                options: None,
            };
            s
        }
        fn tuple_desc() -> &'static TupleFieldDesc {
            lazy_static! {
                static ref TUPLE_DESC: TupleFieldDesc = TupleFieldDesc::new(vec![
                    AttrVoteId::datum_desc().clone(),
                    AttrTopic::datum_desc().clone(),
                    AttrVoteEnded::datum_desc().clone(),
                    AttrTotalVotes::datum_desc().clone(),
                    AttrOptions::datum_desc().clone(),
                ]);
            }
            &TUPLE_DESC
        }

        fn object_name() -> &'static str {
            TABLE_VOTE_RESULT
        }

        fn get_field_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_VOTE_ID => attr_field_access::attr_get_binary::<_>(&self.vote_id),
                COLUMN_TOPIC => attr_field_access::attr_get_binary::<_>(&self.topic),
                COLUMN_VOTE_ENDED => attr_field_access::attr_get_binary::<_>(&self.vote_ended),
                COLUMN_TOTAL_VOTES => attr_field_access::attr_get_binary::<_>(&self.total_votes),
                COLUMN_OPTIONS => attr_field_access::attr_get_binary::<_>(&self.options),
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
                COLUMN_VOTE_ENDED => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.vote_ended,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_TOTAL_VOTES => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.total_votes,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_OPTIONS => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.options, binary.as_ref())?;
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
                COLUMN_VOTE_ENDED => attr_field_access::attr_get_value::<_>(&self.vote_ended),
                COLUMN_TOTAL_VOTES => attr_field_access::attr_get_value::<_>(&self.total_votes),
                COLUMN_OPTIONS => attr_field_access::attr_get_value::<_>(&self.options),
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
                COLUMN_VOTE_ENDED => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.vote_ended, value)?;
                }
                COLUMN_TOTAL_VOTES => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.total_votes, value)?;
                }
                COLUMN_OPTIONS => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.options, value)?;
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
            TABLE_VOTE_RESULT
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
            TABLE_VOTE_RESULT
        }

        fn attr_name() -> &'static str {
            COLUMN_TOPIC
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
            TABLE_VOTE_RESULT
        }

        fn attr_name() -> &'static str {
            COLUMN_VOTE_ENDED
        }
    }

    pub struct AttrTotalVotes {}

    impl AttrValue<i32> for AttrTotalVotes {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTE_RESULT
        }

        fn attr_name() -> &'static str {
            COLUMN_TOTAL_VOTES
        }
    }

    pub struct AttrOptions {}

    impl AttrValue<String> for AttrOptions {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTE_RESULT
        }

        fn attr_name() -> &'static str {
            COLUMN_OPTIONS
        }
    }
} // end mod object
