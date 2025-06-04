import { useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface BuscadorEstudianteProps {
  onSeleccionar: (estudiante: any) => void;
  onCargarExcel?: (file: File) => void;
}

export function BuscadorEstudiante({ onSeleccionar, onCargarExcel }: BuscadorEstudianteProps) {
  const [cedula, setCedula] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleBuscar = async () => {
    setLoading(true);
    setError(null);
    try {
      const cedulaNum = Number(cedula);
      if (!cedulaNum) {
        setError("Ingresa una cédula válida.");
        setLoading(false);
        return;
      }
      const resultado = await invoke<any>("obtener_estudiantes", {
        filtro: { cedula: cedulaNum.toString() },
        paginacion: { pagina: 1, registros_por_pagina: 1 }
      });
      const estudiante = resultado?.datos?.[0];
      if (!estudiante) {
        setError("No se encontró ningún estudiante con esa cédula.");
      } else {
        onSeleccionar(estudiante);
      }
    } catch (e) {
      setError("Error al buscar estudiante.");
    } finally {
      setLoading(false);
    }
  };

  const handleExcelClick = () => {
    fileInputRef.current?.click();
  };

  const handleExcelChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file && onCargarExcel) {
      onCargarExcel(file);
    }
    e.target.value = "";
  };

  return (
    <div className="flex items-center gap-4 mb-8">
      <input
        type="text"
        className="border rounded-lg px-4 py-2 w-64"
        placeholder="Buscar estudiante por cédula"
        value={cedula}
        onChange={e => setCedula(e.target.value)}
        onKeyDown={e => e.key === "Enter" && handleBuscar()}
      />
      <button
        className="px-5 py-2 rounded-lg bg-emerald-600 text-white hover:bg-emerald-700 font-semibold shadow"
        onClick={handleBuscar}
        disabled={loading}
      >
        {loading ? "Buscando..." : "Buscar"}
      </button>
      <button
        type="button"
        className="px-5 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 font-semibold shadow"
        onClick={handleExcelClick}
      >
        Cargar Excel
      </button>
      <input
        type="file"
        accept=".xlsx,.xls"
        ref={fileInputRef}
        style={{ display: "none" }}
        onChange={handleExcelChange}
      />
      {error && <span className="text-red-500 ml-4">{error}</span>}
    </div>
  );
} 