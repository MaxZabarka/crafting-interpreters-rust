use crate::chunk::Chunk;

struct VM {
    chunk: &Chunk,
}

enum InterpretError {
    COMPILE_ERROR,
    RUNTIME_ERROR,
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InterpretError::CompileError => write!(f, "Compilation error occurred"),
            InterpretError::RuntimeError => write!(f, "Runtime error occurred"),
        }
    }
}

impl std::error::Error for InterpretError {}

impl VM {
    fn interpret(&self) -> Result {}
}
  
