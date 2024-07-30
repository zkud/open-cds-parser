mod implementation;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(cds, "/parser/parser/single_module/cds.rs");
#[cfg(test)]
mod tests;