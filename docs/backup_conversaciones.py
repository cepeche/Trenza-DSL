import os
import shutil
from pathlib import Path
import datetime

def backup_conversations():
    # Ruta al "cerebro" local de Antigravity del usuario
    user_home = Path(os.path.expanduser("~"))
    brain_dir = user_home / ".gemini" / "antigravity" / "brain"
    
    # Directorio de destino en el repositorio
    dest_dir = Path(__file__).parent / "historial_ias"
    os.makedirs(dest_dir, exist_ok=True)
    
    if not brain_dir.exists():
        print(f"No se encontró el directorio de conversaciones en {brain_dir}")
        return
        
    print(f"Buscando conversaciones en {brain_dir}...")
    copied = 0
    
    for session_dir in brain_dir.iterdir():
        if session_dir.is_dir():
            # Buscar si tiene un system_generated/logs/overview.txt
            log_file = session_dir / ".system_generated" / "logs" / "overview.txt"
            if log_file.exists():
                session_id = session_dir.name
                # Podemos usar la fecha de modificación del archivo
                mtime = os.path.getmtime(log_file)
                date_str = datetime.datetime.fromtimestamp(mtime).strftime('%Y-%m-%d')
                
                dest_file = dest_dir / f"{date_str}_antigravity_{session_id[:8]}.txt"
                shutil.copy2(log_file, dest_file)
                
                # Intentar copiar artefactos (walkthrough, task, etc)
                for artifact in ["task.md", "implementation_plan.md", "walkthrough.md"]:
                    art_file = session_dir / artifact
                    if art_file.exists():
                        shutil.copy2(art_file, dest_dir / f"{date_str}_antigravity_{session_id[:8]}_{artifact}")
                
                copied += 1
                print(f"  -> Guardada sesión {session_id[:8]} ({date_str})")
                
    print(f"\n¡Backup completado! Se guardaron {copied} sesiones en {dest_dir}. Haz 'git commit' para preservarlas.")

if __name__ == "__main__":
    backup_conversations()
