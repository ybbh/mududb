## Key Traits

### SQLStmt

<!--
quote_begin
content="[DatumDyn](../../mudu/src/database/sql_stmt.rs#L3-L8)"
lang="rust"
-->
```rust
pub trait SQLStmt: fmt::Debug + fmt::Display + Sync + Send {
    fn to_sql_string(&self) -> String;

    fn clone_boxed(&self) -> Box<dyn SQLStmt>;
}
```
<!--quote_end-->

### DatumDyn

<!--
quote_begin
content="[DatumDyn](../../mudu/src/tuple/datum.rs#L23-L36)"
lang="rust"
-->
```rust
pub trait DatumDyn: fmt::Debug + Send + Sync + Any {
    fn dat_type_id_self(&self) -> RS<DatTypeID>;

    fn to_typed(&self, param: &ParamObj) -> RS<DatTyped>;

    fn to_binary(&self, param: &ParamObj) -> RS<DatBinary>;

    fn to_printable(&self, param: &ParamObj) -> RS<DatPrintable>;

    fn to_internal(&self, param: &ParamObj) -> RS<DatInternal>;

    fn clone_boxed(&self) -> Box<dyn DatumDyn>;
}
```
<!--quote_end-->
