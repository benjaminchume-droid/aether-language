use crate::{ast,hir,lexer,lower,parser,typecheck::{TypeChecker,TypedProgram}};
use std::{collections::HashSet,fs,path::{Path,PathBuf}};

pub fn compile(source:&str)->Result<TypedProgram,Vec<String>>{let tokens=lexer::lex(source).map_err(|e|vec![e])?;let ast=parser::parse(&tokens).map_err(|e|vec![e])?;if ast.items.iter().any(|i|matches!(i,ast::Item::Import(_))){return Err(vec!["imports require compile_file/run_file so relative paths can be resolved".into()])}TypeChecker::check(&ast)}
pub fn compile_file(path:impl AsRef<Path>)->Result<TypedProgram,Vec<String>>{let ast=load_file(path).map_err(|e|vec![e])?;TypeChecker::check(&ast)}
pub fn lower_to_hir(source:&str)->Result<hir::Program,String>{let tokens=lexer::lex(source)?;let ast=parser::parse(&tokens)?;Ok(lower::lower(&ast))}
pub fn lower_file(path:impl AsRef<Path>)->Result<hir::Program,String>{Ok(lower::lower(&load_file(path)?))}
pub fn load_file(path:impl AsRef<Path>)->Result<ast::Program,String>{let canonical=path.as_ref().canonicalize().map_err(|e|format!("cannot open {}: {e}",path.as_ref().display()))?;let mut loader=Loader{active:HashSet::new(),loaded:HashSet::new()};loader.load(canonical)}
struct Loader{active:HashSet<PathBuf>,loaded:HashSet<PathBuf>}
impl Loader{fn load(&mut self,path:PathBuf)->Result<ast::Program,String>{if self.active.contains(&path){return Err(format!("cyclic import detected at {}",path.display()))}if self.loaded.contains(&path){return Ok(ast::Program{items:Vec::new()})}self.active.insert(path.clone());let source=fs::read_to_string(&path).map_err(|e|format!("cannot read {}: {e}",path.display()))?;let tokens=lexer::lex(&source).map_err(|e|format!("{}: {e}",path.display()))?;let parsed=parser::parse(&tokens).map_err(|e|format!("{}: {e}",path.display()))?;let base=path.parent().unwrap_or(Path::new("."));let mut items=Vec::new();for item in parsed.items{match item{ast::Item::Import(spec)=>{let child=resolve_import(base,&spec);items.extend(self.load(child)?.items)},other=>items.push(other)}}self.active.remove(&path);self.loaded.insert(path);Ok(ast::Program{items})}}
fn resolve_import(base:&Path,spec:&str)->PathBuf{let mut p=base.join(spec);if p.extension().is_none(){p.set_extension("ae")}p}
