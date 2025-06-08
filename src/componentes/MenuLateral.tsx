import React from 'react';
import { Link } from 'react-router-dom';
import { ListItem, ListItemIcon, ListItemText } from '@mui/material';
import DashboardIcon from '@mui/icons-material/Dashboard';
import SchoolIcon from '@mui/icons-material/School';

const MenuLateral = () => {
    return (
        <div>
            <ListItem button component={Link} to="/dashboard" selected={location.pathname === '/dashboard'}>
                <ListItemIcon>
                    <DashboardIcon />
                </ListItemIcon>
                <ListItemText primary="Dashboard" />
            </ListItem>
            <ListItem button component={Link} to="/cursos" selected={location.pathname === '/cursos'}>
                <ListItemIcon>
                    <SchoolIcon />
                </ListItemIcon>
                <ListItemText primary="Cursos" />
            </ListItem>
            <ListItem button component={Link} to="/estudiantes" selected={location.pathname === '/estudiantes'}>
                {/* Add the rest of the existing code here */}
            </ListItem>
        </div>
    );
};

export default MenuLateral; 