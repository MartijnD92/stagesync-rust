import Grid from '@mui/material/Grid2'
import Box from '@mui/material/Box'
import Typography from '@mui/material/Typography'
import Copyright from '../Dashboard/internals/components/Copyright'
import CustomizedDataGrid from '../CustomizedDataGrid'

export default function MainGrid() {
    return (
        <Box sx={{ width: '100%', maxWidth: { sm: '100%', md: '1700px' } }}>
            <Typography component="h2" variant="h6" sx={{ mb: 2 }}>
                Artists
            </Typography>
            <Grid container spacing={2} columns={12}>
                <Grid size={{ xs: 12 }}>
                    <CustomizedDataGrid />
                </Grid>
            </Grid>
            <Copyright sx={{ my: 4 }} />
        </Box>
    )
}