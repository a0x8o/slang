mod test_nodes;

use std::{self, cmp::max, fmt::Write};

use anyhow::Result;
use solidity_rust_lib::generated::language::ParserOutput;

use crate::cst_snapshots::test_nodes::TestNode;

pub trait ParserOutputTestSnapshotExtensions {
    fn to_test_snapshot(&self, source_id: &str, source: &str) -> Result<String>;
}

impl ParserOutputTestSnapshotExtensions for ParserOutput {
    fn to_test_snapshot(&self, source_id: &str, source: &str) -> Result<String> {
        let mut w = String::new();

        write_source(&mut w, source)?;
        writeln!(&mut w)?;

        write_errors(&mut w, &self, source_id, source)?;
        writeln!(&mut w)?;

        write_tree(&mut w, &self, source)?;

        return Ok(w);
    }
}

fn write_source<W: Write>(w: &mut W, source: &str) -> Result<()> {
    if source.is_empty() {
        writeln!(w, "Source: \"\"")?;
        return Ok(());
    }

    let line_data = source
        .lines()
        .enumerate()
        .map(|(index, line)| (index, line, line.chars().count()))
        .collect::<Vec<_>>();

    let source_width = {
        let source_width = line_data
            .iter()
            .map(|(_, _, length)| *length)
            .max()
            .unwrap_or(0);

        max(source_width, 80)
    };

    writeln!(w, "Source: >")?;

    let mut offset = 0;
    for (index, line, length) in line_data.iter() {
        let range = offset..(offset + length);
        writeln!(
            w,
            "  {line_number: <2} │ {line}{padding} │ {range:?}",
            line_number = index + 1,
            padding = " ".repeat(source_width - length),
        )?;

        offset = range.end + 1;
    }

    return Ok(());
}

fn write_errors<W: Write>(
    w: &mut W,
    output: &ParserOutput,
    source_id: &str,
    source: &str,
) -> Result<()> {
    let errors = output.errors_as_strings(source_id, source, /* with_colour */ false);

    if errors.len() == 0 {
        writeln!(w, "Errors: []")?;
        return Ok(());
    }

    writeln!(w, "Errors: # {} total", errors.len())?;

    for error in errors {
        writeln!(w, "  - >")?;
        for line in error.lines() {
            writeln!(w, "    {line}")?;
        }
    }

    return Ok(());
}

fn write_tree<W: Write>(w: &mut W, output: &ParserOutput, source: &str) -> Result<()> {
    let root_node = if let Some(parse_tree) = output.parse_tree() {
        TestNode::from_cst(&parse_tree)
    } else {
        writeln!(w, "Tree: null")?;
        return Ok(());
    };

    writeln!(w, "Tree:")?;
    write_node(w, &root_node, source, 0)?;
    return Ok(());
}

fn write_node<W: Write>(
    w: &mut W,
    node: &TestNode,
    source: &str,
    indentation: usize,
) -> Result<()> {
    writeln!(
        w,
        "{indentation}  - {kind}: {contents} # {range}",
        indentation = " ".repeat(4 * indentation),
        kind = node.kind,
        contents = node.render_contents(source)?,
        range = if let Some(range) = &node.range {
            format!("{range:?}")
        } else {
            "<empty>".to_owned()
        },
    )?;

    for child in &node.children {
        write_node(w, child, source, indentation + 1)?;
    }

    return Ok(());
}