use crate::ast::{Program, ToTrz};

/// Serializa un `Program` a texto Trenza válido, parseable por `parse_file`
/// sin pérdida de estructura.
///
/// Construido sobre el trait `ToTrz` definido en `ast.rs`. Alcance: `Data`,
/// `Context`, `External`, `System`, `Import`. No cubre construcciones que no
/// aparecen en superficies públicas típicas (p.ej. bloques `runtime`).
pub fn serialize_trz(program: &Program) -> String {
    program.to_trz()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_file;
    use crate::pub_surface::public_surface;
    use crate::ast::Definition;

    /// Roundtrip básico: la serialización de una superficie pública debe
    /// reparsear sin pérdida de definiciones.
    #[test]
    fn roundtrip_public_surface_data() {
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
        let text = serialize_trz(&surface);
        let reparsed = parse_file(&text).expect("serialized output must reparse");
        assert_eq!(
            reparsed.definitions.len(),
            surface.definitions.len(),
            "roundtrip lost definitions. Serialized:\n{}", text
        );
    }

    /// Roundtrip con un Context que referencia un data no marcado pub:
    /// el cierre transitivo debe incluir el data y el roundtrip debe conservarlo.
    #[test]
    fn roundtrip_context_with_transitive_data() {
        let source = "
pub context Compra:
  input:
    precio: Importe

data Importe:
  valor: Entero
  moneda: Texto

data Interno:
  secreto: Texto
        ";
        let program = parse_file(source).unwrap();
        let surface = public_surface(&program);
        let text = serialize_trz(&surface);
        let reparsed = parse_file(&text).expect("serialized output must reparse");

        // La superficie debe contener Compra e Importe (transitivo), no Interno.
        let names: Vec<_> = surface.definitions.iter().filter_map(|d| match d {
            Definition::Data(d) => Some(d.name.as_str()),
            Definition::Context(c) => Some(c.name.as_str()),
            _ => None,
        }).collect();
        assert!(names.contains(&"Compra"), "Compra debe estar en la superficie");
        assert!(names.contains(&"Importe"), "Importe debe estar por cierre transitivo");
        assert!(!names.contains(&"Interno"), "Interno no debe estar en la superficie");

        // Y el roundtrip conserva el número de definiciones.
        assert_eq!(reparsed.definitions.len(), surface.definitions.len(),
            "roundtrip lost definitions. Serialized:\n{}", text);
    }
}
