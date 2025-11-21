use crate::src_gen::column_def::TableColumnDef;
use crate::src_gen::src_builder::SrcBuilder;
use crate::src_gen::table_def::TableDef;
use mudu::common::result::RS;
use mudu::data_type::dat_prim::DatPrim;
use mudu::data_type::dat_type_id::DatTypeID;
use mudu::error::ec::EC;
use mudu::error::err::MError;
use mudu::m_error;
use std::fmt;
use std::fmt::Write;
use tera::Tera;

pub struct RustBuilder {}

pub fn to_struct_field_name(column_name: &str) -> RS<String> {
    Ok(column_name.to_string().to_ascii_lowercase())
}

pub fn to_object_struct_name(table_name: &str) -> RS<String> {
    let object_name = snake_case_to_upper_camel_case(table_name);
    Ok(object_name)
}

fn to_attr_struct_name(column_name: &str) -> RS<String> {
    let n = snake_case_to_upper_camel_case(column_name);
    Ok(format!("Attr{}", n))
}

fn write_error(e: fmt::Error) -> MError {
    m_error!(EC::FmtWriteErr, "format write error", e)
}

fn print_indent(s: &str, indent_n: u32, writer: &mut dyn Write) -> RS<()> {
    let vec = s.split("\n").collect::<Vec<&str>>();
    let mut indent_space = String::new();
    for _i in 0..indent_n {
        indent_space.push_str("\t");
    }
    for (i, s) in vec.iter().enumerate() {
        if i != vec.len() - 1 {
            writer
                .write_fmt(format_args!("{indent_space}{s}\n"))
                .map_err(write_error)?;
        }
    }
    Ok(())
}

fn to_data_type_str(id: DatTypeID) -> RS<String> {
    let s = match id {
        DatTypeID::I32 => "i32".to_string(),
        DatTypeID::I64 => "i64".to_string(),
        DatTypeID::F32 => "f32".to_string(),
        DatTypeID::F64 => "f64".to_string(),
        DatTypeID::String => "String".to_string(),
        _ => {
            panic!("unsupported data type {:?}", id);
        }
    };
    Ok(s)
}


fn is_basic_type(_n: &DatPrim) -> bool {
    true
}

fn to_table_name_const(s: &str) -> String {
    format!("TABLE_{}", s.to_ascii_uppercase())
}

fn to_column_name_const(s: &str) -> String {
    format!("COLUMN_{}", s.to_ascii_uppercase())
}

fn snake_case_to_upper_camel_case(n: &str) -> String {
    n.split('_')
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

impl RustBuilder {
    pub fn new() -> Self {
        Self {}
    }

    fn const_list(&self, table_def: &TableDef, writer: &mut dyn Write) -> RS<()> {
        let str_table_name = table_def.table_name();
        let const_table_name = to_table_name_const(&str_table_name);
        writer
            .write_fmt(format_args!(
                "const {const_table_name}:&str = \"{str_table_name}\";\n"
            ))
            .map_err(write_error)?;
        for c in table_def.table_columns() {
            let str_column_name = c.column_name();
            let const_column_name = to_column_name_const(&str_column_name);
            writer
                .write_fmt(format_args!(
                    "const {const_column_name}:&str = \"{str_column_name}\";\n"
                ))
                .map_err(write_error)?;
        }
        Ok(())
    }

    fn use_mod(&self, writer: &mut dyn Write) -> RS<()> {
        let s_use_mod = r#"
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
    "#;
        writer
            .write_str(s_use_mod)
            .map_err(write_error)?;
        Ok(())
    }
    fn build_object(&self, table_def: &TableDef, writer: &mut dyn Write) -> RS<()> {
        writer
            .write_str("pub mod object {\n")
            .map_err(write_error)?;

        let mut use_mod = String::new();
        self.use_mod(&mut use_mod)?;
        print_indent(&use_mod, 1, writer)?;

        writer.write_str("\n\n").map_err(write_error)?;

        let mut const_list = String::new();
        self.const_list(table_def, &mut const_list)?;
        print_indent(&const_list, 1, writer)?;

        writer.write_str("\n\n").map_err(write_error)?;

        let mut obj_struct = String::new();
        self.object_struct(table_def, &mut obj_struct)?;
        print_indent(&obj_struct, 1, writer)?;

        let mut obj_impl = String::new();
        self.impl_object(table_def, &mut obj_impl)?;
        print_indent(&obj_impl, 1, writer)?;

        let mut datum_impl = String::new();
        self.impl_datum_trait(table_def, &mut datum_impl)?;
        print_indent(&datum_impl, 1, writer)?;
        writer.write_str("\n").map_err(write_error)?;

        let mut impl_record_trait = String::new();
        self.impl_record_trait(table_def, &mut impl_record_trait)?;
        print_indent(&impl_record_trait, 1, writer)?;

        writer.write_fmt(format_args!("\n")).map_err(write_error)?;

        writer.write_fmt(format_args!("\n")).map_err(write_error)?;

        self.struct_impl_attribute_list(table_def, writer)?;

        writer
            .write_str("} // end mod object\n")
            .map_err(write_error)?;

        Ok(())
    }

    fn struct_impl_attribute_list(&self, table_def: &TableDef, writer: &mut dyn Write) -> RS<()> {
        let mut attributes = vec![];
        for column in table_def.table_columns() {
            let mut attribute = String::new();
            self.struct_impl_attribute(table_def.table_name(), column, &mut attribute)?;
            attributes.push(attribute);
        }

        for (i, attribute) in attributes.iter().enumerate() {
            print_indent(attribute, 1, writer)?;
            if i != attributes.len() - 1 {
                writer.write_str("\n").map_err(write_error)?;
            }
        }
        Ok(())
    }

    fn object_struct(&self, def: &TableDef, writer: &mut dyn Write) -> RS<()> {
        let name = to_object_struct_name(def.table_name())?;
        writer.write_fmt(format_args!("#[derive(Debug, Clone)]\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("pub struct {name} {{\n"))
            .map_err(write_error)?;

        for column in def.table_columns() {
            let mut field = String::new();
            self.build_field(column, &mut field)?;
            writer
                .write_fmt(format_args!("\t{field}"))
                .map_err(write_error)?;
        }
        writer
            .write_fmt(format_args!("}}\n\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn impl_object(&self, def: &TableDef, writer: &mut dyn Write) -> RS<()> {
        let name = to_object_struct_name(def.table_name())?;
        writer
            .write_fmt(format_args!("impl {name} {{\n"))
            .map_err(write_error)?;
        let mut methods = Vec::new();
        let mut constructor = String::new();
        self.impl_object_fn_constructor(def, &mut constructor, false)?;
        methods.push(constructor);

        for column in def.table_columns() {
            let mut setter = String::new();
            self.build_setter(column, &mut setter)?;
            methods.push(setter);

            let mut getter = String::new();
            self.build_getter(column, &mut getter)?;
            methods.push(getter);
        }

        for (i, method) in methods.iter().enumerate() {
            print_indent(method, 1, writer)?;
            if i != methods.len() - 1 {
                writer.write_fmt(format_args!("\n")).map_err(write_error)?;
            }
        }

        writer.write_str("}\n").map_err(write_error)?;
        Ok(())
    }
    fn impl_datum_trait(&self, table_def:&TableDef, writer: &mut dyn Write) -> RS<()> {
        let s = r#"
impl Datum for {{entry_struct_name}} {
    fn dat_type() -> &'static DatType {
        lazy_static! {
            static ref DAT_TYPE: DatType = entity_utils::entity_dat_type::<{{entry_struct_name}}>();
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

impl DatumDyn for {{entry_struct_name}} {
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
"#;
        let entry_struct_name = to_object_struct_name(table_def.table_name())?;
        let mut tera = Tera::default();
        tera.add_raw_template("template", s).unwrap();
        let mut context = tera::Context::new();
        context.insert("entry_struct_name", &entry_struct_name);
        let result = tera.render("template", &context).unwrap();
        writer.write_fmt(format_args!("{}", result)).map_err(write_error)?;
        Ok(())
    }

    fn impl_record_trait(&self, table_def: &TableDef, writer: &mut dyn Write) -> RS<()> {
        let table_name = table_def.table_name();
        let struct_obj_name = to_object_struct_name(table_name)?;
        // impl Entity for XXX {
        writer
            .write_fmt(format_args!("impl Entity for {struct_obj_name} {{\n"))
            .map_err(write_error)?;

        let mut constructor_empty = String::new();
        self.impl_object_fn_constructor(table_def, &mut constructor_empty, true)?;
        print_indent(&constructor_empty, 1, writer)?;

        let mut fn_tuple_desc = String::new();
        self.impl_record_fn_tuple_desc(table_def, &mut fn_tuple_desc)?;
        print_indent(&fn_tuple_desc, 1, writer)?;
        writer.write_str("\n").map_err(write_error)?;

        let mut fn_table_name = String::new();
        self.impl_record_fn_table_name(table_name, &mut fn_table_name)?;
        print_indent(&fn_table_name, 1, writer)?;
        writer.write_str("\n").map_err(write_error)?;

        let mut fn_get = String::new();
        self.impl_record_fn_get_binary(table_def, &mut fn_get)?;
        print_indent(&fn_get, 1, writer)?;

        writer.write_str("\n").map_err(write_error)?;

        let mut fn_set = String::new();
        self.impl_record_fn_set_binary(table_def, &mut fn_set)?;
        print_indent(&fn_set, 1, writer)?;

        let mut fn_get = String::new();
        self.impl_record_fn_get_value(table_def, &mut fn_get)?;
        print_indent(&fn_get, 1, writer)?;

        writer.write_str("\n").map_err(write_error)?;

        let mut fn_set = String::new();
        self.impl_record_fn_set_value(table_def, &mut fn_set)?;
        print_indent(&fn_set, 1, writer)?;
        
        // } end impl Entity for XXX {
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn impl_record_fn_tuple_desc(&self, table_def: &TableDef, writer: &mut dyn Write) -> RS<()> {
        writer
            .write_fmt(format_args!(
                "fn tuple_desc() -> &'static TupleFieldDesc {{\n"
            ))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("\tlazy_static! {{\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!(
                "\t\tstatic ref TUPLE_DESC:TupleFieldDesc = TupleFieldDesc::new(vec![\n"
            ))
            .map_err(write_error)?;
        for column_def in table_def.table_columns() {
            let attr_struct_name = to_attr_struct_name(column_def.column_name())?;
            writer
                .write_fmt(format_args!(
                    "\t\t\t{attr_struct_name}::datum_desc().clone(),\n"
                ))
                .map_err(write_error)?;
        }
        writer
            .write_fmt(format_args!("\t\t]);\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("\t}}\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("\t&TUPLE_DESC\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn impl_record_fn_table_name(&self, table_name: &String, writer: &mut dyn Write) -> RS<()> {
        let table_name_const = to_table_name_const(table_name);
        writer
            .write_fmt(format_args!("fn object_name() -> &'static str {{\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("\t{table_name_const}\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn impl_record_fn_get_binary(&self, table_def: &TableDef, writer: &mut dyn Write) -> RS<()> {
        writer
            .write_fmt(format_args!(
                "fn get_field_binary(&self, column:&str) -> RS<Option<Vec<u8>>> {{\n"
            ))
            .map_err(write_error)?;

        // let datum = match column {
        writer
            .write_fmt(format_args!("\tmatch column {{\n"))
            .map_err(write_error)?;
        for column in table_def.table_columns() {
            let name = column.column_name();
            let column_name_const = to_column_name_const(name);
            let field_name = to_struct_field_name(name)?;
            writer
                .write_fmt(format_args!("\t\t{column_name_const} => {{\n"))
                .map_err(write_error)?;

            writer
                .write_fmt(format_args!("\t\t\tattr_field_access::attr_get_binary::<_>(&self.{field_name})\n"))
                .map_err(write_error)?;

            writer
                .write_fmt(format_args!("\t\t}}\n"))
                .map_err(write_error)?;
        }

        writer
            .write_fmt(format_args!("\t\t_ => {{ panic!(\"unknown name\"); }}\n"))
            .map_err(write_error)?;

        // }; END match column {
        writer
            .write_fmt(format_args!("\t}}\n"))
            .map_err(write_error)?;

        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn impl_record_fn_get_value(&self, table_def: &TableDef, writer: &mut dyn Write) -> RS<()> {
        writer
            .write_fmt(format_args!(
                "fn get_field_value(&self, column:&str) -> RS<Option<DatValue>> {{\n"
            ))
            .map_err(write_error)?;

        // let datum = match column {
        writer
            .write_fmt(format_args!("\tmatch column {{\n"))
            .map_err(write_error)?;
        for column in table_def.table_columns() {
            let name = column.column_name();
            let column_name_const = to_column_name_const(name);
            let field_name = to_struct_field_name(name)?;
            writer
                .write_fmt(format_args!("\t\t{column_name_const} => {{\n"))
                .map_err(write_error)?;

            writer
                .write_fmt(format_args!("\t\t\tattr_field_access::attr_get_value::<_>(&self.{field_name})\n"))
                .map_err(write_error)?;

            writer
                .write_fmt(format_args!("\t\t}}\n"))
                .map_err(write_error)?;
        }

        writer
            .write_fmt(format_args!("\t\t_ => {{ panic!(\"unknown name\"); }}\n"))
            .map_err(write_error)?;

        // }; END match column {
        writer
            .write_fmt(format_args!("\t}}\n"))
            .map_err(write_error)?;

        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }
    fn impl_record_fn_set_binary(&self, table_def: &TableDef, writer: &mut dyn Write) -> RS<()> {
        writer
            .write_fmt(format_args!(
                "fn set_field_binary<B:AsRef<[u8]>>(&mut self, column:&str, binary:B) -> RS<()> {{\n"
            ))
            .map_err(write_error)?;
        // match column {{
        writer
            .write_fmt(format_args!("\tmatch column {{\n"))
            .map_err(write_error)?;
        for column in table_def.table_columns() {
            let name = column.column_name();
            let column_name_const = to_column_name_const(name);
            let field_name = to_struct_field_name(name)?;
            writer
                .write_fmt(format_args!("\t\t{column_name_const} => {{\n"))
                .map_err(write_error)?;
            writer
                .write_fmt(format_args!(
                    "\t\t\tattr_field_access::attr_set_binary::<_, _>(&mut self.{field_name}, binary.as_ref())?;\n"
                ))
                .map_err(write_error)?;

            writer
                .write_fmt(format_args!("\t\t}}\n"))
                .map_err(write_error)?;
        }

        writer
            .write_fmt(format_args!("\t\t_ => {{ panic!(\"unknown name\"); }}\n"))
            .map_err(write_error)?;

        // } END match column {{
        writer
            .write_fmt(format_args!("\t}}\n"))
            .map_err(write_error)?;

        writer
            .write_fmt(format_args!("\tOk(())\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn impl_record_fn_set_value(&self, table_def: &TableDef, writer: &mut dyn Write) -> RS<()> {
        writer
            .write_fmt(format_args!(
                "fn set_field_value<B:AsRef<DatValue>>(&mut self, column:&str, value:B) -> RS<()> {{\n"
            ))
            .map_err(write_error)?;
        // match column {{
        writer
            .write_fmt(format_args!("\tmatch column {{\n"))
            .map_err(write_error)?;
        for column in table_def.table_columns() {
            let name = column.column_name();
            let column_name_const = to_column_name_const(name);
            let field_name = to_struct_field_name(name)?;
            writer
                .write_fmt(format_args!("\t\t{column_name_const} => {{\n"))
                .map_err(write_error)?;
            writer
                .write_fmt(format_args!(
                    "\t\t\tattr_field_access::attr_set_value::<_, _>(&mut self.{field_name}, value)?;\n"
                ))
                .map_err(write_error)?;

            writer
                .write_fmt(format_args!("\t\t}}\n"))
                .map_err(write_error)?;
        }

        writer
            .write_fmt(format_args!("\t\t_ => {{ panic!(\"unknown name\"); }}\n"))
            .map_err(write_error)?;

        // } END match column {{
        writer
            .write_fmt(format_args!("\t}}\n"))
            .map_err(write_error)?;

        writer
            .write_fmt(format_args!("\tOk(())\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn impl_object_fn_constructor(
        &self,
        table_def: &TableDef,
        writer: &mut dyn Write,
        is_empty: bool,
    ) -> RS<()> {
        let pub_str = if !is_empty {
            "pub "
        } else {
            // impl Entity trait
            ""
        };
        let constructor_name = if is_empty { "new_empty" } else { "new" };
        writer
            .write_fmt(format_args!("{pub_str}fn {constructor_name}(\n"))
            .map_err(write_error)?;
        if !is_empty {
            for column_def in table_def.table_columns() {
                let field_name = to_struct_field_name(column_def.column_name())?;
                let data_type = to_data_type_str(column_def.dat_type().dat_type_id())?;
                writer
                    .write_fmt(format_args!("\t{field_name}:Option<{data_type}>,\n"))
                    .map_err(write_error)?;
            }
        }
        writer
            .write_fmt(format_args!(") -> Self {{\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("\tlet s = Self {{\n"))
            .map_err(write_error)?;
        if is_empty {
            for column_def in table_def.table_columns() {
                let field_name = to_struct_field_name(column_def.column_name())?;
                writer
                    .write_fmt(format_args!("\t\t{field_name}:None,\n"))
                    .map_err(write_error)?;
            }

            writer
                .write_fmt(format_args!("\t}};\n"))
                .map_err(write_error)?;
        } else {
            for column_def in table_def.table_columns() {
                let field_name = to_struct_field_name(column_def.column_name())?;
                writer
                    .write_fmt(format_args!("\t\t{field_name},\n"))
                    .map_err(write_error)?;
            }

            writer
                .write_fmt(format_args!("\t}};\n"))
                .map_err(write_error)?;
        }

        writer
            .write_fmt(format_args!("\ts\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn build_field(&self, column_def: &TableColumnDef, writer: &mut dyn Write) -> RS<()> {
        let field_name = to_struct_field_name(column_def.column_name())?;
        let data_type = to_data_type_str(column_def.dat_type().dat_type_id())?;
        writer
            .write_fmt(format_args!("{field_name} : Option<{data_type}>,\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn build_setter(&self, column_def: &TableColumnDef, writer: &mut dyn Write) -> RS<()> {
        let field_name = to_struct_field_name(column_def.column_name())?;
        let data_type = to_data_type_str(column_def.dat_type().dat_type_id())?;
        writer
            .write_fmt(format_args!("pub fn set_{field_name}(\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("\t& mut self,\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("\t{field_name} : {data_type},\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("){{\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("\tself.{field_name} = Some({field_name});\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn build_getter(&self, column_def: &TableColumnDef, writer: &mut dyn Write) -> RS<()> {
        let field_name = to_struct_field_name(column_def.column_name())?;
        let data_type = to_data_type_str(column_def.dat_type().dat_type_id())?;
        writer
            .write_fmt(format_args!("pub fn get_{field_name}(\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("\t& self,\n"))
            .map_err(write_error)?;
        let is_basic_type = is_basic_type(column_def.data_type());

        if is_basic_type {
            writer
                .write_fmt(format_args!(") -> & Option<{data_type}> {{\n"))
                .map_err(write_error)?;
            writer
                .write_fmt(format_args!("\t& self.{field_name}\n"))
                .map_err(write_error)?;
        } else {
            writer
                .write_fmt(format_args!("\t) -> & Option<{data_type}> {{\n"))
                .map_err(write_error)?;
            writer
                .write_fmt(format_args!("\t & self.{field_name}\n"))
                .map_err(write_error)?;
        }
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn struct_impl_attribute(
        &self,
        table_name: &String,
        column_def: &TableColumnDef,
        writer: &mut dyn Write,
    ) -> RS<()> {
        let column_name = column_def.column_name();
        let attr_obj_name = to_attr_struct_name(column_name)?;
        let data_type = to_data_type_str(column_def.dat_type().dat_type_id())?;
        // pub struct AttrXXX {
        writer
            .write_fmt(format_args!("pub struct {attr_obj_name} {{\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        writer.write_fmt(format_args!("\n")).map_err(write_error)?;
        // }


        // impl AttrValue trait
        writer
            .write_fmt(format_args!(
                "impl AttrValue<{data_type}> for {attr_obj_name} {{\n"
            ))
            .map_err(write_error)?;
        let mut attr_trait_fn = String::new();
        self.build_attr_trait_fn(table_name, column_def, &mut attr_trait_fn)?;
        print_indent(&attr_trait_fn, 1, writer)?;
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;

        Ok(())
    }

    fn build_attr_trait_fn(
        &self,
        table_name: &String,
        column_def: &TableColumnDef,
        writer: &mut dyn Write,
    ) -> RS<()> {
        let data_type = to_data_type_str(column_def.dat_type().dat_type_id())?;
        let column_name = column_def.column_name();
        writer.write_str("\n").map_err(write_error)?;
        self.build_attr_fn_dat_type(&data_type, writer)?;
        writer.write_str("\n").map_err(write_error)?;
        self.build_attr_fn_datum_desc(column_name, &data_type, writer)?;
        writer.write_str("\n").map_err(write_error)?;
        self.build_attr_fn_table_name(table_name, writer)?;
        writer.write_str("\n").map_err(write_error)?;

        self.build_attr_fn_column_name(column_name, writer)?;
        writer.write_str("\n").map_err(write_error)?;

        Ok(())
    }

    fn build_attr_fn_dat_type(&self, _data_type_str: &String, writer: &mut dyn Write) -> RS<()> {
        writer
            .write_fmt(format_args!("fn dat_type() -> &'static DatType {{\n"))
            .map_err(write_error)?;

        writer
            .write_fmt(format_args!(
                "\t\tstatic ONCE_LOCK:std::sync::OnceLock<DatType> = std::sync::OnceLock::new();\n"))
            .map_err(write_error)?;
        writer.write_fmt(format_args!(" ONCE_LOCK.get_or_init(||{{ Self::attr_dat_type() }})\n"
        ))
            .map_err(write_error)?;

        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn build_attr_fn_datum_desc(&self, _column_name: &String, _data_type_str: &String, writer: &mut dyn Write) -> RS<()> {
        writer
            .write_fmt(format_args!("fn datum_desc() -> &'static DatumDesc {{\n"))
            .map_err(write_error)?;


        writer
            .write_fmt(format_args!(
                "\t\tstatic ONCE_LOCK:std::sync::OnceLock<DatumDesc> = std::sync::OnceLock::new();\n"))
            .map_err(write_error)?;
        writer.write_fmt(format_args!(" ONCE_LOCK.get_or_init(||{{ Self::attr_datum_desc() }})\n"
        ))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }


    fn build_attr_fn_table_name(&self, table_name: &String, writer: &mut dyn Write) -> RS<()> {
        let table_name_const = to_table_name_const(table_name);
        writer
            .write_fmt(format_args!("fn object_name() -> &'static str {{\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("\t{table_name_const}\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }

    fn build_attr_fn_column_name(&self, column_name: &String, writer: &mut dyn Write) -> RS<()> {
        let column_name_const = to_column_name_const(column_name);
        writer
            .write_fmt(format_args!("fn attr_name() -> &'static str {{\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("\t{column_name_const}\n"))
            .map_err(write_error)?;
        writer
            .write_fmt(format_args!("}}\n"))
            .map_err(write_error)?;
        Ok(())
    }
}

impl SrcBuilder for RustBuilder {
    fn build(&self, table_def: &TableDef, writer: &mut dyn Write) -> RS<()> {
        self.build_object(table_def, writer)?;
        Ok(())
    }
}
