import React, { useRef, useState } from 'react';
import * as XLSX from 'xlsx';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

export function CargaMasivaCalificaciones() {
  const [calificaciones, setCalificaciones] = useState<any[]>([]);
  const [resumen, setResumen] = useState<any | null>(null);
  const [cargando, setCargando] = useState(false);
  const [advertencias, setAdvertencias] = useState<string[]>([]);
  const [erroresPrevios, setErroresPrevios] = useState<string[]>([]);
  const inputRef = useRef<HTMLInputElement>(null);
  const { mostrarMensaje } = useMensajeGlobal();
  const [nombreArchivo, setNombreArchivo] = useState<string>("");

  // Validación avanzada
  const validarCalificaciones = (rows: any[]): { validas: any[], advertencias: string[], errores: string[] } => {
    const advertencias: string[] = [];
    const errores: string[] = [];
    const idsVistos = new Set<string>();
    const validas = rows.filter((c, idx) => {
      let filaValida = true;
      // Validar campos obligatorios
      if (!c.id_estudiante || !c.id_asignatura || !c.id_periodo) {
        errores.push(`Fila ${idx + 2}: Faltan campos obligatorios (id_estudiante, id_asignatura, id_periodo)`);
        filaValida = false;
      }
      // Validar IDs
      if (isNaN(Number(c.id_estudiante)) || Number(c.id_estudiante) <= 0) {
        errores.push(`Fila ${idx + 2}: id_estudiante inválido`);
        filaValida = false;
      }
      if (isNaN(Number(c.id_asignatura)) || Number(c.id_asignatura) <= 0) {
        errores.push(`Fila ${idx + 2}: id_asignatura inválido`);
        filaValida = false;
      }
      if (isNaN(Number(c.id_periodo)) || Number(c.id_periodo) <= 0) {
        errores.push(`Fila ${idx + 2}: id_periodo inválido`);
        filaValida = false;
      }
      // Validar notas
      ['lapso_1','lapso_2','lapso_3','lapso_1_ajustado','lapso_2_ajustado','lapso_3_ajustado','nota_final','revision'].forEach(campo => {
        if (c[campo] !== undefined && c[campo] !== null && c[campo] !== '') {
          const valor = Number(c[campo]);
          if (isNaN(valor) || valor < 0 || valor > 20) {
            advertencias.push(`Fila ${idx + 2}: El campo ${campo} tiene un valor fuera de rango (0-20)`);
          }
        }
      });
      // Duplicados
      const key = `${c.id_estudiante}-${c.id_asignatura}-${c.id_periodo}`;
      if (idsVistos.has(key)) {
        advertencias.push(`Fila ${idx + 2}: Duplicado de estudiante/asignatura/periodo`);
      } else {
        idsVistos.add(key);
      }
      return filaValida;
    });
    return { validas, advertencias, errores };
  };

  // Leer y validar el Excel
  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    setNombreArchivo(file.name);
    const reader = new FileReader();
    reader.onload = (evt) => {
      const data = evt.target?.result;
      const workbook = XLSX.read(data, { type: 'binary' });
      // Buscar la hoja llamada 'CARGA_MASIVA' (insensible a mayúsculas/minúsculas)
      const sheetName = workbook.SheetNames.find(name => name.toUpperCase().includes('CARGA_MASIVA')) || workbook.SheetNames[0];
      const sheet = workbook.Sheets[sheetName];
      const rows = XLSX.utils.sheet_to_json(sheet);
      const { validas, advertencias, errores } = validarCalificaciones(rows);
      setCalificaciones(validas);
      setAdvertencias(advertencias);
      setErroresPrevios(errores);
      setResumen(null);
      if (errores.length > 0) {
        mostrarMensaje('Hay errores críticos en el archivo. Corrige antes de continuar.', 'error');
      } else if (advertencias.length > 0) {
        mostrarMensaje('Hay advertencias en el archivo. Revisa antes de continuar.', 'advertencia');
      } else {
        mostrarMensaje('Archivo cargado correctamente.', 'exito');
      }
    };
    reader.readAsBinaryString(file);
  };

  // Enviar al backend
  const handleEnviar = async () => {
    setCargando(true);
    try {
      const payload = calificaciones.map((c) => ({
        id_estudiante: Number(c.id_estudiante),
        id_asignatura: Number(c.id_asignatura),
        id_periodo: Number(c.id_periodo),
        lapso_1: c.lapso_1 !== undefined ? Number(c.lapso_1) : null,
        lapso_2: c.lapso_2 !== undefined ? Number(c.lapso_2) : null,
        lapso_3: c.lapso_3 !== undefined ? Number(c.lapso_3) : null,
        revision: c.revision !== undefined ? Number(c.revision) : null,
        lapso_1_ajustado: c.lapso_1_ajustado !== undefined ? Number(c.lapso_1_ajustado) : null,
        lapso_2_ajustado: c.lapso_2_ajustado !== undefined ? Number(c.lapso_2_ajustado) : null,
        lapso_3_ajustado: c.lapso_3_ajustado !== undefined ? Number(c.lapso_3_ajustado) : null,
        nota_final: c.nota_final !== undefined ? Number(c.nota_final) : null,
      }));
      const resultado = await invoke('cargar_calificaciones_masivo', { calificaciones: payload });
      setResumen(resultado);
      mostrarMensaje('Carga masiva finalizada.', 'exito');
    } catch (err: any) {
      setResumen({ error: err });
      mostrarMensaje(err?.message || 'Error al cargar calificaciones', 'error');
    }
    setCargando(false);
  };

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-bold">Carga Masiva de Calificaciones</h2>
      <div className="flex items-center gap-4 mb-4">
        <input
          type="file"
          accept=".xlsx,.xls"
          ref={inputRef}
          onChange={handleFileChange}
          style={{ display: 'none' }}
        />
        <button
          type="button"
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          onClick={() => inputRef.current?.click()}
        >
          Cargar Excel
        </button>
        {nombreArchivo && (
          <span className="ml-2">{nombreArchivo}</span>
        )}
      </div>
      {erroresPrevios.length > 0 && (
        <div className="bg-red-50 border border-red-300 text-red-700 rounded-lg p-4 mb-2">
          <b>Errores críticos detectados:</b>
          <ul className="list-disc ml-6 text-sm">
            {erroresPrevios.map((err, idx) => <li key={idx}>{err}</li>)}
          </ul>
          <div className="mt-2 text-xs">Corrige estos errores en el archivo antes de continuar.</div>
        </div>
      )}
      {advertencias.length > 0 && (
        <div className="bg-yellow-50 border border-yellow-300 text-yellow-800 rounded-lg p-4 mb-2">
          <b>Advertencias:</b>
          <ul className="list-disc ml-6 text-sm">
            {advertencias.map((adv, idx) => <li key={idx}>{adv}</li>)}
          </ul>
          <div className="mt-2 text-xs">Puedes continuar, pero revisa estas advertencias.</div>
        </div>
      )}
      {calificaciones.length > 0 && erroresPrevios.length === 0 && (
        <div className="bg-gray-50 p-4 rounded-lg shadow">
          <p>
            <b>Registros listos para cargar:</b> {calificaciones.length}
          </p>
          <button
            onClick={handleEnviar}
            disabled={cargando}
            className="mt-2 px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700"
          >
            {cargando ? 'Cargando...' : 'Enviar al sistema'}
          </button>
        </div>
      )}
      {resumen && (
        <div className="bg-white p-4 rounded-lg shadow mt-4">
          <h3 className="font-bold mb-2">Resumen de la carga</h3>
          {resumen.error ? (
            <p className="text-red-600">Error: {String(resumen.error)}</p>
          ) : (
            <ul>
              <li><b>Total registros:</b> {resumen.total_registros}</li>
              <li><b>Insertados:</b> {resumen.insertados}</li>
              <li><b>Actualizados:</b> {resumen.actualizados}</li>
              <li><b>Errores:</b> {resumen.errores.length}</li>
            </ul>
          )}
          {resumen.errores && resumen.errores.length > 0 && (
            <div className="mt-2 max-h-40 overflow-y-auto">
              <b>Errores:</b>
              <ul className="text-xs text-red-500">
                {resumen.errores.map((err: string, idx: number) => (
                  <li key={idx}>{err}</li>
                ))}
              </ul>
            </div>
          )}
        </div>
      )}
    </div>
  );
} 