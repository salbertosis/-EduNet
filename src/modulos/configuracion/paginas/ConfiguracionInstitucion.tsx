import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../../../componentes/ui/card';
import { Button } from '../../../componentes/ui/button';
import { Input } from '../../../componentes/ui/input';
import { Label } from '../../../componentes/ui/label';
import { Building2, Save, Loader2 } from 'lucide-react';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { obtenerDatosInstitucion, guardarDatosInstitucion, DatosInstitucion } from '../../../servicios/institucion';



export function ConfiguracionInstitucion() {
  const { mostrarMensaje } = useMensajeGlobal();
  const [cargando, setCargando] = useState(false);
  const [datos, setDatos] = useState<DatosInstitucion>({
    codigo: '',
    denominacion: '',
    direccion: '',
    telefono: '',
    municipio: '',
    entidad_federal: '',
    cdcee: '',
    director: '',
    cedula_director: ''
  });

  useEffect(() => {
    cargarDatosInstitucion();
  }, []);

  const cargarDatosInstitucion = async () => {
    try {
      setCargando(true);
      const respuesta = await obtenerDatosInstitucion();
      
      if (respuesta.exito && respuesta.datos) {
        setDatos(respuesta.datos);
        mostrarMensaje('Datos cargados exitosamente', 'exito');
      } else {
        mostrarMensaje(respuesta.mensaje, 'error');
      }
    } catch (error) {
      mostrarMensaje('Error al cargar los datos de la institución', 'error');
    } finally {
      setCargando(false);
    }
  };

  const handleGuardarDatos = async () => {
    try {
      console.log('🔵 [FRONTEND] Iniciando guardado de datos de institución...');
      console.log('🔵 [FRONTEND] Datos a guardar:', datos);
      
      setCargando(true);
      
      console.log('🔵 [FRONTEND] Llamando al servicio guardarDatosInstitucion...');
      const respuesta = await guardarDatosInstitucion(datos);
      
      console.log('🔵 [FRONTEND] Respuesta recibida:', respuesta);
      
      if (!respuesta) {
        console.log('❌ [FRONTEND] Respuesta es undefined');
        mostrarMensaje('Error: No se recibió respuesta del servidor', 'error');
        return;
      }
      
      if (respuesta.exito) {
        console.log('✅ [FRONTEND] Datos guardados exitosamente');
        mostrarMensaje('Datos de la institución guardados exitosamente', 'exito');
      } else {
        console.log('❌ [FRONTEND] Error en respuesta:', respuesta.mensaje);
        mostrarMensaje(respuesta.mensaje || 'Error desconocido', 'error');
      }
    } catch (error) {
      console.error('💥 [FRONTEND] Error capturado:', error);
      mostrarMensaje('Error al guardar los datos de la institución', 'error');
    } finally {
      setCargando(false);
      console.log('🔵 [FRONTEND] Proceso de guardado finalizado');
    }
  };

  const handleInputChange = (campo: keyof DatosInstitucion, valor: string) => {
    setDatos(prev => ({
      ...prev,
      [campo]: valor
    }));
  };

  return (
    <div className="max-w-4xl mx-auto p-6">
      <Card className="shadow-lg">
        <CardHeader className="bg-gradient-to-r from-emerald-500 to-cyan-500 text-white">
          <div className="flex items-center space-x-3">
            <Building2 className="w-8 h-8" />
            <div>
              <CardTitle className="text-2xl">Configuración de la Institución</CardTitle>
              <CardDescription className="text-emerald-100">
                Gestiona los datos oficiales de la institución educativa
              </CardDescription>
            </div>
          </div>
        </CardHeader>
        
        <CardContent className="p-6">
          <form onSubmit={(e) => { e.preventDefault(); handleGuardarDatos(); }}>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              
              {/* Código de la Institución */}
              <div className="space-y-2">
                <Label htmlFor="codigo">Código de la Institución Educativa *</Label>
                <Input
                  id="codigo"
                  value={datos.codigo}
                  onChange={(e) => handleInputChange('codigo', e.target.value)}
                  placeholder="Ej: S1371D0310"
                  maxLength={20}
                  required
                />
              </div>

              {/* Denominación y Epónimo */}
              <div className="space-y-2">
                <Label htmlFor="denominacion">Denominación y Epónimo *</Label>
                <Input
                  id="denominacion"
                  value={datos.denominacion}
                  onChange={(e) => handleInputChange('denominacion', e.target.value)}
                  placeholder="Nombre completo de la institución"
                  required
                />
              </div>

              {/* Dirección */}
              <div className="space-y-2 md:col-span-2">
                <Label htmlFor="direccion">Dirección *</Label>
                <Input
                  id="direccion"
                  value={datos.direccion}
                  onChange={(e) => handleInputChange('direccion', e.target.value)}
                  placeholder="Dirección completa de la institución"
                  required
                />
              </div>

              {/* Teléfono */}
              <div className="space-y-2">
                <Label htmlFor="telefono">Teléfono</Label>
                <Input
                  id="telefono"
                  value={datos.telefono}
                  onChange={(e) => handleInputChange('telefono', e.target.value)}
                  placeholder="Ej: 04148473484"
                  maxLength={20}
                />
              </div>

              {/* Municipio */}
              <div className="space-y-2">
                <Label htmlFor="municipio">Municipio *</Label>
                <Input
                  id="municipio"
                  value={datos.municipio}
                  onChange={(e) => handleInputChange('municipio', e.target.value)}
                  placeholder="Ej: GUANIPA"
                  maxLength={25}
                  required
                />
              </div>

              {/* Entidad Federal */}
              <div className="space-y-2">
                <Label htmlFor="entidad_federal">Entidad Federal *</Label>
                <Input
                  id="entidad_federal"
                  value={datos.entidad_federal}
                  onChange={(e) => handleInputChange('entidad_federal', e.target.value)}
                  placeholder="Ej: ANZOATEGUI"
                  maxLength={15}
                  required
                />
              </div>

              {/* CDCEE */}
              <div className="space-y-2">
                <Label htmlFor="cdcee">CDCEE</Label>
                <Input
                  id="cdcee"
                  value={datos.cdcee}
                  onChange={(e) => handleInputChange('cdcee', e.target.value)}
                  placeholder="Centro de Desarrollo de la Calidad Educativa"
                />
              </div>

              {/* Director */}
              <div className="space-y-2">
                <Label htmlFor="director">Director(a) *</Label>
                <Input
                  id="director"
                  value={datos.director}
                  onChange={(e) => handleInputChange('director', e.target.value)}
                  placeholder="Apellidos y Nombres del Director"
                  required
                />
              </div>

              {/* Cédula del Director */}
              <div className="space-y-2">
                <Label htmlFor="cedula_director">Cédula de Identidad del Director *</Label>
                <Input
                  id="cedula_director"
                  value={datos.cedula_director}
                  onChange={(e) => handleInputChange('cedula_director', e.target.value)}
                  placeholder="Ej: 14956886"
                  maxLength={10}
                  required
                />
              </div>

            </div>

            {/* Botones de acción */}
            <div className="flex justify-end space-x-4 mt-8 pt-6 border-t border-gray-200">
              <Button
                type="button"
                variant="outline"
                onClick={cargarDatosInstitucion}
                disabled={cargando}
              >
                {cargando ? (
                  <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                ) : (
                  <Building2 className="w-4 h-4 mr-2" />
                )}
                Cargar Datos
              </Button>
              
              <Button
                type="submit"
                disabled={cargando}
                className="bg-emerald-600 hover:bg-emerald-700"
              >
                {cargando ? (
                  <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                ) : (
                  <Save className="w-4 h-4 mr-2" />
                )}
                Guardar Configuración
              </Button>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  );
} 