import { styled } from '@mui/material/styles'
import Avatar from '@mui/material/Avatar'
import MuiDrawer, { drawerClasses } from '@mui/material/Drawer'
import Box from '@mui/material/Box'
import Stack from '@mui/material/Stack'
import Typography from '@mui/material/Typography'
import MenuContent from '../MenuContent/MenuContent'
import OptionsMenu from '../OptionsMenu'
import useData from '../../hooks/useData'

const drawerWidth = 240

const Drawer = styled(MuiDrawer)({
    width: drawerWidth,
    flexShrink: 0,
    boxSizing: 'border-box',
    mt: 10,
    [`& .${drawerClasses.paper}`]: {
        width: drawerWidth,
        boxSizing: 'border-box',
    },
})

export default function SideMenu() {
    const [data, error] = useData('user')
    const user = data[0]

    return (
        <Drawer
            variant="permanent"
            sx={{
                display: { xs: 'none', md: 'block' },
                [`& .${drawerClasses.paper}`]: {
                    backgroundColor: 'background.paper',
                },
            }}
        >
            <MenuContent />
            <Stack
                direction="row"
                sx={{
                    p: 2,
                    gap: 1,
                    alignItems: 'center',
                    borderTop: '1px solid',
                    borderColor: 'divider',
                }}
            >
                {user && (
                    <>
                        <Avatar
                            sizes="small"
                            alt={`${user.first_name}`}
                            src="/static/images/avatar/7.jpg"
                            sx={{ width: 36, height: 36 }}
                        />
                        <Box sx={{ mr: 'auto' }}>
                            <Typography
                                variant="body2"
                                sx={{ fontWeight: 500, lineHeight: '16px' }}
                            >
                                {`${user.first_name} ${user.last_name}`}
                            </Typography>
                            <Typography
                                variant="caption"
                                sx={{ color: 'text.secondary' }}
                            >
                                {user.email}
                            </Typography>
                        </Box>
                    </>
                )}
                <OptionsMenu />
            </Stack>
        </Drawer>
    )
}
