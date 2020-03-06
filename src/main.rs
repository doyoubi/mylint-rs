extern crate env_logger;
use ansi_term::Colour::{Cyan, Green, Red};
use mylint::{default_filter, AllRulesValidator, SourceCode, ValidationError, Validator, RULES};
use scan_dir::ScanDir;
use std::fs;
use structopt::StructOpt;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

#[derive(Debug, StructOpt)]
#[structopt(name = "mylint", about = "Customized rust linter.")]
struct Opt {
    #[structopt(short, long, default_value = "./src")]
    path: String,

    #[structopt(short, long)]
    list: bool,

    #[structopt(short, long)]
    help: bool,

    #[structopt(short, long)]
    suppress: Vec<String>,
}

struct LintErr {
    filepath: String,
    source_code: String,
    err: ValidationError,
}

fn print_err_lines(errs: Vec<LintErr>) -> Result<(), String> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps
        .find_syntax_by_extension("rs")
        .ok_or_else(|| "failed to load rust syntax".to_string())?;
    let mut h = HighlightLines::new(
        syntax,
        ts.themes
            .get("base16-ocean.dark")
            .ok_or_else(|| "failed to get theme".to_string())?,
    );
    for LintErr {
        filepath,
        source_code,
        err,
    } in errs.into_iter()
    {
        for (i, line) in LinesWithEndings::from(&source_code).enumerate() {
            if i < err.code_range.start.row || i > err.code_range.end.row {
                continue;
            }
            let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
            let escaped = as_24_bit_terminal_escaped(ranges.as_slice(), false);
            println!(
                "{}: {}:{} - {}:{}",
                Green.paint(filepath.clone()),
                Cyan.paint(err.code_range.start.row.to_string()),
                Cyan.paint(err.code_range.start.column.to_string()),
                Cyan.paint(err.code_range.end.row.to_string()),
                Cyan.paint(err.code_range.end.column.to_string()),
            );
            println!("{} {}", Red.paint("ERROR:"), Red.paint(err.rule.desc));
            println!("{} | {}", format!("{:>6}", i), escaped);

            if let Some(hint) = err.rule.hint {
                for hint_line in hint.trim_start().lines() {
                    let ranges: Vec<(Style, &str)> = h.highlight(hint_line, &ps);
                    let escaped = as_24_bit_terminal_escaped(ranges.as_slice(), false);
                    println!("{}", escaped);
                }
            }
            println!();
        }
    }
    Ok(())
}

fn print_rules() {
    for rule in &RULES {
        println!(
            "{}: {}",
            Red.paint(rule.code.to_string()),
            Red.paint(rule.desc)
        );
    }
}

fn print_help() {
    println!("{}", Green.paint("Run linter:"));
    println!("\t{}", Green.paint("mylint -p <directoy>"));
    println!("{}", Green.paint("List all rules:"));
    println!("\t{}", Green.paint("mylint -l"));
    println!("{}", Green.paint("Suppress rules:"));
    println!("\t{}", Green.paint("mylint -s <rules>"));
}

fn main() -> Result<(), String> {
    env_logger::init();
    let opt = Opt::from_args();

    if opt.help {
        print_help();
        return Ok(());
    }

    if opt.list {
        print_rules();
        return Ok(());
    }

    let suppress = opt.suppress.clone();
    if !suppress.is_empty() {
        println!("{}", Red.paint(format!("Suppressing: {:#?}", suppress)));
    }

    let mut paths = vec![];
    ScanDir::files()
        .walk(opt.path.clone(), |iter| {
            for (entry, name) in iter {
                if !name.ends_with(".rs") {
                    continue;
                }
                paths.push(entry.path());
            }
        })
        .map_err(|err| format!("failed to open path {}: {:#?}", opt.path.as_str(), err))?;

    if paths.is_empty() {
        return Err("failed to find any files".to_string());
    }

    for p in &paths {
        println!("{}", p.to_string_lossy());
    }

    let validator = AllRulesValidator::new(suppress, default_filter());
    let mut errs = vec![];

    for path in paths.into_iter() {
        let path_str = path.to_string_lossy().to_owned();
        let source_code = fs::read_to_string(path.clone())
            .map_err(|err| format!("failed to open file {}: {}", path_str, err))?;

        let source = SourceCode::parse(&source_code)
            .ok_or_else(|| format!("failed to parse source code: {}", path_str))?;
        let root_node = source.get_root_node();
        let res = validator.validate(&root_node, &source_code);

        if let Err(err) = res {
            errs.push(LintErr {
                filepath: path_str.to_string(),
                source_code: source_code.to_string(),
                err,
            });
        }
    }

    if errs.is_empty() {
        Ok(())
    } else {
        print_err_lines(errs)?;
        Err("Not all lint passed".to_string())
    }
}
