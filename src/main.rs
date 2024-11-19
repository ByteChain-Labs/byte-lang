mod compiler;
mod utils;
use compiler::{ bcgen, bingen, bytec };
use utils::{ parse_args, parse_source_file };

fn main() {
    let args = parse_args();
    let ast = parse_source_file(args.source_file);

    match args.output_flag.as_str() {
        "-bc" => {
            let bytecode = bcgen::generate_bytecode(ast);
            write_to_file(bytecode, "output.byc");
        }
        "-bin" => {
            let binary = bingen::generate_binary(ast);
            write_to_file(binary, "output.bin");
        }
    }
}