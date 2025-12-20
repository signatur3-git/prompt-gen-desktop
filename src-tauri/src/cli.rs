// M6 Phase 2: CLI Tool for Package Operations
// Beautiful terminal interface with helpful output

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;
use std::process;

mod core;
mod parser;
mod validator;
mod renderer;
mod context;
mod rules;

use parser::{load_package, DependencyResolver};
use validator::{PackageValidator, ValidationError, ValidationWarning};

#[derive(Parser)]
#[command(name = "rpg-cli")]
#[command(version = "0.1.0")]
#[command(about = "Random Prompt Generator CLI - Package validation and rendering", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a package file
    Validate {
        /// Path to the package file (YAML or JSON)
        #[arg(value_name = "FILE")]
        path: PathBuf,

        /// Show warnings as well as errors
        #[arg(short, long)]
        warnings: bool,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Display package information
    Info {
        /// Path to the package file (YAML or JSON)
        #[arg(value_name = "FILE")]
        path: PathBuf,
    },

    /// Render a prompt section
    Render {
        /// Path to the package file (YAML or JSON)
        #[arg(value_name = "FILE")]
        path: PathBuf,

        /// Prompt section to render (format: namespace:section)
        #[arg(value_name = "SECTION")]
        section: String,

        /// Seed for deterministic rendering
        #[arg(short, long, default_value = "42")]
        seed: u64,

        /// Number of renders to generate
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Validate { path, warnings, verbose } => {
            validate_command(path, warnings, verbose)
        }
        Commands::Info { path } => {
            info_command(path)
        }
        Commands::Render { path, section, seed, count } => {
            render_command(path, section, seed, count)
        }
    };

    if let Err(exit_code) = result {
        process::exit(exit_code);
    }
}

/// Validate command - check package for errors and warnings
fn validate_command(path: PathBuf, show_warnings: bool, verbose: bool) -> Result<(), i32> {
    // Header
    println!("{}", "=".repeat(60).bright_blue());
    println!("{} {}", "Validating:".bright_cyan().bold(), path.display());
    println!("{}", "=".repeat(60).bright_blue());
    println!();

    // M9 Phase 2.7: Load package with dependencies
    if verbose {
        println!("{} Loading package with dependencies...", "→".bright_blue());
    }

    // Set up search paths for dependencies
    let search_paths = vec![
        PathBuf::from("./packages"),
        PathBuf::from("./test-packages"),
        PathBuf::from("../packages"),
    ];

    let mut resolver = DependencyResolver::new(search_paths);

    let (package, dependencies) = match resolver.load_package_with_deps(&path) {
        Ok((pkg, deps)) => {
            if verbose {
                println!("{} Package loaded successfully", "✓".green());
                if !deps.is_empty() {
                    println!("  {} Dependencies loaded: {}", "→".bright_blue(), deps.len());
                    for (dep_id, dep_pkg) in &deps {
                        println!("    - {} v{}", dep_id.bright_yellow(), dep_pkg.version.bright_black());
                    }
                }
            }
            (pkg, deps)
        }
        Err(e) => {
            println!("{} {}", "✗".red().bold(), "Failed to load package".red());
            println!("  {}", format!("{}", e).bright_red());
            return Err(1);
        }
    };

    // Run validation with dependencies
    if verbose {
        println!("{} Running validation...", "→".bright_blue());
    }

    let result = PackageValidator::validate_with_dependencies(&package, &dependencies);

    println!();

    // Display results
    if result.is_valid() {
        // Success!
        println!("{}", "✓ VALIDATION PASSED".green().bold());
        println!();

        if show_warnings && result.has_warnings() {
            display_warnings(&result.warnings, verbose);
        }

        // Summary
        println!("{}", "─".repeat(60).bright_black());
        println!("{} {}", "Result:".bright_cyan(), "VALID".green().bold());

        if result.has_warnings() {
            println!("{} {}", "Warnings:".yellow(), result.warnings.len());
        } else {
            println!("{} {}", "Warnings:".bright_black(), "0".bright_black());
        }

        println!("{}", "─".repeat(60).bright_black());

        Ok(())
    } else {
        // Failed validation
        println!("{}", "✗ VALIDATION FAILED".red().bold());
        println!();

        display_errors(&result.errors, verbose);

        if show_warnings && result.has_warnings() {
            println!();
            display_warnings(&result.warnings, verbose);
        }

        // Summary
        println!();
        println!("{}", "─".repeat(60).bright_black());
        println!("{} {}", "Result:".bright_cyan(), "INVALID".red().bold());
        println!("{} {}", "Errors:".red(), result.errors.len());

        if result.has_warnings() {
            println!("{} {}", "Warnings:".yellow(), result.warnings.len());
        }

        println!("{}", "─".repeat(60).bright_black());

        Err(1)
    }
}

/// Display validation errors
fn display_errors(errors: &[ValidationError], verbose: bool) {
    println!("{} ({})", "Errors".red().bold(), errors.len());
    println!();

    for (i, error) in errors.iter().enumerate() {
        println!("  {}. {}", (i + 1).to_string().red(), format!("{}", error).bright_red());

        if verbose {
            // Could add more details here in verbose mode
        }

        println!();
    }
}

/// Display validation warnings
fn display_warnings(warnings: &[ValidationWarning], verbose: bool) {
    println!("{} ({})", "Warnings".yellow().bold(), warnings.len());
    println!();

    for (i, warning) in warnings.iter().enumerate() {
        println!("  {}. {}", (i + 1).to_string().yellow(), format!("{}", warning).bright_yellow());

        if verbose {
            // Could add more details here in verbose mode
        }

        println!();
    }
}

/// Info command - display package information
fn info_command(path: PathBuf) -> Result<(), i32> {
    println!("{}", "=".repeat(60).bright_blue());
    println!("{} {}", "Package Information:".bright_cyan().bold(), path.display());
    println!("{}", "=".repeat(60).bright_blue());
    println!();

    // Load package
    let package = match load_package(&path) {
        Ok(pkg) => pkg,
        Err(e) => {
            println!("{} {}", "✗".red().bold(), "Failed to load package".red());
            println!("  {}", format!("{}", e).bright_red());
            return Err(1);
        }
    };

    // Display info
    println!("{} {} v{}",
        "Package:".bright_cyan().bold(),
        package.id.bright_white().bold(),
        package.version.bright_white());

    if let Some(desc) = &package.metadata.description {
        println!("{} {}", "Description:".bright_cyan(), desc);
    }

    if !package.metadata.authors.is_empty() {
        println!("{} {}", "Authors:".bright_cyan(), package.metadata.authors.join(", "));
    }

    println!();
    println!("{} {}", "Namespaces:".bright_cyan().bold(), package.namespaces.len());

    for (ns_id, namespace) in &package.namespaces {
        println!();
        println!("  {} {}", "└─".bright_blue(), ns_id.bright_white().bold());
        println!("     {} {} datatype(s)", "├─".bright_black(), namespace.datatypes.len());
        println!("     {} {} promptsection(s)", "├─".bright_black(), namespace.prompt_sections.len());
        println!("     {} {} separator set(s)", "├─".bright_black(), namespace.separator_sets.len());
        println!("     {} {} rule(s)", "└─".bright_black(), namespace.rules.len());
    }

    println!();
    println!("{} {}", "Dependencies:".bright_cyan().bold(), package.dependencies.len());

    if !package.dependencies.is_empty() {
        for dep in &package.dependencies {
            println!("  {} {} (v{})", "└─".bright_blue(), dep.package_id, dep.version);
        }
    }

    println!();
    println!("{}", "─".repeat(60).bright_black());

    Ok(())
}

/// Render command - render a prompt section
fn render_command(path: PathBuf, section: String, seed: u64, count: usize) -> Result<(), i32> {
    use renderer::Renderer;
    use parser::load_package_with_dependencies;

    println!("{}", "=".repeat(60).bright_blue());
    println!("{} {}", "Rendering:".bright_cyan().bold(), section.bright_white());
    println!("{}", "=".repeat(60).bright_blue());
    println!();

    // M8.5 Blocker 2 Phase 2: Load package with dependencies
    println!("{} Loading package from {}", "→".bright_blue(), path.display());

    let loaded = match load_package_with_dependencies(&path) {
        Ok(loaded) => {
            println!("{} Package loaded", "✓".green());
            if !loaded.dependencies.is_empty() {
                println!("{} {} dependencies loaded", "✓".green(), loaded.dependencies.len());
            }
            loaded
        }
        Err(e) => {
            println!("{} {}", "✗".red().bold(), "Failed to load package".red());
            println!("  {}", format!("{}", e).bright_red());
            return Err(1);
        }
    };

    println!();

    // Render
    for i in 0..count {
        let current_seed = seed + i as u64;

        if count > 1 {
            println!("{} {} (Seed: {})",
                format!("#{}", i + 1).bright_blue().bold(),
                "─".repeat(54).bright_black(),
                current_seed.to_string().bright_black());
        } else {
            println!("{} {}", "Seed:".bright_cyan(), current_seed);
        }

        // M8.5 Blocker 2 Phase 2: Use new_with_dependencies
        let renderer = Renderer::new_with_dependencies(&loaded.package, &loaded.dependencies, current_seed);

        match renderer.render(&section) {
            Ok(result) => {
                println!();
                println!("  {}", result.output.bright_white().bold());
                println!();

                if count == 1 {
                    println!("{}", "─".repeat(60).bright_black());
                    println!("{} {:.2}ms", "Render time:".bright_cyan(), 0.0); // TODO: actual timing
                }
            }
            Err(e) => {
                println!();
                println!("  {} {}", "✗".red().bold(), format!("Render error: {}", e).red());
                println!();
                return Err(1);
            }
        }

        if i < count - 1 {
            println!();
        }
    }

    if count > 1 {
        println!();
        println!("{}", "─".repeat(60).bright_black());
        println!("{} {}", "Total:".bright_cyan(), format!("{} prompts rendered", count).bright_white());
        println!("{}", "─".repeat(60).bright_black());
    }

    Ok(())
}

