import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { RUTAS } from '../../../configuracion/rutas';
import { useStore } from '../../../estado/useStore';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '../../../componentes/ui/card';
import { Button } from '../../../componentes/ui/button';
import { cn } from '../../../utilidades/cn';
import { LogIn, Loader2 } from 'lucide-react';
import { autenticarUsuario } from '../../../servicios/autenticacion';

export function InicioSesion() {
  const navigate = useNavigate();
  const { iniciarSesion, establecerError, tema } = useStore();
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [cargando, setCargando] = useState(false);

  const manejarEnvio = async (e: React.FormEvent) => {
    e.preventDefault();
    setCargando(true);
    establecerError(null);

    try {
      const resultado = await autenticarUsuario({ username, password });
      
      if (!resultado) {
        establecerError('Credenciales inválidas. Por favor, verifica tu usuario y contraseña.');
        return;
      }

      iniciarSesion({
        usuario: resultado.usuario,
        token: resultado.token,
        expiracion: new Date(Date.now() + 24 * 60 * 60 * 1000) // 24 horas
      });

      navigate(RUTAS.DASHBOARD.PRINCIPAL);
    } catch (error) {
      establecerError('Error al iniciar sesión. Por favor, intenta nuevamente.');
      console.error('Error de autenticación:', error);
    } finally {
      setCargando(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-background p-4">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-1">
          <div className="flex items-center justify-center mb-4">
            <h1 className="text-3xl font-bold bg-gradient-to-r from-primary to-primary/80 bg-clip-text text-transparent">
              EduNet
            </h1>
          </div>
          <CardTitle className="text-2xl text-center">Bienvenido de nuevo</CardTitle>
          <CardDescription className="text-center">
            Ingresa tu usuario y contraseña para acceder a tu cuenta
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={manejarEnvio} className="space-y-4">
            <div className="space-y-2">
              <label
                htmlFor="username"
                className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
              >
                Usuario
              </label>
              <input
                id="username"
                type="text"
                required
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                className={cn(
                  "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background",
                  "file:border-0 file:bg-transparent file:text-sm file:font-medium",
                  "placeholder:text-muted-foreground",
                  "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
                  "disabled:cursor-not-allowed disabled:opacity-50"
                )}
                placeholder="Tu usuario"
              />
            </div>

            <div className="space-y-2">
              <label
                htmlFor="password"
                className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
              >
                Contraseña
              </label>
              <input
                id="password"
                type="password"
                required
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                className={cn(
                  "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background",
                  "file:border-0 file:bg-transparent file:text-sm file:font-medium",
                  "placeholder:text-muted-foreground",
                  "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
                  "disabled:cursor-not-allowed disabled:opacity-50"
                )}
                placeholder="••••••••"
              />
            </div>

            <Button
              type="submit"
              disabled={cargando}
              className="w-full"
            >
              {cargando ? (
                <>
                  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                  Iniciando sesión...
                </>
              ) : (
                <>
                  <LogIn className="mr-2 h-4 w-4" />
                  Iniciar Sesión
                </>
              )}
            </Button>
          </form>
        </CardContent>
        <CardFooter className="flex flex-col space-y-4">
          <div className="text-sm text-muted-foreground text-center">
            ¿Olvidaste tu contraseña?{' '}
            <a
              href="#"
              className="text-primary underline-offset-4 hover:underline"
            >
              Recuperar contraseña
            </a>
          </div>
        </CardFooter>
      </Card>
    </div>
  );
} 