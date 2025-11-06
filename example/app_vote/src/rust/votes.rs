pub mod object {
    use lazy_static::lazy_static;
    use mudu::common::result::RS;
    use mudu::database::attr_binary::AttrBinary;
    use mudu::database::attr_set_get::{attr_get_binary, attr_set_binary};
    use mudu::database::attr_value::AttrValue;
    use mudu::database::record::Record;
    use mudu::database::record_convert_tuple::{record_from_tuple, record_to_tuple};
    use mudu::tuple::datum_convert::{datum_from_binary, datum_to_binary};
    use mudu::tuple::tuple_field::TupleField;
    use mudu::tuple::tuple_field_desc::TupleFieldDesc;

    const TABLE_VOTES: &str = "votes";
    const COLUMN_VOTE_ID: &str = "vote_id";
    const COLUMN_CREATOR_ID: &str = "creator_id";
    const COLUMN_TOPIC: &str = "topic";
    const COLUMN_VOTE_TYPE: &str = "vote_type";
    const COLUMN_MAX_CHOICES: &str = "max_choices";
    const COLUMN_END_TIME: &str = "end_time";
    const COLUMN_VISIBILITY_RULE: &str = "visibility_rule";

    pub struct Votes {
        vote_id: Option<AttrVoteId>,
        creator_id: Option<AttrCreatorId>,
        topic: Option<AttrTopic>,
        vote_type: Option<AttrVoteType>,
        max_choices: Option<AttrMaxChoices>,
        end_time: Option<AttrEndTime>,
        visibility_rule: Option<AttrVisibilityRule>,
    }

    impl Votes {
        pub fn new(
            vote_id: AttrVoteId,
            creator_id: AttrCreatorId,
            topic: AttrTopic,
            vote_type: AttrVoteType,
            max_choices: AttrMaxChoices,
            end_time: AttrEndTime,
            visibility_rule: AttrVisibilityRule,
        ) -> Self {
            let s = Self {
                vote_id: Some(vote_id),
                creator_id: Some(creator_id),
                topic: Some(topic),
                vote_type: Some(vote_type),
                max_choices: Some(max_choices),
                end_time: Some(end_time),
                visibility_rule: Some(visibility_rule),
            };
            s
        }

        pub fn set_vote_id(&mut self, vote_id: AttrVoteId) {
            self.vote_id = Some(vote_id);
        }

        pub fn get_vote_id(&self) -> &Option<AttrVoteId> {
            &self.vote_id
        }

        pub fn set_creator_id(&mut self, creator_id: AttrCreatorId) {
            self.creator_id = Some(creator_id);
        }

        pub fn get_creator_id(&self) -> &Option<AttrCreatorId> {
            &self.creator_id
        }

        pub fn set_topic(&mut self, topic: AttrTopic) {
            self.topic = Some(topic);
        }

        pub fn get_topic(&self) -> &Option<AttrTopic> {
            &self.topic
        }

        pub fn set_vote_type(&mut self, vote_type: AttrVoteType) {
            self.vote_type = Some(vote_type);
        }

        pub fn get_vote_type(&self) -> &Option<AttrVoteType> {
            &self.vote_type
        }

        pub fn set_max_choices(&mut self, max_choices: AttrMaxChoices) {
            self.max_choices = Some(max_choices);
        }

        pub fn get_max_choices(&self) -> &Option<AttrMaxChoices> {
            &self.max_choices
        }

        pub fn set_end_time(&mut self, end_time: AttrEndTime) {
            self.end_time = Some(end_time);
        }

        pub fn get_end_time(&self) -> &Option<AttrEndTime> {
            &self.end_time
        }

        pub fn set_visibility_rule(&mut self, visibility_rule: AttrVisibilityRule) {
            self.visibility_rule = Some(visibility_rule);
        }

        pub fn get_visibility_rule(&self) -> &Option<AttrVisibilityRule> {
            &self.visibility_rule
        }
    }

    impl Record for Votes {
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

        fn table_name() -> &'static str {
            TABLE_VOTES
        }

        fn from_tuple<T: AsRef<TupleField>, D: AsRef<TupleFieldDesc>>(row: T, desc: D) -> RS<Self> {
            record_from_tuple::<Self, T, D>(row, desc)
        }

        fn to_tuple<D: AsRef<TupleFieldDesc>>(&self, desc: D) -> RS<TupleField> {
            record_to_tuple(self, desc)
        }

        fn get_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_VOTE_ID => attr_get_binary(&self.vote_id),
                COLUMN_CREATOR_ID => attr_get_binary(&self.creator_id),
                COLUMN_TOPIC => attr_get_binary(&self.topic),
                COLUMN_VOTE_TYPE => attr_get_binary(&self.vote_type),
                COLUMN_MAX_CHOICES => attr_get_binary(&self.max_choices),
                COLUMN_END_TIME => attr_get_binary(&self.end_time),
                COLUMN_VISIBILITY_RULE => attr_get_binary(&self.visibility_rule),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_binary<B: AsRef<[u8]>>(&mut self, column: &str, binary: B) -> RS<()> {
            match column {
                COLUMN_VOTE_ID => {
                    attr_set_binary(&mut self.vote_id, binary.as_ref())?;
                }
                COLUMN_CREATOR_ID => {
                    attr_set_binary(&mut self.creator_id, binary.as_ref())?;
                }
                COLUMN_TOPIC => {
                    attr_set_binary(&mut self.topic, binary.as_ref())?;
                }
                COLUMN_VOTE_TYPE => {
                    attr_set_binary(&mut self.vote_type, binary.as_ref())?;
                }
                COLUMN_MAX_CHOICES => {
                    attr_set_binary(&mut self.max_choices, binary.as_ref())?;
                }
                COLUMN_END_TIME => {
                    attr_set_binary(&mut self.end_time, binary.as_ref())?;
                }
                COLUMN_VISIBILITY_RULE => {
                    attr_set_binary(&mut self.visibility_rule, binary.as_ref())?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
        }
    }

    pub struct AttrVoteId {
        value: String,
    }

    impl AttrVoteId {}

    impl AttrBinary for AttrVoteId {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: String = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<String> for AttrVoteId {
        fn new(datum: String) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTES
        }

        fn column_name() -> &'static str {
            COLUMN_VOTE_ID
        }

        fn get_value(&self) -> String {
            self.value.clone()
        }

        fn set_value(&mut self, value: String) {
            self.value = value;
        }
    }

    pub struct AttrCreatorId {
        value: String,
    }

    impl AttrCreatorId {}

    impl AttrBinary for AttrCreatorId {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: String = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<String> for AttrCreatorId {
        fn new(datum: String) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTES
        }

        fn column_name() -> &'static str {
            COLUMN_CREATOR_ID
        }

        fn get_value(&self) -> String {
            self.value.clone()
        }

        fn set_value(&mut self, value: String) {
            self.value = value;
        }
    }

    pub struct AttrTopic {
        value: String,
    }

    impl AttrTopic {}

    impl AttrBinary for AttrTopic {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: String = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<String> for AttrTopic {
        fn new(datum: String) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTES
        }

        fn column_name() -> &'static str {
            COLUMN_TOPIC
        }

        fn get_value(&self) -> String {
            self.value.clone()
        }

        fn set_value(&mut self, value: String) {
            self.value = value;
        }
    }

    pub struct AttrVoteType {
        value: String,
    }

    impl AttrVoteType {}

    impl AttrBinary for AttrVoteType {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: String = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<String> for AttrVoteType {
        fn new(datum: String) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTES
        }

        fn column_name() -> &'static str {
            COLUMN_VOTE_TYPE
        }

        fn get_value(&self) -> String {
            self.value.clone()
        }

        fn set_value(&mut self, value: String) {
            self.value = value;
        }
    }

    pub struct AttrMaxChoices {
        value: i32,
    }

    impl AttrMaxChoices {}

    impl AttrBinary for AttrMaxChoices {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: i32 = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<i32> for AttrMaxChoices {
        fn new(datum: i32) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTES
        }

        fn column_name() -> &'static str {
            COLUMN_MAX_CHOICES
        }

        fn get_value(&self) -> i32 {
            self.value.clone()
        }

        fn set_value(&mut self, value: i32) {
            self.value = value;
        }
    }

    pub struct AttrEndTime {
        value: i32,
    }

    impl AttrEndTime {}

    impl AttrBinary for AttrEndTime {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: i32 = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<i32> for AttrEndTime {
        fn new(datum: i32) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTES
        }

        fn column_name() -> &'static str {
            COLUMN_END_TIME
        }

        fn get_value(&self) -> i32 {
            self.value.clone()
        }

        fn set_value(&mut self, value: i32) {
            self.value = value;
        }
    }

    pub struct AttrVisibilityRule {
        value: String,
    }

    impl AttrVisibilityRule {}

    impl AttrBinary for AttrVisibilityRule {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: String = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<String> for AttrVisibilityRule {
        fn new(datum: String) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTES
        }

        fn column_name() -> &'static str {
            COLUMN_VISIBILITY_RULE
        }

        fn get_value(&self) -> String {
            self.value.clone()
        }

        fn set_value(&mut self, value: String) {
            self.value = value;
        }
    }
} // end mod object
