use crate::ast::*;
use std::collections::{HashSet, VecDeque};

pub fn public_surface(program: &Program) -> Program {
    let mut public_definitions = Vec::new();
    let mut reachable_names = HashSet::new();
    let mut queue = VecDeque::new();

    // 1. Encontrar semillas (elementos marcados como pub)
    for def in &program.definitions {
        match def {
            Definition::Data(d) if d.is_public => {
                reachable_names.insert(d.name.clone());
                queue.push_back(def.clone());
            }
            Definition::Context(c) if c.is_public => {
                reachable_names.insert(c.name.clone());
                queue.push_back(def.clone());
            }
            Definition::Enum(e) if e.is_public => {
                reachable_names.insert(e.name.clone());
                queue.push_back(def.clone());
            }
            // Los imports se mantienen siempre
            Definition::Import(_) => {
                public_definitions.push(def.clone());
            }
            // El bloque System se mantiene siempre si existe
            Definition::System(_) => {
                public_definitions.push(def.clone());
            }
            _ => {}
        }
    }

    // 2. Recorrido transitivo para encontrar dependencias de tipos
    while let Some(def) = queue.pop_front() {
        public_definitions.push(def.clone());
        let deps = get_dependencies(&def);
        
        for dep in deps {
            if !is_primitive(&dep) && !reachable_names.contains(&dep) {
                // Buscar la definición en el programa original
                if let Some(dep_def) = find_definition(program, &dep) {
                    reachable_names.insert(dep.clone());
                    queue.push_back(dep_def);
                }
            }
        }
    }

    // Sort definitions by type and name for deterministic hashing (ADR-022/Bloque 2.1)
    public_definitions.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));

    Program {
        definitions: public_definitions,
    }
}

fn get_dependencies(def: &Definition) -> Vec<String> {
    let mut deps = Vec::new();
    match def {
        Definition::Data(d) => {
            for field in &d.fields {
                deps.push(field.datatype.clone());
            }
        }
        Definition::Context(c) => {
            for input in &c.inputs {
                deps.push(input.datatype.clone());
            }
            for role in &c.roles {
                deps.push(role.datatype.clone());
            }
            for fills in &c.fills {
                for role in &fills.roles {
                    deps.push(role.datatype.clone());
                }
            }
        }
        Definition::External(e) => {
            for action in &e.actions {
                for (_, ty) in &action.params {
                    deps.push(ty.clone());
                }
                deps.push(action.return_type.clone());
            }
        }
        Definition::Enum(_) => {}
        _ => {}
    }
    deps
}

fn find_definition(program: &Program, name: &str) -> Option<Definition> {
    for def in &program.definitions {
        match def {
            Definition::Data(d) if d.name == name => return Some(def.clone()),
            Definition::Context(c) if c.name == name => return Some(def.clone()),
            Definition::External(e) if e.name == name => return Some(def.clone()),
            Definition::Enum(e) if e.name == name => return Some(def.clone()),
            _ => {}
        }
    }
    None
}

fn is_primitive(name: &str) -> bool {
    // Lista básica de primitivos de Trenza
    match name {
        "Texto" | "Entero" | "Decimal" | "Booleano" | "Instante" | "Nada" | "Raiz" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_file;

    #[test]
    fn test_transitive_closure() {
        let source = "
pub data A:
  f: B
  g: Texto

data B:
  h: C

data C:
  i: Entero

data D:
  j: Texto
        ";
        let program = parse_file(source).unwrap();
        let surface = public_surface(&program);
        
        // A es pub, por lo tanto A está
        // A -> B, por lo tanto B está
        // B -> C, por lo tanto C está
        // D no es pub ni referenciado, por lo tanto D NO está
        
        let names: HashSet<_> = surface.definitions.iter().filter_map(|d| {
            match d {
                Definition::Data(d) => Some(d.name.clone()),
                _ => None
            }
        }).collect();
        
        assert!(names.contains("C"));
        assert!(!names.contains("D"));
    }

    #[test]
    fn test_deterministic_surface() {
        let source1 = "
pub data A:
  x: Entero
pub data B:
  y: Texto
";
        let source2 = "
pub data B:
  y: Texto
pub data A:
  x: Entero
";
        let prog1 = parse_file(source1).unwrap();
        let prog2 = parse_file(source2).unwrap();
        
        let surf1 = public_surface(&prog1);
        let surf2 = public_surface(&prog2);
        
        // Serialized strings should be identical
        use crate::serializer::serialize_trz;
        assert_eq!(serialize_trz(&surf1), serialize_trz(&surf2));
    }
}
