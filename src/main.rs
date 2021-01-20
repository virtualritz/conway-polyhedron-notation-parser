use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;
//extern crate pest;
#[macro_use]
extern crate pest_derive;

/*
extern crate polyhedron_ops;
*/

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use polyhedron_ops::prelude::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ConwayPolyhedronNotationParser;

fn main() {
    let conway_notation_token_tree = match ConwayPolyhedronNotationParser::parse(
        Rule::conway_notation_string,
        "g0.2k,,{t}b2T",
    ) {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut token_tree = Vec::new();
    for token in conway_notation_token_tree {
        token_tree.push(token);
    }
    token_tree.pop(); // remove EOI
    token_tree.reverse();

    let mut poly = Polyhedron::new();

    token_tree.iter().for_each(|t| match t.as_rule() {
        Rule::tetrahedron => {
            poly = Polyhedron::tetrahedron();
        }
        Rule::ambo => {
            poly.ambo(to_number(&mut t.clone().into_inner()).0, true);
        }
        Rule::bevel => {
            let mut inner = t.clone().into_inner();
            let (ratio, mut inner) = to_number(&mut inner);
            let (height, mut inner) = to_number(&mut inner);
            let (vertex_valence, mut inner) = to_vec(&mut inner);
            let (regular, _) = to_number(&mut inner);
            poly.bevel(
                height,
                ratio,
                vertex_valence,
                regular,
                true,
            );
        }
        Rule::chamfer => {
            poly.chamfer(to_number(&mut t.clone().into_inner()).0, true);
        }
        Rule::dual => {
            poly.dual(true);
        }
        Rule::expand => {
            poly.expand(to_number(&mut t.clone().into_inner()).0, true);
        }
        Rule::gyro => {
            let mut inner = t.clone().into_inner();
            let (ratio, mut inner) = to_number(&mut inner);
            let (height, _) = to_number(&mut inner);
            poly.gyro(ratio, height, true);
        }
        Rule::kis => {
            let mut inner = t.clone().into_inner();
            let (height, mut inner) = to_number(&mut inner);
            let (vertex_valence, mut inner) = to_vec(&mut inner);
            let (regular, _) = to_number(&mut inner);
            poly.kis(
                height,
                vertex_valence,
                regular,
                true,
            );
        }
        _ => (),
    });

    println!(
        "Exported to {}",
        poly.write_to_obj(Path::new("."), true).unwrap().display()
    );
}

fn to_number<'a, T>(tokens: &'a mut Pairs<'a, Rule>) -> (Option<T>, &'a mut Pairs<'a, Rule>)
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    if tokens.clone().next().is_none() {
        tokens.next();
        return (None, tokens);
    }

    if Rule::separator == tokens.clone().next().unwrap().as_rule() {
        tokens.next();
    }

    if tokens.clone().next().is_none() {
        tokens.next();
        return (None, tokens);
    }

    if Rule::separator == tokens.clone().next().unwrap().as_rule() {
        tokens.next();
        return (None, tokens);
    }

    let value = tokens.as_str().parse::<T>().unwrap();
    tokens.next();
    (Some(value), tokens)
}

fn to_vec<'a, T>(tokens: &'a mut Pairs<'a, Rule>) -> (Option<Vec<T>>, &'a mut Pairs<'a, Rule>)
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    if tokens.clone().next().is_none() {
        tokens.next();
        return (None, tokens);
    }

    if Rule::separator == tokens.clone().next().unwrap().as_rule() {
        tokens.next();
        return (None, tokens);
    }

    let mut vertex_valence = Vec::new();
    for token in tokens.clone() {
        vertex_valence.push(token.as_str().parse::<T>().unwrap());
    }

    if vertex_valence.is_empty() {
        (None, tokens)
    } else {
        tokens.next();
        (Some(vertex_valence), tokens)
    }
}

/*
fn format_iter<'a, T: 'a + Display>(vector: impl IntoIterator<Item = &'a T>
) -> String {
    if 0 == vector.len() {
        String::new()
    } else {
        let mut string = String::with_capacity(vector.len() * 2);
        string.push('[');
        write!(&mut string, "{}", vector[0]);
        for i in vector.get(1..).unwrap() {
        write!(&mut string, ",{}", i);
        }
        string.push(']');
        string
    }
}*/
