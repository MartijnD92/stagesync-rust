import { GridRowsProp, GridColDef } from '@mui/x-data-grid'
import { toTitleCase } from './string'

// Use this array to order the columns in the UI
const INCLUDED_KEYS = [
    'name', // this comment is only used to Prettier keeps elements on multiple lines
    'fee_with_currency',
    'next_gig',
    'owner_name',
]

const RENAMED_KEYS = [
    { old: 'fee_with_currency', new: 'Fee' },
    { old: 'owner_name', new: 'Owner' },
]

interface GridData {
    columns: GridColDef[]
    rows: GridRowsProp
}

export function mapDataToGrid<T extends object[]>(data: T): GridData {
    let columns: GridColDef[] = []
    const rows: GridRowsProp = data

    if (data.length) {
        columns = Object.keys(data[0]).reduce((acc, key) => {
            if (INCLUDED_KEYS.includes(key)) {
                const renameEntry = RENAMED_KEYS.find(
                    (item) => item.old === key
                )
                const headerName = renameEntry
                    ? renameEntry.new
                    : toTitleCase(key)

                const col = {
                    field: key,
                    headerName: headerName,
                    flex: 1.5,
                    minWidth: 200,
                }
                acc.push(col)
            }
            return acc
        }, [] as GridColDef[])

        // Sort columns based on INCLUDED_KEYS order
        columns.sort((a, b) => {
            return (
                INCLUDED_KEYS.indexOf(a.field) - INCLUDED_KEYS.indexOf(b.field)
            )
        })
    }

    return {
        columns,
        rows,
    }
}
