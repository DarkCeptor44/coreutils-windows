const NAME: &str = env!("CARGO_BIN_NAME");

use anyhow::Result;
use clap::Parser;
use colored::{Color, Colorize};
use rayon::ThreadPoolBuilder;
use std::{
    num::NonZero,
    path::{MAIN_SEPARATOR, Path},
    process::exit,
    thread::available_parallelism,
};
use which::which;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct App {
    #[arg(help = "The search pattern")]
    pattern: String,

    #[arg(
        short,
        long,
        help = "Color to use for highlighting (accepts hexcode as well like `#ff0000`)",
        default_value = "blue"
    )]
    color: String,

    #[arg(short, long, help = "Number of threads to use", default_value_t = num_threads())]
    jobs: usize,
}

fn main() {
    if let Err(e) = main_impl() {
        eprintln!("{}", format!("{NAME}: {e:?}").red());
        exit(1);
    }
}

fn main_impl() -> Result<()> {
    let args = App::parse();
    let App {
        pattern,
        jobs,
        color: color_str,
    } = args;
    let color = Color::from(color_str);

    ThreadPoolBuilder::new().num_threads(jobs).build_global()?;

    let paths = which(&pattern)?;
    for path in paths {
        println!("{}", highlight_text(&path, &pattern, color, None));
    }

    Ok(())
}

fn highlight_text(path: &Path, pattern: &str, color: Color, separator: Option<char>) -> String {
    let sep = separator.unwrap_or(MAIN_SEPARATOR);
    let path_str = path.to_string_lossy();
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return path_str.to_string();
    };
    let stem = match path.parent() {
        Some(p) => p.to_string_lossy().to_string(),
        None => String::new(),
    };

    let pattern_lower = pattern.to_lowercase();
    let name_lower = name.to_lowercase();

    if let Some(index) = name_lower.find(&pattern_lower) {
        format!(
            "{stem}{}{}{}{}",
            if stem.is_empty() || stem.ends_with(sep) {
                String::new()
            } else {
                sep.to_string()
            },
            &name[..index],
            name[index..index + pattern.len()].color(color).bold(),
            &name[index + pattern.len()..]
        )
    } else {
        path_str.to_string()
    }
}

fn num_threads() -> usize {
    let cpus = available_parallelism().map_or(1, NonZero::get);
    std::cmp::max(1, (cpus * 3).div_ceil(4))
}
