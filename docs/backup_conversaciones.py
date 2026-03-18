import os
import shutil
from pathlib import Path
import datetime

def backup_conversations():
    user_home = Path(os.path.expanduser("~"))
    brain_dir = user_home / ".gemini" / "antigravity" / "brain"
    
    dest_dir = Path(__file__).parent / "historial_ias"
    os.makedirs(dest_dir, exist_ok=True)
    
    if not brain_dir.exists():
        print(f"No se encontró el directorio de conversaciones en {brain_dir}")
        return
        
    date_str = datetime.datetime.now().strftime('%Y-%m-%d')
    zip_path = dest_dir / f"{date_str}_antigravity_brain_backup"
    
    print(f"Comprimiendo {brain_dir} en {zip_path}.zip ...")
    shutil.make_archive(str(zip_path), 'zip', str(brain_dir))
    
    print(f"\n¡Backup completado! Todo el cerebro de Antigravity está a salvo en {zip_path}.zip. Haz 'git commit' de este archivo para preservarlo en el repositorio.")

if __name__ == "__main__":
    backup_conversations()
