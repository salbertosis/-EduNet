import { Pool } from 'pg';

export const pool = new Pool({
  user: 'Salbertosis',
  password: 'salbertosis13497674',
  host: 'localhost',
  database: 'EduNet',
  port: 5432,
});

// Función para probar la conexión
export async function probarConexion() {
  try {
    const client = await pool.connect();
    console.log('Conexión exitosa a la base de datos');
    client.release();
    return true;
  } catch (error) {
    console.error('Error al conectar a la base de datos:', error);
    return false;
  }
} 