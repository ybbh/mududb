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

    const TABLE_OPTIONS: &str = "options";
    const COLUMN_OPTION_ID: &str = "option_id";
    const COLUMN_VOTE_ID: &str = "vote_id";
    const COLUMN_OPTION_TEXT: &str = "option_text";

    #[derive(Debug, Clone)]
    pub struct Options {
        option_id: Option<String>,
        vote_id: Option<String>,
        option_text: Option<String>,
    }

    impl Options {
        pub fn new(
            option_id: Option<String>,
            vote_id: Option<String>,
            option_text: Option<String>,
        ) -> Self {
            let s = Self {
                option_id,
                vote_id,
                option_text,
            };
            s
        }

        pub fn set_option_id(&mut self, option_id: String) {
            self.option_id = Some(option_id);
        }

        pub fn get_option_id(&self) -> &Option<String> {
            &self.option_id
        }

        pub fn set_vote_id(&mut self, vote_id: String) {
            self.vote_id = Some(vote_id);
        }

        pub fn get_vote_id(&self) -> &Option<String> {
            &self.vote_id
        }

        pub fn set_option_text(&mut self, option_text: String) {
            self.option_text = Some(option_text);
        }

        pub fn get_option_text(&self) -> &Option<String> {
            &self.option_text
        }
    }

    impl Datum for Options {
        fn dat_type() -> &'static DatType {
            lazy_static! {
                static ref DAT_TYPE: DatType = entity_utils::entity_dat_type::<Options>();
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

    impl DatumDyn for Options {
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

    impl Entity for Options {
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

        fn object_name() -> &'static str {
            TABLE_OPTIONS
        }

        fn get_field_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_OPTION_ID => attr_field_access::attr_get_binary::<_>(&self.option_id),
                COLUMN_VOTE_ID => attr_field_access::attr_get_binary::<_>(&self.vote_id),
                COLUMN_OPTION_TEXT => attr_field_access::attr_get_binary::<_>(&self.option_text),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_binary<B: AsRef<[u8]>>(&mut self, column: &str, binary: B) -> RS<()> {
            match column {
                COLUMN_OPTION_ID => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.option_id,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_VOTE_ID => {
                    attr_field_access::attr_set_binary::<_, _>(&mut self.vote_id, binary.as_ref())?;
                }
                COLUMN_OPTION_TEXT => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.option_text,
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
                COLUMN_OPTION_ID => attr_field_access::attr_get_value::<_>(&self.option_id),
                COLUMN_VOTE_ID => attr_field_access::attr_get_value::<_>(&self.vote_id),
                COLUMN_OPTION_TEXT => attr_field_access::attr_get_value::<_>(&self.option_text),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_value<B: AsRef<DatValue>>(&mut self, column: &str, value: B) -> RS<()> {
            match column {
                COLUMN_OPTION_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.option_id, value)?;
                }
                COLUMN_VOTE_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.vote_id, value)?;
                }
                COLUMN_OPTION_TEXT => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.option_text, value)?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
        }
    }

    pub struct AttrOptionId {}

    impl AttrValue<String> for AttrOptionId {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_OPTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_OPTION_ID
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
            TABLE_OPTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_VOTE_ID
        }
    }

    pub struct AttrOptionText {}

    impl AttrValue<String> for AttrOptionText {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_OPTIONS
        }

        fn attr_name() -> &'static str {
            COLUMN_OPTION_TEXT
        }
    }
} // end mod object
