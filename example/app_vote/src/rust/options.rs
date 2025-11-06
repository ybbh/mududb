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

    const TABLE_OPTIONS: &str = "options";
    const COLUMN_OPTION_ID: &str = "option_id";
    const COLUMN_VOTE_ID: &str = "vote_id";
    const COLUMN_OPTION_TEXT: &str = "option_text";

    pub struct Options {
        option_id: Option<AttrOptionId>,
        vote_id: Option<AttrVoteId>,
        option_text: Option<AttrOptionText>,
    }

    impl Options {
        pub fn new(
            option_id: AttrOptionId,
            vote_id: AttrVoteId,
            option_text: AttrOptionText,
        ) -> Self {
            let s = Self {
                option_id: Some(option_id),
                vote_id: Some(vote_id),
                option_text: Some(option_text),
            };
            s
        }

        pub fn set_option_id(&mut self, option_id: AttrOptionId) {
            self.option_id = Some(option_id);
        }

        pub fn get_option_id(&self) -> &Option<AttrOptionId> {
            &self.option_id
        }

        pub fn set_vote_id(&mut self, vote_id: AttrVoteId) {
            self.vote_id = Some(vote_id);
        }

        pub fn get_vote_id(&self) -> &Option<AttrVoteId> {
            &self.vote_id
        }

        pub fn set_option_text(&mut self, option_text: AttrOptionText) {
            self.option_text = Some(option_text);
        }

        pub fn get_option_text(&self) -> &Option<AttrOptionText> {
            &self.option_text
        }
    }

    impl Record for Options {
        fn new_empty() -> Self {
            let s = Self {
                option_id: None,
                vote_id: None,
                option_text: None,
            };
            s
        }
        fn tuple_desc() -> &'static TupleFieldDesc {
            lazy_static! {
                static ref TUPLE_DESC: TupleFieldDesc = TupleFieldDesc::new(vec![
                    AttrOptionId::datum_desc().clone(),
                    AttrVoteId::datum_desc().clone(),
                    AttrOptionText::datum_desc().clone(),
                ]);
            }
            &TUPLE_DESC
        }

        fn table_name() -> &'static str {
            TABLE_OPTIONS
        }

        fn from_tuple<T: AsRef<TupleField>, D: AsRef<TupleFieldDesc>>(row: T, desc: D) -> RS<Self> {
            record_from_tuple::<Self, T, D>(row, desc)
        }

        fn to_tuple<D: AsRef<TupleFieldDesc>>(&self, desc: D) -> RS<TupleField> {
            record_to_tuple(self, desc)
        }

        fn get_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_OPTION_ID => attr_get_binary(&self.option_id),
                COLUMN_VOTE_ID => attr_get_binary(&self.vote_id),
                COLUMN_OPTION_TEXT => attr_get_binary(&self.option_text),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_binary<B: AsRef<[u8]>>(&mut self, column: &str, binary: B) -> RS<()> {
            match column {
                COLUMN_OPTION_ID => {
                    attr_set_binary(&mut self.option_id, binary.as_ref())?;
                }
                COLUMN_VOTE_ID => {
                    attr_set_binary(&mut self.vote_id, binary.as_ref())?;
                }
                COLUMN_OPTION_TEXT => {
                    attr_set_binary(&mut self.option_text, binary.as_ref())?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
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
            TABLE_OPTIONS
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
            TABLE_OPTIONS
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

    pub struct AttrOptionText {
        value: String,
    }

    impl AttrOptionText {}

    impl AttrBinary for AttrOptionText {
        fn get_binary(&self) -> RS<Vec<u8>> {
            datum_to_binary(&self.value)
        }

        fn set_binary<D: AsRef<[u8]>>(&mut self, binary: D) -> RS<()> {
            let value: String = datum_from_binary(binary.as_ref())?;
            self.set_value(value);
            Ok(())
        }
    }

    impl AttrValue<String> for AttrOptionText {
        fn new(datum: String) -> Self {
            Self { value: datum }
        }

        fn from_binary<B: AsRef<[u8]>>(binary: B) -> RS<Self> {
            Ok(Self::new(datum_from_binary(binary)?))
        }

        fn table_name() -> &'static str {
            TABLE_OPTIONS
        }

        fn column_name() -> &'static str {
            COLUMN_OPTION_TEXT
        }

        fn get_value(&self) -> String {
            self.value.clone()
        }

        fn set_value(&mut self, value: String) {
            self.value = value;
        }
    }
} // end mod object
