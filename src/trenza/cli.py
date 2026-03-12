import os
import sys
import json
import argparse
from pathlib import Path
from typing import Dict, Any

from .ast import Manifest, TrenzaProject
from .parser import TrenzaParser
from .verifier import TrenzaVerifier

def main():
    parser = argparse.ArgumentParser(description="Prototipo de Parser Trenza")
    parser.add_argument("path", help="Directorio del proyecto Trenza")
    parser.add_argument("--out", help="Archivo JSON de salida", default="trenza-ast.json")
    
    args = parser.parse_args()
    
    base_dir = Path(args.path)
    if not base_dir.is_dir():
        print(f"Error: El directorio {base_dir} no existe.")
        sys.exit(1)
        
    print(f"Analizando proyecto Trenza en {base_dir}...")
    
    t_parser = TrenzaParser(str(base_dir))
    
    # 1. Buscar system.trz (para el prototipo lo referenciamos directo)
    sys_path = base_dir / "system.trz"
    if not sys_path.exists():
        print(f"Error: {sys_path} no encontrado.")
        sys.exit(1)

    system_decl = t_parser.parse_system(str(sys_path))
    
    # Manifest prototipo
    manifest = Manifest(
        manifest_version="0.1",
        trenza_version="0.1.0",
        system="system.trz",
        data="data.trz",
        contexts=[]
    )
    
    project = TrenzaProject(
        manifest=manifest,
        system_decl=system_decl,
        parsed_contexts={}
    )
    
    # Load all mentioned contexts (base, concurrent, overlays)
    # They should exist in contexts/
    all_context_names = system_decl.contexts + system_decl.concurrent + system_decl.overlays
    for ctx_name in all_context_names:
        ctx_file = base_dir / "contexts" / f"{ctx_name}.trz"
        if ctx_file.exists():
            print(f"  Parseando {ctx_file.name}...")
            ctx_node = t_parser.parse_context_file(str(ctx_file))
            project.parsed_contexts[ctx_node.name] = ctx_node
            manifest.contexts.append({"path": f"contexts/{ctx_file.name}", "sha256": "dummy"})
            
            # En a real loop, parse nested too. The _parse_context_block does it recursively inside the file
        else:
            print(f"Advertencia: no se encontró archivo para el contexto {ctx_name} ({ctx_file})")

    # Verificacion
    print("\nVerificando AST y aplicando reglas de herencia...")
    verifier = TrenzaVerifier(project)
    try:
        verifier.verify()
        for note in verifier.notes:
            print(f"  [Verificador] {note}")
        print("Verificación completada: OK")
    except Exception as e:
        print(f"\nError de Verificación: {e}")
        sys.exit(1)

    # Volcado AST
    out_file = Path(args.out)
    with open(out_file, "w", encoding="utf-8") as f:
        f.write(project.to_json())
    
    print(f"\nAST exportado con éxito a {out_file}")

if __name__ == "__main__":
    main()
