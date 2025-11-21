use crate::common::result::RS;
use crate::data_type::dat_type::DatType;
use crate::tuple::{datum_desc::DatumDesc, tuple_binary_desc::TupleBinaryDesc};
use serde::{Deserialize, Serialize};

/// Describes the structure and types of a tuple's elements
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TupleFieldDesc {
    fields: Vec<DatumDesc>,
}

impl TupleFieldDesc {
    /// Creates a new TupleItemDesc from a collection of field descriptions
    pub fn new(fields: Vec<DatumDesc>) -> Self {
        Self { fields }
    }

    /// Returns a reference to the field descriptions
    pub fn fields(&self) -> &[DatumDesc] {
        &self.fields
    }

    pub fn into_fields(self) -> Vec<DatumDesc> {
        self.fields
    }

    /// Converts to a binary tuple description with index mapping
    /// Returns a tuple of (binary_descriptor, original_to_normalized_index_mapping)
    pub fn to_tuple_binary_desc(&self) -> RS<(TupleBinaryDesc, Vec<usize>)> {
        let type_descs_with_indices: Vec<(DatType, usize)> = self
            .fields
            .iter()
            .enumerate()
            .map(|(original_index, field_desc)| {
                let type_desc = field_desc.dat_type();
                (type_desc.clone(), original_index)
            })
            .collect();

        let (normalized_type_descs, index_mapping) =
            TupleBinaryDesc::normalized_type_desc_vec(type_descs_with_indices)?;

        let binary_desc = TupleBinaryDesc::from(normalized_type_descs)?;
        Ok((binary_desc, index_mapping))
    }
}

impl AsRef<TupleFieldDesc> for TupleFieldDesc {
    fn as_ref(&self) -> &TupleFieldDesc {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_type::dat_type::DatType;
    use crate::data_type::dat_type_id::DatTypeID;
    use crate::{
        common::serde_utils::{deserialize_from_json, serialize_to_json},
        tuple::datum_desc::DatumDesc,
    };

    #[test]
    fn test_serialization_round_trip() {
        let fields = vec![
            DatumDesc::new("c1".to_string(), DatType::default_for(DatTypeID::I32)),
            DatumDesc::new("c2".to_string(), DatType::default_for(DatTypeID::I64)),
            DatumDesc::new("c3".to_string(), DatType::default_for(DatTypeID::I32)),
        ];

        let original_desc = TupleFieldDesc::new(fields);
        let json = serialize_to_json(&original_desc).unwrap();
        println!("Serialized JSON:\n{}", json);

        let deserialized_desc: TupleFieldDesc = deserialize_from_json(&json).unwrap();
        assert_eq!(
            original_desc.fields().len(),
            deserialized_desc.fields().len()
        );
    }
}
