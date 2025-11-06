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

    const TABLE_VOTE_ACTIONS: &str = "vote_actions";
    const COLUMN_ACTION_ID: &str = "action_id";
    const COLUMN_USER_ID: &str = "user_id";
    const COLUMN_VOTE_ID: &str = "vote_id";
    const COLUMN_ACTION_TIME: &str = "action_time";
    const COLUMN_IS_WITHDRAWN: &str = "is_withdrawn";

    pub struct VoteActions {
        action_id: Option<AttrActionId>,
        user_id: Option<AttrUserId>,
        vote_id: Option<AttrVoteId>,
        action_time: Option<AttrActionTime>,
        is_withdrawn: Option<AttrIsWithdrawn>,
    }

    impl VoteActions {
        pub fn new(
            action_id: AttrActionId,
            user_id: AttrUserId,
            vote_id: AttrVoteId,
            action_time: AttrActionTime,
            is_withdrawn: AttrIsWithdrawn,
        ) -> Self {
            let s = Self {
                action_id: Some(action_id),
                user_id: Some(user_id),
                vote_id: Some(vote_id),
                action_time: Some(action_time),
                is_withdrawn: Some(is_withdrawn),
            };
            s
        }

        pub fn set_action_id(&mut self, action_id: AttrActionId) {
            self.action_id = Some(action_id);
        }

        pub fn get_action_id(&self) -> &Option<AttrActionId> {
            &self.action_id
        }

        pub fn set_user_id(&mut self, user_id: AttrUserId) {
            self.user_id = Some(user_id);
        }

        pub fn get_user_id(&self) -> &Option<AttrUserId> {
            &self.user_id
        }

        pub fn set_vote_id(&mut self, vote_id: AttrVoteId) {
            self.vote_id = Some(vote_id);
        }

        pub fn get_vote_id(&self) -> &Option<AttrVoteId> {
            &self.vote_id
        }

        pub fn set_action_time(&mut self, action_time: AttrActionTime) {
            self.action_time = Some(action_time);
        }

        pub fn get_action_time(&self) -> &Option<AttrActionTime> {
            &self.action_time
        }

        pub fn set_is_withdrawn(&mut self, is_withdrawn: AttrIsWithdrawn) {
            self.is_withdrawn = Some(is_withdrawn);
        }

        pub fn get_is_withdrawn(&self) -> &Option<AttrIsWithdrawn> {
            &self.is_withdrawn
        }
    }

    impl Record for VoteActions {
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

        fn table_name() -> &'static str {
            TABLE_VOTE_ACTIONS
        }

        fn from_tuple<T: AsRef<TupleField>, D: AsRef<TupleFieldDesc>>(row: T, desc: D) -> RS<Self> {
            record_from_tuple::<Self, T, D>(row, desc)
        }

        fn to_tuple<D: AsRef<TupleFieldDesc>>(&self, desc: D) -> RS<TupleField> {
            record_to_tuple(self, desc)
        }

        fn get_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_ACTION_ID => attr_get_binary(&self.action_id),
                COLUMN_USER_ID => attr_get_binary(&self.user_id),
                COLUMN_VOTE_ID => attr_get_binary(&self.vote_id),
                COLUMN_ACTION_TIME => attr_get_binary(&self.action_time),
                COLUMN_IS_WITHDRAWN => attr_get_binary(&self.is_withdrawn),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_binary<B: AsRef<[u8]>>(&mut self, column: &str, binary: B) -> RS<()> {
            match column {
                COLUMN_ACTION_ID => {
                    attr_set_binary(&mut self.action_id, binary.as_ref())?;
                }
                COLUMN_USER_ID => {
                    attr_set_binary(&mut self.user_id, binary.as_ref())?;
                }
                COLUMN_VOTE_ID => {
                    attr_set_binary(&mut self.vote_id, binary.as_ref())?;
                }
                COLUMN_ACTION_TIME => {
                    attr_set_binary(&mut self.action_time, binary.as_ref())?;
                }
                COLUMN_IS_WITHDRAWN => {
                    attr_set_binary(&mut self.is_withdrawn, binary.as_ref())?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
        }
    }

    pub struct AttrActionId {
        value: String,
    }

    impl AttrActionId {}

    impl AttrBinary for AttrActionId {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: String = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<String> for AttrActionId {
        fn new(datum: String) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTE_ACTIONS
        }

        fn column_name() -> &'static str {
            COLUMN_ACTION_ID
        }

        fn get_value(&self) -> String {
            self.value.clone()
        }

        fn set_value(&mut self, value: String) {
            self.value = value;
        }
    }

    pub struct AttrUserId {
        value: String,
    }

    impl AttrUserId {}

    impl AttrBinary for AttrUserId {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: String = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<String> for AttrUserId {
        fn new(datum: String) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTE_ACTIONS
        }

        fn column_name() -> &'static str {
            COLUMN_USER_ID
        }

        fn get_value(&self) -> String {
            self.value.clone()
        }

        fn set_value(&mut self, value: String) {
            self.value = value;
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
            TABLE_VOTE_ACTIONS
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

    pub struct AttrActionTime {
        value: i32,
    }

    impl AttrActionTime {}

    impl AttrBinary for AttrActionTime {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: i32 = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<i32> for AttrActionTime {
        fn new(datum: i32) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTE_ACTIONS
        }

        fn column_name() -> &'static str {
            COLUMN_ACTION_TIME
        }

        fn get_value(&self) -> i32 {
            self.value.clone()
        }

        fn set_value(&mut self, value: i32) {
            self.value = value;
        }
    }

    pub struct AttrIsWithdrawn {
        value: i32,
    }

    impl AttrIsWithdrawn {}

    impl AttrBinary for AttrIsWithdrawn {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: i32 = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<i32> for AttrIsWithdrawn {
        fn new(datum: i32) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTE_ACTIONS
        }

        fn column_name() -> &'static str {
            COLUMN_IS_WITHDRAWN
        }

        fn get_value(&self) -> i32 {
            self.value.clone()
        }

        fn set_value(&mut self, value: i32) {
            self.value = value;
        }
    }
} // end mod object
