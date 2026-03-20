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
        print(f"No se encontró el directorio de Antigravity en {brain_dir}")
        
    date_str = datetime.datetime.now().strftime('%Y-%m-%d')
    zip_path_ag = dest_dir / f"{date_str}_antigravity_brain_backup"
    
    if brain_dir.exists():
        print(f"Comprimiendo {brain_dir} en {zip_path_ag}.zip ...")
        shutil.make_archive(str(zip_path_ag), 'zip', str(brain_dir))
        
    # Claude Backups
    claude_dir = user_home / ".claude"
    claude_code_dir = user_home / ".claude-code"
    
    if claude_dir.exists():
        zip_path_claude = dest_dir / f"{date_str}_claude_backup"
        print(f"Comprimiendo {claude_dir} en {zip_path_claude}.zip ...")
        shutil.make_archive(str(zip_path_claude), 'zip', str(claude_dir))
        
    if claude_code_dir.exists():
        zip_path_clc = dest_dir / f"{date_str}_claude_code_backup"
        print(f"Comprimiendo {claude_code_dir} en {zip_path_clc}.zip ...")
        shutil.make_archive(str(zip_path_clc), 'zip', str(claude_code_dir))
    
    print(f"\n¡Backup completado! Los cerebros e historiales están a salvo en la carpeta {dest_dir}.")

if __name__ == "__main__":
    backup_conversations()
