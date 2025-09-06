use crate::module_importer::ant_module_importer::AntModuleImporter;

pub enum Importer {
    AntModuleImporter(AntModuleImporter),
    NativeModuleImporter(),
}

pub struct ModuleImporter {
    pub imports: Vec<String>,
    pub importer: Importer,
}

