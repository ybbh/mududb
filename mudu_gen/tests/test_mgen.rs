
#[cfg(test)]
mod tests {
    use mudu_gen::src_gen::ddl_parser::DDLParser;
    use mudu_gen::src_gen::src_gen::{Language, SrcGen};
    use mudu::common::result::RS;
    use mudu::error::ec::EC;
    use mudu::{m_error, this_file};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    #[test]
    fn test_mudul_src_gen() {
        let r = test_src_file_gen();
        if r.is_err() {
            let e = r.as_ref().err().unwrap();
            println!("test error : {}", e);
        }
        assert!(r.is_ok());
    }

    fn test_src_file_gen() -> RS<()> {
        let example_proj_path = PathBuf::from(this_file!())
            .parent().unwrap().parent().unwrap().parent().unwrap()
            .join("example");
        for (sql_file, out_dir) in vec![
            ("wallet/src/sql/ddl.sql", "wallet/src/rust"),
            ("vote/src/sql/ddl.sql", "vote/src/rust"),
            ("vote/src/sql/type.sql", "vote/src/rust"),
        ] {
            let input_sql_path = example_proj_path.join(PathBuf::from(sql_file));
            let output_dir_path = example_proj_path.join(PathBuf::from(out_dir));
            test_src_gen_gut(&input_sql_path, &output_dir_path)?;
        }
        Ok(())
    }

    fn test_src_gen_gut<P1:AsRef<Path>, P2:AsRef<Path>>(input_sql_path:P1, output_dir_path:P2) -> RS<()> {
        let parser = DDLParser::new();
        let text = fs::read_to_string(&input_sql_path).unwrap();
        let vec = parser.parse(&text)?;
        let src_gen = SrcGen::new();
        for table_def in vec.iter() {
            let text_src = src_gen.generate(Language::Rust, table_def)?;
            let file_name = format!("{}.rs", table_def.table_name());
            let path_out = PathBuf::from(output_dir_path.as_ref()).join(file_name);
            write_to_file(&text_src, &path_out)?;
            let output = Command::new("rustc")
                .arg("--emit=metadata")
                .arg("--crate-type=lib") // crate-type=lib, no main
                .arg("--edition=2024") //   edition
                .arg(&path_out)
                .output()
                .map_err(|e| m_error!(EC::IOErr, "build command line", e))?;
            if output.status.success() {
                println!("compile {} OK", path_out.to_str().unwrap());
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("syntax error:\n{}", stderr);
            }
        }

        std::env::set_current_dir(&output_dir_path).unwrap();

        let output = Command::new("cargo")
            .arg("fmt")
            .output()
            .map_err(|e| m_error!(EC::IOErr, "cargo fmt error", e))?;
        if output.status.success() {
            println!("cargo fmt OK");
        }

        let output = Command::new("cargo")
            .arg("build")
            .output()
            .map_err(|e| m_error!(EC::IOErr, "cargo build error", e))?;
        if output.status.success() {
            println!("cargo build OK");
        }
        Ok(())
    }

    fn write_to_file<P:AsRef<Path>>(content: &String, file_path: P) -> RS<()> {
        fs::write(file_path.as_ref(), content)
            .map_err(|e| m_error!(EC::IOErr, format!("write file {:?} error", file_path.as_ref()), e))?;
        Ok(())
    }
}
