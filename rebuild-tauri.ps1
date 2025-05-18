# Script para limpiar y recompilar el backend y frontend de EduNet (Tauri)
# Uso: Ejecuta este archivo en PowerShell desde la raíz del proyecto

Write-Host "Cerrando procesos de Tauri..."
Get-Process | Where-Object { $_.ProcessName -like 'EduNet*' -or $_.ProcessName -like 'tauri*' } | ForEach-Object { $_.CloseMainWindow() | Out-Null }

Write-Host "Limpiando y compilando backend (Rust/Tauri)..."
Set-Location src-tauri
cargo clean
cargo build
Set-Location ..

Write-Host "Reiniciando frontend (React)..."
# Detener procesos de npm/yarn si están corriendo
Get-Process | Where-Object { $_.ProcessName -like 'node*' -or $_.ProcessName -like 'npm*' -or $_.ProcessName -like 'yarn*' } | ForEach-Object { $_.CloseMainWindow() | Out-Null }

# Iniciar frontend y Tauri en modo desarrollo
Start-Process powershell -ArgumentList 'npm run dev' -NoNewWindow
Start-Process powershell -ArgumentList 'npm run tauri dev' -NoNewWindow

Write-Host "Listo. El backend y frontend han sido limpiados y reiniciados." 