use gen::Generator;
use getopts::Options;
use std::env;

mod gen;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("i", "input", "set input file name", "");
    opts.optopt("I", "header-dir", "set the directory to save headers", "");
    opts.optopt(
        "O",
        "source-dir",
        "set the directory to save source files",
        "",
    );
    opts.optopt("n", "namespace", "specify targeting namespace", "");
    opts.optflag("h", "help", "print this message");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let input = matches.opt_str("i");
    let header_dir = matches.opt_str("I");
    let source_dir = matches.opt_str("O");
    let namespace = matches.opt_str("n");

    if !input.is_some() {
        panic!("no input file");
    }

    if !header_dir.is_some() {
        panic!("no header dir");
    }

    if !source_dir.is_some() {
        panic!("no source dir");
    }

    let code = {
        let file = std::fs::read_to_string(input.unwrap());

        if let Err(err) = &file {
            panic!("failed to open input file: {}", err);
        }

        file.unwrap()
    };

    let mut builder = Generator::builder_with(&code)
        .save_headers_at(&header_dir.unwrap())
        .save_sources_at(&source_dir.unwrap());

    if let Some(namespace) = namespace {
        builder = builder.with_namespace(&namespace);
    }

    let generator = builder.build();

    if let Err(err) = &generator {
        panic!("failed to create generator: {}", err);
    }

    let generator = generator.unwrap();

    if let Err(err) = generator.generate() {
        panic!("failed to generate code: {}", err);
    }
}
