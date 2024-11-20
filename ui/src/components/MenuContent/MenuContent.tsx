import React from 'react'
import { Link, useResolvedPath } from 'react-router-dom'

import List from '@mui/material/List'
import ListItem from '@mui/material/ListItem'
import ListItemButton from '@mui/material/ListItemButton'
import ListItemIcon from '@mui/material/ListItemIcon'
import ListItemText from '@mui/material/ListItemText'
import Stack from '@mui/material/Stack'
import DashboardRoundedIcon from '@mui/icons-material/DashboardRounded'
import MusicNoteRoundedIcon from '@mui/icons-material/MusicNoteRounded'
import EventNoteRoundedIcon from '@mui/icons-material/EventNoteRounded'
import PersonRoundedIcon from '@mui/icons-material/PersonRounded'
import SettingsRoundedIcon from '@mui/icons-material/SettingsRounded'
import InfoRoundedIcon from '@mui/icons-material/InfoRounded'

const mainListItems = [
    { text: 'Dashboard', icon: <DashboardRoundedIcon />, href: '/dashboard' },
    { text: 'Bookings', icon: <EventNoteRoundedIcon />, href: '/bookings' },
    { text: 'Artists', icon: <MusicNoteRoundedIcon />, href: '/artists' },
    { text: 'My profile', icon: <PersonRoundedIcon />, href: '/profile' },
]

const secondaryListItems = [
    { text: 'Settings', icon: <SettingsRoundedIcon /> },
    { text: 'About', icon: <InfoRoundedIcon /> },
]

export default function MenuContent() {
    const url = useResolvedPath('').pathname

    return (
        <Stack sx={{ flexGrow: 1, p: 1, justifyContent: 'space-between' }}>
            <List dense>
                {mainListItems.map((item, index) => (
                    <ListItem
                        key={index}
                        disablePadding
                        sx={{ display: 'block' }}
                    >
                        <ListItemButton
                            component={Link}
                            to={item.href}
                            selected={item.href === url}
                        >
                            <ListItemIcon>{item.icon}</ListItemIcon>
                            <ListItemText primary={item.text} />
                        </ListItemButton>
                    </ListItem>
                ))}
            </List>

            <List dense>
                {secondaryListItems.map((item, index) => (
                    <ListItem
                        key={index}
                        disablePadding
                        sx={{ display: 'block' }}
                    >
                        <ListItemButton>
                            <ListItemIcon>{item.icon}</ListItemIcon>
                            <ListItemText primary={item.text} />
                        </ListItemButton>
                    </ListItem>
                ))}
            </List>
        </Stack>
    )
}
