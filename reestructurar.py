import os
import subprocess
from pathlib import Path

def run_cmd(cmd):
    print(f"Running: {cmd}")
    subprocess.run(cmd, shell=True)

def main():
    base = Path(r"c:\Proyectos\Trenza-DSL")
    os.chdir(base)
    
    # 1. Conservar cambios sueltos y taggear v0.0.0
    run_cmd('git add .')
    run_cmd('git commit -m "chore: consolidacion final pre-v0.0.0"')
    run_cmd('git tag -a v0.0.0 -m "Primera especificacion completa de Trenza"')
    
    # 2. Reemplazo bloqueado -> forbidden en ejemplos
    ejemplos = base / "examples"
    if ejemplos.exists():
        for root, _, files in os.walk(ejemplos):
            for f in files:
                if f.endswith('.trz') or f.endswith('.helix'):
                    path = Path(root) / f
                    content = path.read_text(encoding='utf-8')
                    if 'bloqueado' in content:
                        path.write_text(content.replace('bloqueado', 'forbidden'), encoding='utf-8')
                        run_cmd(f'git add "{path}"')
    
    # 3. Crear árbol de directorios
    dirs = [
        "spec/language", "spec/reference/cronometro-psp",
        "docs/design", "docs/manual", "docs/generated/sistema",
        "history/chronicle", "history/decisions", "history/inspirations", "history/meta",
        ".agents/scripts"
    ]
    for d in dirs:
        (base / d).mkdir(parents=True, exist_ok=True)
        
    # 4. Movimientos especiales prioritarios
    docs_dir = base / "docs"
    specific_moves = [
        ("docs/Directrices_PI_IA.md", "history/meta/directrices-pi-ia.md"),
        ("docs/2026-03-20-06-directrices-pi-ia.md", "history/meta/directrices-pi-ia.md"),
        ("docs/2026-03-13-09-metafisica-de-trenza.md", "history/meta/metafisica-de-trenza.md"),
        ("docs/backup_conversaciones.py", ".agents/scripts/backup_conversaciones.py"),
        ("docs/2026-03-20-04-manual-usuario-trenza.md", "docs/manual/trenza-manual.md"),
    ]
    for src, dst in specific_moves:
        p_src = base / src
        if p_src.exists():
            run_cmd(f'git mv "{src}" "{dst}"')
            
    # 5. Mover el resto de la cronica (docs/2026-03-*.md)
    if docs_dir.exists():
        for file in os.listdir(docs_dir):
            if file.startswith("2026-03-") and file.endswith(".md"):
                date_str = file[:10]  # extrae YYYY-MM-DD
                new_name = file[11:]  # elimina YYYY-MM-DD-
                dest_dir = base / "history" / "chronicle" / date_str
                dest_dir.mkdir(parents=True, exist_ok=True)
                # Opcional: si new_name empieza por número pero no queremos alterarlo:
                run_cmd(f'git mv "docs/{file}" "history/chronicle/{date_str}/{new_name}"')
                
    # 6. Mover la especificacion del cronometro
    src_trenza = base / "examples" / "cronometro-psp" / "trenza"
    if src_trenza.exists():
        run_cmd('git mv "examples/cronometro-psp/trenza" "spec/reference/cronometro-psp/"')
        
    # 7. Si existía helix/ lo cambiamos también por si acaso
    src_helix = base / "examples" / "cronometro-psp" / "helix"
    if src_helix.exists():
        run_cmd('git mv "examples/cronometro-psp/helix" "spec/reference/cronometro-psp/trenza"')
        # Renombrar .helix a .trz si quedan
        for root, _, files in os.walk(base / "spec" / "reference" / "cronometro-psp" / "trenza"):
            for f in files:
                if f.endswith('.helix'):
                    old_path = Path(root) / f
                    new_path = Path(root) / (f[:-6] + ".trz")
                    run_cmd(f'git mv "{old_path}" "{new_path}"')

    # Status final
    run_cmd('git status')
    print("Reestructuración completada.")

if __name__ == "__main__":
    main()
