use std::fs;

use shaderc::{Compiler, ShaderKind};

trait SimpleCompiler {
    fn compile_to_spirv(&mut self, src_file: &str, kind: ShaderKind);
}

impl SimpleCompiler for Compiler {
    fn compile_to_spirv(&mut self, src_file: &str, kind: ShaderKind) {
        let src = fs::read_to_string(src_file).unwrap();
        let spirv = self.compile_into_spirv(&src, kind, src_file, "main",
                                            None).unwrap();
        fs::write(format!("{}.spv", src_file), spirv.as_binary_u8()).unwrap();
    }
}

fn main() {
    let shaders = &[("src/bin/gui/graphics/blit/shader.vert", ShaderKind::Vertex),
                    ("src/bin/gui/graphics/blit/shader.frag", ShaderKind::Fragment)];
    let mut compiler = Compiler::new().unwrap();
    for (shader_file, kind) in shaders {
        compiler.compile_to_spirv(shader_file, *kind);
        println!("cargo:rerun-if-changed={}", shader_file);
    }
}
