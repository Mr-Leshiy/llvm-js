pub mod js_ast;
pub mod llvm_ast;

pub type LexerError = lexer::Error;

type Precompiler = precompiler::Precompiler<js_ast::Identifier, llvm_ast::FunctionDeclaration>;
pub type PrecompilerError = precompiler::Error<js_ast::Identifier>;

type Compiler<'ctx> = compiler::Compiler<'ctx, llvm_ast::Identifier>;
type Function<'ctx> = compiler::Function<'ctx, llvm_ast::Identifier>;
pub type CompilerError = compiler::Error<llvm_ast::Identifier>;
