use std::{env, fmt::Debug, path::Path, str::FromStr};
#[macro_use]
extern crate pest_derive;
use miette::{IntoDiagnostic, Result};
use pest::{iterators::Pairs, Parser};
use polyhedron_ops::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ConwayPolyhedronNotationParser;

fn main() -> Result<()> {
    let poly_string = if let Some(arg) = env::args().collect::<Vec<_>>().get(1) {
        arg.to_string()
    } else {
        eprintln!("No input string provided. Using default `eD` (Rhombicosidodecahedron)");
        "eD".to_string()
    };

    let mut poly = Polyhedron::new();

    let conway_notation_token_tree =
        ConwayPolyhedronNotationParser::parse(Rule::conway_notation_string, &poly_string)
            .into_diagnostic()?;

    conway_notation_token_tree.rev().skip(1).for_each(|pair| {
        let token = pair.clone().into_inner();
        match pair.as_rule() {
            Rule::tetrahedron => {
                poly = Polyhedron::tetrahedron();
            }
            Rule::hexahedron => {
                poly = Polyhedron::hexahedron();
            }
            Rule::octahedron => {
                poly = Polyhedron::octahedron();
            }
            Rule::dodecahedron => {
                poly = Polyhedron::dodecahedron();
            }
            Rule::icosahedron => {
                poly = Polyhedron::icosahedron();
            }
            Rule::ambo => {
                poly.ambo(to_number(token).0, true);
            }
            Rule::bevel => {
                let (ratio, token) = to_number(token);
                let (height, token) = to_number(token);
                let (vertex_valence, token) = to_vec(token);
                let (regular, _) = to_bool(token);
                poly.bevel(
                    ratio,
                    height,
                    if vertex_valence.is_empty() {
                        None
                    } else {
                        Some(vertex_valence.as_slice())
                    },
                    regular,
                    true,
                );
            }
            Rule::catmull_clark_subdivide => {
                poly.catmull_clark_subdivide(true);
            }
            Rule::chamfer => {
                poly.chamfer(to_number(token).0, true);
            }
            Rule::dual => {
                poly.dual(true);
            }
            Rule::expand => {
                poly.expand(to_number(token).0, true);
            }
            Rule::extrude => {
                let (height, token) = to_number(token);
                let (offset, token) = to_number(token);
                let (face_arity_mask, _) = to_vec(token);
                poly.extrude(
                    height,
                    offset,
                    if face_arity_mask.is_empty() {
                        None
                    } else {
                        Some(face_arity_mask.as_slice())
                    },
                    true,
                );
            }
            Rule::gyro => {
                let (ratio, token) = to_number(token);
                let (height, _) = to_number(token);
                poly.gyro(ratio, height, true);
            }
            Rule::inset => {
                let (offset, token) = to_number(token);
                let (face_arity_mask, _) = to_vec(token);
                poly.inset(
                    offset,
                    if face_arity_mask.is_empty() {
                        None
                    } else {
                        Some(face_arity_mask.as_slice())
                    },
                    true,
                );
            }
            Rule::join => {
                poly.join(to_number(token).0, true);
            }
            Rule::kis => {
                let (height, token) = to_number(token);
                let (face_arity_mask, token) = to_vec(token);
                let (face_index_mask, token) = to_vec(token);
                let (regular, _) = to_bool(token);
                poly.kis(
                    height,
                    if face_arity_mask.is_empty() {
                        None
                    } else {
                        Some(face_arity_mask.as_slice())
                    },
                    if face_index_mask.is_empty() {
                        None
                    } else {
                        Some(face_index_mask.as_slice())
                    },
                    regular,
                    true,
                );
            }
            Rule::medial => {
                let (ratio, token) = to_number(token);
                let (height, token) = to_number(token);
                let (vertex_valence, token) = to_vec(token);
                let (regular, _) = to_bool(token);
                poly.medial(
                    ratio,
                    height,
                    if vertex_valence.is_empty() {
                        None
                    } else {
                        Some(vertex_valence.as_slice())
                    },
                    regular,
                    true,
                );
            }
            Rule::meta => {
                let (ratio, token) = to_number(token);
                let (height, token) = to_number(token);
                let (vertex_valence, token) = to_vec(token);
                let (regular, _) = to_bool(token);
                poly.meta(
                    ratio,
                    height,
                    if vertex_valence.is_empty() {
                        None
                    } else {
                        Some(vertex_valence.as_slice())
                    },
                    regular,
                    true,
                );
            }
            Rule::needle => {
                let (height, token) = to_number(token);
                let (vertex_valence, token) = to_vec(token);
                let (regular, _) = to_bool(token);
                poly.needle(
                    height,
                    if vertex_valence.is_empty() {
                        None
                    } else {
                        Some(vertex_valence.as_slice())
                    },
                    regular,
                    true,
                );
            }
            Rule::ortho => {
                poly.ortho(to_number(token).0, true);
            }
            Rule::planarize => {
                poly.planarize(to_number(token).0, true);
            }
            Rule::propellor => {
                poly.propellor(to_number(token).0, true);
            }
            Rule::quinto => {
                poly.quinto(to_number(token).0, true);
            }
            Rule::reflect => {
                poly.reflect(true);
            }
            Rule::snub => {
                let (ratio, token) = to_number(token);
                let (height, _) = to_number(token);
                poly.snub(ratio, height, true);
            }
            Rule::spherize => {
                poly.spherize(to_number(token).0, true);
            }
            Rule::truncate => {
                let (height, token) = to_number(token);
                let (vertex_valence_mask, token) = to_vec(token);
                let (regular, _) = to_bool(token);
                poly.truncate(
                    height,
                    if vertex_valence_mask.is_empty() {
                        None
                    } else {
                        Some(vertex_valence_mask.as_slice())
                    },
                    regular,
                    true,
                );
            }
            Rule::whirl => {
                let (ratio, token) = to_number(token);
                let (height, _) = to_number(token);
                poly.whirl(ratio, height, true);
            }
            Rule::zip => {
                let (height, token) = to_number(token);
                let (vertex_valence_mask, token) = to_vec(token);
                let (regular, _) = to_bool(token);
                poly.zip(
                    height,
                    if vertex_valence_mask.is_empty() {
                        None
                    } else {
                        Some(vertex_valence_mask.as_slice())
                    },
                    regular,
                    true,
                );
            }
            _ => (),
        }
    });

    let path = poly.write_obj(Path::new("."), true).unwrap();
    println!("Exported to {}", path.display());

    let name = path.as_path().file_stem().unwrap().to_str().unwrap();

    assert!(name.ends_with(
        &poly_string
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    ));

    Ok(())
}

fn is_empty_or_comma<'a>(mut tokens: Pairs<'a, Rule>) -> (bool, Pairs<'a, Rule>) {
    // No more tokens? Return None.
    match tokens.clone().next() {
        Some(token) => {
            if Rule::separator == token.as_rule() {
                tokens.next();
                (true, tokens)
            } else {
                (false, tokens)
            }
        }
        None => (true, tokens),
    }
}

fn to_bool<'a>(tokens: Pairs<'a, Rule>) -> (Option<bool>, Pairs<'a, Rule>) {
    let (exit, mut tokens) = is_empty_or_comma(tokens);

    if exit {
        return (None, tokens);
    }

    let result = match tokens.next().unwrap().as_str() {
        "{t}" => Some(true),
        "{f}" => Some(false),
        _ => None,
    };

    (result, tokens)
}

fn to_number<'a, T>(tokens: Pairs<'a, Rule>) -> (Option<T>, Pairs<'a, Rule>)
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let (exit, mut tokens) = is_empty_or_comma(tokens);

    if exit {
        return (None, tokens);
    }

    // Parse the next token as a number.
    let value = tokens.next().unwrap().as_str().parse::<T>().unwrap();

    // Skip possible trailing seprarator.
    tokens.next();

    (Some(value), tokens)
}

fn to_vec<'a, T>(tokens: Pairs<'a, Rule>) -> (Vec<T>, Pairs<'a, Rule>)
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let (exit, mut tokens) = is_empty_or_comma(tokens);

    if exit {
        return (Vec::new(), tokens);
    }

    let vertex_valence = tokens
        .clone()
        .take_while(|token| Rule::separator != token.as_rule())
        .map(|token| token.as_str().parse::<T>().unwrap())
        .collect::<Vec<_>>();

    if !vertex_valence.is_empty() {
        tokens.next();
        tokens.next();
    }

    // Skip trailing separator.
    tokens.next();

    (vertex_valence, tokens)
}
