
#[test]
fn test_import_module() {
    use crate::module_system::import_module::ModuleImporter;
    use crate::utils::assert_eq;

    let mut module_importer = ModuleImporter {
        module_name: "test_module".to_string(),
        module_paths: vec!["./src/module_system/".to_string()],
    };

    let result = module_importer.import();
    assert_eq(
        result.is_ok(), 
        true, 
        || {
            println!("module import failed: {:?}", result.clone().err()); println!("{:?}", module_importer.module_paths)
        }
    );

    println!("module imported successfully");
}