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

    const TABLE_VOTE_CHOICES: &str = "vote_choices";
    const COLUMN_CHOICE_ID: &str = "choice_id";
    const COLUMN_ACTION_ID: &str = "action_id";
    const COLUMN_OPTION_ID: &str = "option_id";

    pub struct VoteChoices {
        choice_id: Option<AttrChoiceId>,
        action_id: Option<AttrActionId>,
        option_id: Option<AttrOptionId>,
    }

    impl VoteChoices {
        pub fn new(
            choice_id: AttrChoiceId,
            action_id: AttrActionId,
            option_id: AttrOptionId,
        ) -> Self {
            let s = Self {
                choice_id: Some(choice_id),
                action_id: Some(action_id),
                option_id: Some(option_id),
            };
            s
        }

        pub fn set_choice_id(&mut self, choice_id: AttrChoiceId) {
            self.choice_id = Some(choice_id);
        }

        pub fn get_choice_id(&self) -> &Option<AttrChoiceId> {
            &self.choice_id
        }

        pub fn set_action_id(&mut self, action_id: AttrActionId) {
            self.action_id = Some(action_id);
        }

        pub fn get_action_id(&self) -> &Option<AttrActionId> {
            &self.action_id
        }

        pub fn set_option_id(&mut self, option_id: AttrOptionId) {
            self.option_id = Some(option_id);
        }

        pub fn get_option_id(&self) -> &Option<AttrOptionId> {
            &self.option_id
        }
    }

    impl Record for VoteChoices {
        fn new_empty() -> Self {
            let s = Self {
                choice_id: None,
                action_id: None,
                option_id: None,
            };
            s
        }
        fn tuple_desc() -> &'static TupleFieldDesc {
            lazy_static! {
                static ref TUPLE_DESC: TupleFieldDesc = TupleFieldDesc::new(vec![
                    AttrChoiceId::datum_desc().clone(),
                    AttrActionId::datum_desc().clone(),
                    AttrOptionId::datum_desc().clone(),
                ]);
            }
            &TUPLE_DESC
        }

        fn table_name() -> &'static str {
            TABLE_VOTE_CHOICES
        }

        fn from_tuple<T: AsRef<TupleField>, D: AsRef<TupleFieldDesc>>(row: T, desc: D) -> RS<Self> {
            record_from_tuple::<Self, T, D>(row, desc)
        }

        fn to_tuple<D: AsRef<TupleFieldDesc>>(&self, desc: D) -> RS<TupleField> {
            record_to_tuple(self, desc)
        }

        fn get_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_CHOICE_ID => attr_get_binary(&self.choice_id),
                COLUMN_ACTION_ID => attr_get_binary(&self.action_id),
                COLUMN_OPTION_ID => attr_get_binary(&self.option_id),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_binary<B: AsRef<[u8]>>(&mut self, column: &str, binary: B) -> RS<()> {
            match column {
                COLUMN_CHOICE_ID => {
                    attr_set_binary(&mut self.choice_id, binary.as_ref())?;
                }
                COLUMN_ACTION_ID => {
                    attr_set_binary(&mut self.action_id, binary.as_ref())?;
                }
                COLUMN_OPTION_ID => {
                    attr_set_binary(&mut self.option_id, binary.as_ref())?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
        }
    }

    pub struct AttrChoiceId {
        value: String,
    }

    impl AttrChoiceId {}

    impl AttrBinary for AttrChoiceId {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: String = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<String> for AttrChoiceId {
        fn new(datum: String) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTE_CHOICES
        }

        fn column_name() -> &'static str {
            COLUMN_CHOICE_ID
        }

        fn get_value(&self) -> String {
            self.value.clone()
        }

        fn set_value(&mut self, value: String) {
            self.value = value;
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
            TABLE_VOTE_CHOICES
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

    pub struct AttrOptionId {
        value: String,
    }

    impl AttrOptionId {}

    impl AttrBinary for AttrOptionId {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: String = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<String> for AttrOptionId {
        fn new(datum: String) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_VOTE_CHOICES
        }

        fn column_name() -> &'static str {
            COLUMN_OPTION_ID
        }

        fn get_value(&self) -> String {
            self.value.clone()
        }

        fn set_value(&mut self, value: String) {
            self.value = value;
        }
    }
} // end mod object
