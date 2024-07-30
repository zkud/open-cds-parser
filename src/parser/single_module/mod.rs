mod implementation;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(cds, "/parser/single_module/cds.rs");
mod interface;
#[cfg(test)]
mod tests;

pub use implementation::SingleModuleParserImpl;
pub use interface::SingleModuleParser;
