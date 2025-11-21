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

    const TABLE_VOTE_CHOICES: &str = "vote_choices";
    const COLUMN_CHOICE_ID: &str = "choice_id";
    const COLUMN_ACTION_ID: &str = "action_id";
    const COLUMN_OPTION_ID: &str = "option_id";

    #[derive(Debug, Clone)]
    pub struct VoteChoices {
        choice_id: Option<String>,
        action_id: Option<String>,
        option_id: Option<String>,
    }

    impl VoteChoices {
        pub fn new(
            choice_id: Option<String>,
            action_id: Option<String>,
            option_id: Option<String>,
        ) -> Self {
            let s = Self {
                choice_id,
                action_id,
                option_id,
            };
            s
        }

        pub fn set_choice_id(&mut self, choice_id: String) {
            self.choice_id = Some(choice_id);
        }

        pub fn get_choice_id(&self) -> &Option<String> {
            &self.choice_id
        }

        pub fn set_action_id(&mut self, action_id: String) {
            self.action_id = Some(action_id);
        }

        pub fn get_action_id(&self) -> &Option<String> {
            &self.action_id
        }

        pub fn set_option_id(&mut self, option_id: String) {
            self.option_id = Some(option_id);
        }

        pub fn get_option_id(&self) -> &Option<String> {
            &self.option_id
        }
    }

    impl Datum for VoteChoices {
        fn dat_type() -> &'static DatType {
            lazy_static! {
                static ref DAT_TYPE: DatType = entity_utils::entity_dat_type::<VoteChoices>();
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

    impl DatumDyn for VoteChoices {
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

    impl Entity for VoteChoices {
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

        fn object_name() -> &'static str {
            TABLE_VOTE_CHOICES
        }

        fn get_field_binary(&self, column: &str) -> RS<Option<Vec<u8>>> {
            match column {
                COLUMN_CHOICE_ID => attr_field_access::attr_get_binary::<_>(&self.choice_id),
                COLUMN_ACTION_ID => attr_field_access::attr_get_binary::<_>(&self.action_id),
                COLUMN_OPTION_ID => attr_field_access::attr_get_binary::<_>(&self.option_id),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_binary<B: AsRef<[u8]>>(&mut self, column: &str, binary: B) -> RS<()> {
            match column {
                COLUMN_CHOICE_ID => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.choice_id,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_ACTION_ID => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.action_id,
                        binary.as_ref(),
                    )?;
                }
                COLUMN_OPTION_ID => {
                    attr_field_access::attr_set_binary::<_, _>(
                        &mut self.option_id,
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
                COLUMN_CHOICE_ID => attr_field_access::attr_get_value::<_>(&self.choice_id),
                COLUMN_ACTION_ID => attr_field_access::attr_get_value::<_>(&self.action_id),
                COLUMN_OPTION_ID => attr_field_access::attr_get_value::<_>(&self.option_id),
                _ => {
                    panic!("unknown name");
                }
            }
        }

        fn set_field_value<B: AsRef<DatValue>>(&mut self, column: &str, value: B) -> RS<()> {
            match column {
                COLUMN_CHOICE_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.choice_id, value)?;
                }
                COLUMN_ACTION_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.action_id, value)?;
                }
                COLUMN_OPTION_ID => {
                    attr_field_access::attr_set_value::<_, _>(&mut self.option_id, value)?;
                }
                _ => {
                    panic!("unknown name");
                }
            }
            Ok(())
        }
    }

    pub struct AttrChoiceId {}

    impl AttrValue<String> for AttrChoiceId {
        fn dat_type() -> &'static DatType {
            static ONCE_LOCK: std::sync::OnceLock<DatType> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_dat_type())
        }

        fn datum_desc() -> &'static DatumDesc {
            static ONCE_LOCK: std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();
            ONCE_LOCK.get_or_init(|| Self::attr_datum_desc())
        }

        fn object_name() -> &'static str {
            TABLE_VOTE_CHOICES
        }

        fn attr_name() -> &'static str {
            COLUMN_CHOICE_ID
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
            TABLE_VOTE_CHOICES
        }

        fn attr_name() -> &'static str {
            COLUMN_ACTION_ID
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
            TABLE_VOTE_CHOICES
        }

        fn attr_name() -> &'static str {
            COLUMN_OPTION_ID
        }
    }
} // end mod object
