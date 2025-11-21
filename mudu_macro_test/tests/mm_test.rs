#[cfg(test)]
mod tests {
    use mudu::common::result::RS;
    use mudu::common::serde_utils::{deserialize_sized_from, serialize_sized_to_vec};
    use mudu::common::xid::XID;
    use mudu::procedure::proc_param::ProcParam;
    use mudu::procedure::proc_result::ProcResult;
    use mudu::tuple::rs_tuple_datum::RsTupleDatum;
    use mudu_macro::mudu_proc;


    #[mudu_proc]
    pub fn example(xid: XID, a: i32, b: i64, c: String) -> RS<(i64, String)> {
        Ok((a as i64 + b + 1, format!("c={}, {} function invoked", c, xid)))
    }

    #[mudu_proc]
    pub fn test_proc2(xid: XID, a: i32) -> RS<String> {
        Ok(format!("a {},  {} function invoked", a,  xid))
    }

    #[mudu_proc]
    pub fn example_vec(xid: XID, a: i32, b: Vec<i64>) -> RS<(Vec<i64>, String)> {
        Ok((vec![a as i64, (a + 1) as i64], format!("a {}, b:{:?}, {} function invoked", a, b, xid)))
    }

    #[test]
    fn test_mudu_macro2() {
        let param = ProcParam::from_tuple(
            1,
            (1i32, 3i64, "string".to_string()),
            &<(i32, i64, String)>::tuple_desc_static(
                &["p1".to_string(), "p2".to_string(), "p3".to_string()]
            )).unwrap();
        let result = mudu_inner_example(param);
        println!("result {:?}", result)
    }

    #[test]
    fn test_mudu_macro_macro() {
        let proc_desc = mudu_proc_desc_example();
        let argv = ProcParam::from_datum_vec(0, &[&32i32, &64i64, &"s".to_string()], proc_desc.param_desc()).unwrap();
        let mut output_buf = vec![0u8; 10240usize];
        let input_buf = serialize_sized_to_vec(&argv).unwrap();
        let r = mudu_example(input_buf.as_ptr(), input_buf.len(), output_buf.as_mut_ptr(), output_buf.len());
        if r == 0 {
            let (result, _n): (ProcResult, _) = deserialize_sized_from(&output_buf).unwrap();
            let r = result.to::<(i64, String)>(proc_desc.return_desc()).unwrap();
            println!("result {:?}", r);
        }

        println!("Test passed!");
    }
}