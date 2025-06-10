import React, { useRef, useCallback, useMemo, useTransition } from 'react';
import * as XLSX from 'xlsx';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

// Types y interfaces
interface Calificacion {
  id_estudiante: number;
  id_asignatura: number;
  id_periodo: number;
  lapso_1?: number | null;
  lapso_2?: number | null;
  lapso_3?: number | null;
  revision?: number | null;
  lapso_1_ajustado?: number | null;
  lapso_2_ajustado?: number | null;
  lapso_3_ajustado?: number | null;
  nota_final?: number | null;
}

interface RawCalificacion extends Record<string, unknown> {
  id_estudiante?: unknown;
  id_asignatura?: unknown;
  id_periodo?: unknown;
  lapso_1?: unknown;
  lapso_2?: unknown;
  lapso_3?: unknown;
  revision?: unknown;
  lapso_1_ajustado?: unknown;
  lapso_2_ajustado?: unknown;
  lapso_3_ajustado?: unknown;
  nota_final?: unknown;
}

interface ValidationResult {
  valid: Calificacion[];
  warnings: string[];
  errors: string[];
}

interface FileProcessingResult {
  name: string;
  validCount: number;
  warnings: string[];
  errors: string[];
}

interface ProcessingState {
  calificaciones: Calificacion[];
  warnings: string[];
  errors: string[];
  fileName: string;
  filesSummary: FileProcessingResult[];
  summary: ProcessingSummary | null;
}

interface ProcessingSummary {
  total_registros: number;
  insertados: number;
  actualizados: number;
  errores: string[];
}

// Constants
const GRADE_FIELDS = [
  'lapso_1', 'lapso_2', 'lapso_3', 
  'lapso_1_ajustado', 'lapso_2_ajustado', 'lapso_3_ajustado',
  'nota_final', 'revision'
] as const;

const MIN_GRADE = 0;
const MAX_GRADE = 20;

// Custom hooks
const useProcessingState = () => {
  const [state, setState] = React.useState<ProcessingState>({
    calificaciones: [],
    warnings: [],
    errors: [],
    fileName: '',
    filesSummary: [],
    summary: null,
  });

  const [isLoading, setIsLoading] = React.useState(false);
  const [isPending, startTransition] = useTransition();

  const updateState = useCallback((updates: Partial<ProcessingState>) => {
    startTransition(() => {
      setState(prev => ({ ...prev, ...updates }));
    });
  }, []);

  const resetState = useCallback(() => {
    updateState({
      calificaciones: [],
      warnings: [],
      errors: [],
      fileName: '',
      filesSummary: [],
      summary: null,
    });
  }, [updateState]);

  return {
    state,
    updateState,
    resetState,
    isLoading,
    setIsLoading,
    isPending,
  };
};

// Utility functions
const isValidNumber = (value: unknown): value is number => {
  return typeof value === 'number' && !isNaN(value) && isFinite(value);
};

const parseNumberSafely = (value: unknown): number | null => {
  if (value === undefined || value === null || value === '') return null;
  const parsed = Number(value);
  return isValidNumber(parsed) ? parsed : null;
};

const isValidId = (value: unknown): boolean => {
  const parsed = parseNumberSafely(value);
  return parsed !== null && parsed > 0 && Number.isInteger(parsed);
};

const isValidGrade = (value: unknown): boolean => {
  if (value === undefined || value === null || value === '') return true;
  const parsed = parseNumberSafely(value);
  return parsed !== null && parsed >= MIN_GRADE && parsed <= MAX_GRADE;
};

// Validation logic
const validateCalificaciones = (rows: RawCalificacion[]): ValidationResult => {
  const warnings: string[] = [];
  const errors: string[] = [];
  const seenIds = new Set<string>();
  
  const valid = rows.reduce<Calificacion[]>((acc, row, index) => {
    const rowNumber = index + 2; // Excel row number (header + 1-based)
    let isRowValid = true;

    // Validate required fields
    const requiredFields = ['id_estudiante', 'id_asignatura', 'id_periodo'];
    for (const field of requiredFields) {
      if (!row[field]) {
        errors.push(`Fila ${rowNumber}: Campo obligatorio '${field}' faltante`);
        isRowValid = false;
      }
    }

    // Validate IDs
    if (!isValidId(row.id_estudiante)) {
      errors.push(`Fila ${rowNumber}: id_estudiante debe ser un número entero positivo`);
      isRowValid = false;
    }
    if (!isValidId(row.id_asignatura)) {
      errors.push(`Fila ${rowNumber}: id_asignatura debe ser un número entero positivo`);
      isRowValid = false;
    }
    if (!isValidId(row.id_periodo)) {
      errors.push(`Fila ${rowNumber}: id_periodo debe ser un número entero positivo`);
      isRowValid = false;
    }

    // Validate grades
    for (const field of GRADE_FIELDS) {
      if (!isValidGrade(row[field])) {
        warnings.push(`Fila ${rowNumber}: '${field}' debe estar entre ${MIN_GRADE} y ${MAX_GRADE}`);
      }
    }

    // Check for duplicates
    if (isRowValid) {
      const key = `${row.id_estudiante}-${row.id_asignatura}-${row.id_periodo}`;
      if (seenIds.has(key)) {
        warnings.push(`Fila ${rowNumber}: Registro duplicado (estudiante/asignatura/periodo)`);
      } else {
        seenIds.add(key);
      }

      // Transform to valid Calificacion
      const calificacion: Calificacion = {
        id_estudiante: Number(row.id_estudiante),
        id_asignatura: Number(row.id_asignatura),
        id_periodo: Number(row.id_periodo),
        lapso_1: parseNumberSafely(row.lapso_1),
        lapso_2: parseNumberSafely(row.lapso_2),
        lapso_3: parseNumberSafely(row.lapso_3),
        revision: parseNumberSafely(row.revision),
        lapso_1_ajustado: parseNumberSafely(row.lapso_1_ajustado),
        lapso_2_ajustado: parseNumberSafely(row.lapso_2_ajustado),
        lapso_3_ajustado: parseNumberSafely(row.lapso_3_ajustado),
        nota_final: parseNumberSafely(row.nota_final),
      };

      acc.push(calificacion);
    }

    return acc;
  }, []);

  return { valid, warnings, errors };
};

// File processing
const processExcelFile = async (file: File): Promise<ValidationResult & { fileName: string }> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    
    reader.onerror = () => reject(new Error(`Error leyendo archivo: ${file.name}`));
    
    reader.onload = (event) => {
      try {
        const data = event.target?.result;
        if (!data) throw new Error('No se pudo leer el archivo');

        const workbook = XLSX.read(data, { type: 'binary' });
        
        // Find the CARGA_MASIVA sheet (case insensitive)
        const sheetName = workbook.SheetNames.find(name => 
          name.toUpperCase().includes('CARGA_MASIVA')
        ) || workbook.SheetNames[0];

        if (!sheetName) throw new Error('No se encontraron hojas en el archivo');

        const sheet = workbook.Sheets[sheetName];
        const rows = XLSX.utils.sheet_to_json(sheet) as RawCalificacion[];
        
        const result = validateCalificaciones(rows);
        resolve({ ...result, fileName: file.name });
      } catch (error) {
        reject(error instanceof Error ? error : new Error(String(error)));
      }
    };

    reader.readAsBinaryString(file);
  });
};

// Main component
export function CargaMasivaCalificaciones() {
  const singleFileRef = useRef<HTMLInputElement>(null);
  const multipleFilesRef = useRef<HTMLInputElement>(null);
  const { mostrarMensaje } = useMensajeGlobal();
  
  const {
    state,
    updateState,
    resetState,
    isLoading,
    setIsLoading,
    isPending,
  } = useProcessingState();

  // Handlers
  const handleSingleFile = useCallback(async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    setIsLoading(true);
    resetState();

    try {
      const result = await processExcelFile(file);
      
      updateState({
        calificaciones: result.valid,
        warnings: result.warnings,
        errors: result.errors,
        fileName: file.name,
      });

      if (result.errors.length > 0) {
        mostrarMensaje('Errores críticos detectados. Revisa el archivo.', 'error');
      } else if (result.warnings.length > 0) {
        mostrarMensaje('Advertencias encontradas. Revisa antes de continuar.', 'advertencia');
      } else {
        mostrarMensaje('Archivo cargado exitosamente.', 'exito');
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Error desconocido';
      updateState({ errors: [message] });
      mostrarMensaje(`Error procesando archivo: ${message}`, 'error');
    } finally {
      setIsLoading(false);
      if (singleFileRef.current) singleFileRef.current.value = '';
    }
  }, [updateState, resetState, setIsLoading, mostrarMensaje]);

  const handleMultipleFiles = useCallback(async (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(event.target.files || []);
    if (files.length === 0) return;

    setIsLoading(true);
    resetState();

    try {
      // Process all files concurrently
      const results = await Promise.allSettled(
        files.map(file => processExcelFile(file))
      );

      const allCalificaciones: Calificacion[] = [];
      const allWarnings: string[] = [];
      const allErrors: string[] = [];
      const filesSummary: FileProcessingResult[] = [];

      results.forEach((result, index) => {
        const fileName = files[index].name;

        if (result.status === 'fulfilled') {
          const { valid, warnings, errors } = result.value;
          
          allCalificaciones.push(...valid);
          allWarnings.push(...warnings.map(w => `[${fileName}] ${w}`));
          allErrors.push(...errors.map(e => `[${fileName}] ${e}`));
          
          filesSummary.push({
            name: fileName,
            validCount: valid.length,
            warnings,
            errors,
          });
        } else {
          const errorMsg = `Error procesando archivo: ${result.reason}`;
          allErrors.push(`[${fileName}] ${errorMsg}`);
          
          filesSummary.push({
            name: fileName,
            validCount: 0,
            warnings: [],
            errors: [errorMsg],
          });
        }
      });

      updateState({
        calificaciones: allCalificaciones,
        warnings: allWarnings,
        errors: allErrors,
        fileName: `${files.length} archivos procesados`,
        filesSummary,
      });

      // Show appropriate message
      if (allErrors.length > 0) {
        mostrarMensaje('Errores encontrados en algunos archivos.', 'error');
      } else if (allWarnings.length > 0) {
        mostrarMensaje('Advertencias encontradas. Revisa antes de continuar.', 'advertencia');
      } else {
        mostrarMensaje('Todos los archivos procesados exitosamente.', 'exito');
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Error desconocido';
      updateState({ errors: [message] });
      mostrarMensaje(`Error procesando archivos: ${message}`, 'error');
    } finally {
      setIsLoading(false);
      if (multipleFilesRef.current) multipleFilesRef.current.value = '';
    }
  }, [updateState, resetState, setIsLoading, mostrarMensaje]);

  const handleSubmit = useCallback(async () => {
    if (state.calificaciones.length === 0) return;

    setIsLoading(true);

    try {
      const result = await invoke('cargar_calificaciones_masivo', {
        calificaciones: state.calificaciones,
      }) as ProcessingSummary;

      updateState({ summary: result });
      mostrarMensaje('Carga masiva completada exitosamente.', 'exito');
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : String(error);
      updateState({ 
        summary: { 
          total_registros: 0, 
          insertados: 0, 
          actualizados: 0, 
          errores: [errorMsg] 
        } 
      });
      mostrarMensaje(`Error en la carga: ${errorMsg}`, 'error');
    } finally {
      setIsLoading(false);
    }
  }, [state.calificaciones, updateState, setIsLoading, mostrarMensaje]);

  // Computed values
  const canSubmit = useMemo(() => 
    state.calificaciones.length > 0 && 
    state.errors.length === 0 && 
    !isLoading && 
    !isPending
  , [state.calificaciones.length, state.errors.length, isLoading, isPending]);

  const totalStats = useMemo(() => ({
    files: state.filesSummary.length,
    validRecords: state.filesSummary.reduce((sum, file) => sum + file.validCount, 0),
    warnings: state.filesSummary.reduce((sum, file) => sum + file.warnings.length, 0),
    errors: state.filesSummary.reduce((sum, file) => sum + file.errors.length, 0),
  }), [state.filesSummary]);

  return (
    <div className="space-y-6">
      <header>
        <h2 className="text-xl font-bold text-gray-900 dark:text-emerald-300">Carga Masiva de Calificaciones</h2>
      </header>

      {/* File Input Section */}
      <section className="flex flex-col sm:flex-row items-start sm:items-center gap-4">
        <input
          ref={singleFileRef}
          type="file"
          accept=".xlsx,.xls"
          onChange={handleSingleFile}
          disabled={isLoading}
          className="hidden"
          aria-label="Seleccionar archivo Excel individual"
        />
        
        <button
          type="button"
          onClick={() => singleFileRef.current?.click()}
          disabled={isLoading}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          Cargar Excel
        </button>

        <input
          ref={multipleFilesRef}
          type="file"
          accept=".xlsx,.xls"
          multiple
          onChange={handleMultipleFiles}
          disabled={isLoading}
          className="hidden"
          aria-label="Seleccionar múltiples archivos Excel"
        />
        
        <button
          type="button"
          onClick={() => multipleFilesRef.current?.click()}
          disabled={isLoading}
          className="px-4 py-2 bg-cyan-600 text-white rounded-lg hover:bg-cyan-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          Cargar múltiples Excel
        </button>

        {state.fileName && (
          <span className="text-gray-600 dark:text-cyan-300 font-medium">
            {state.fileName}
          </span>
        )}

        {(isLoading || isPending) && (
          <div className="flex items-center gap-2 text-blue-600">
            <div className="w-4 h-4 border-2 border-blue-600 border-t-transparent rounded-full animate-spin" />
            <span className="text-sm">Procesando...</span>
          </div>
        )}
      </section>

      {/* Errors Section */}
      {state.errors.length > 0 && (
        <section className="bg-red-50 border border-red-200 rounded-lg p-4">
          <h3 className="font-semibold text-red-800 mb-2">
            Errores críticos detectados
          </h3>
          <ul className="list-disc list-inside space-y-1 text-sm text-red-700 max-h-40 overflow-y-auto">
            {state.errors.map((error, index) => (
              <li key={index}>{error}</li>
            ))}
          </ul>
          <p className="mt-2 text-xs text-red-600">
            Corrige estos errores antes de continuar.
          </p>
        </section>
      )}

      {/* Warnings Section */}
      {state.warnings.length > 0 && (
        <section className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
          <h3 className="font-semibold text-yellow-800 mb-2">Advertencias</h3>
          <ul className="list-disc list-inside space-y-1 text-sm text-yellow-700 max-h-40 overflow-y-auto">
            {state.warnings.map((warning, index) => (
              <li key={index}>{warning}</li>
            ))}
          </ul>
          <p className="mt-2 text-xs text-yellow-600">
            Puedes continuar, pero revisa estas advertencias.
          </p>
        </section>
      )}

      {/* Ready to Submit Section */}
      {canSubmit && (
        <section className="bg-green-50 dark:bg-neutral-900 border border-green-200 dark:border-neutral-700 rounded-lg p-4 shadow-lg">
          <p className="text-green-800 dark:text-white font-medium mb-3">
            <strong>Registros listos para cargar:</strong> {state.calificaciones.length}
          </p>
          <button
            onClick={handleSubmit}
            disabled={!canSubmit}
            className="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isLoading ? 'Enviando...' : 'Enviar al sistema'}
          </button>
        </section>
      )}

      {/* Processing Summary */}
      {state.summary && (
        <section className="bg-white border border-gray-200 rounded-lg p-4 shadow-sm">
          <h3 className="font-semibold text-gray-900 mb-3">Resumen de la carga</h3>
          
          {state.summary.errores.length > 0 ? (
            <div className="text-red-600">
              <p className="font-medium">Error en el procesamiento</p>
              <ul className="mt-2 text-sm list-disc list-inside">
                {state.summary.errores.map((error, index) => (
                  <li key={index}>{error}</li>
                ))}
              </ul>
            </div>
          ) : (
            <dl className="grid grid-cols-2 gap-4 text-sm">
              <div>
                <dt className="font-medium text-gray-600">Total registros:</dt>
                <dd className="text-gray-900">{state.summary.total_registros}</dd>
              </div>
              <div>
                <dt className="font-medium text-gray-600">Insertados:</dt>
                <dd className="text-green-600">{state.summary.insertados}</dd>
              </div>
              <div>
                <dt className="font-medium text-gray-600">Actualizados:</dt>
                <dd className="text-blue-600">{state.summary.actualizados}</dd>
              </div>
              <div>
                <dt className="font-medium text-gray-600">Errores:</dt>
                <dd className="text-red-600">{state.summary.errores.length}</dd>
              </div>
            </dl>
          )}
        </section>
      )}

      {/* Files Summary */}
      {state.filesSummary.length > 0 && (
        <section className="bg-white dark:bg-neutral-900 border border-gray-200 dark:border-neutral-700 rounded-lg p-4 shadow-lg dark:text-white">
          <h3 className="font-semibold text-cyan-700 dark:text-cyan-200 mb-3">
            Resumen de archivos procesados
          </h3>
          <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-4 text-sm">
            <div>
              <span className="font-medium text-gray-600 dark:text-gray-200">Archivos:</span>
              <span className="ml-1 text-gray-900 dark:text-white">{totalStats.files}</span>
            </div>
            <div>
              <span className="font-medium text-gray-600 dark:text-gray-200">Válidos:</span>
              <span className="ml-1 text-green-600 dark:text-green-300">{totalStats.validRecords}</span>
            </div>
            <div>
              <span className="font-medium text-gray-600 dark:text-gray-200">Advertencias:</span>
              <span className="ml-1 text-yellow-600 dark:text-yellow-200">{totalStats.warnings}</span>
            </div>
            <div>
              <span className="font-medium text-gray-600 dark:text-gray-200">Errores:</span>
              <span className="ml-1 text-red-600 dark:text-red-300">{totalStats.errors}</span>
            </div>
          </div>
          <div className="space-y-3 max-h-60 overflow-y-auto">
            {state.filesSummary.map((file, index) => (
              <div key={index} className="border-l-4 border-emerald-500 pl-4">
                <div className="flex flex-wrap items-center gap-2 text-sm">
                  <span className="font-medium text-emerald-700 dark:text-cyan-300">{file.name}</span>
                  <span className="text-gray-600 dark:text-gray-200">{file.validCount} válidos</span>
                  {file.warnings.length > 0 && (
                    <span className="px-2 py-1 bg-yellow-100 dark:bg-yellow-900 text-yellow-700 dark:text-yellow-200 rounded text-xs">
                      {file.warnings.length} advertencias
                    </span>
                  )}
                  {file.errors.length > 0 && (
                    <span className="px-2 py-1 bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200 rounded text-xs">
                      {file.errors.length} errores
                    </span>
                  )}
                </div>
                {file.errors.length > 0 && (
                  <ul className="mt-2 text-xs text-red-600 dark:text-red-200 list-disc list-inside">
                    {file.errors.map((error, i) => (
                      <li key={i}>{error}</li>
                    ))}
                  </ul>
                )}
                {file.warnings.length > 0 && (
                  <ul className="mt-2 text-xs text-yellow-700 dark:text-yellow-200 list-disc list-inside">
                    {file.warnings.map((warning, i) => (
                      <li key={i}>{warning}</li>
                    ))}
                  </ul>
                )}
              </div>
            ))}
          </div>
        </section>
      )}
    </div>
  );
}