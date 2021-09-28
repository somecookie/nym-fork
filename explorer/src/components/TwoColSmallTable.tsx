import * as React from 'react';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';
import CheckCircleSharpIcon from '@mui/icons-material/CheckCircleSharp';
import { Typography } from '@mui/material';

interface TableProps {
    title?: string
    icons?: boolean
    keys: string[]
    values: number[]
    marginBottom?: boolean
}

export function TwoColSmallTable({ title, icons, keys, values, marginBottom }: TableProps) {
        return (
            <>
                {title && (
                    <Typography sx={{ marginTop: 2 }}>
                        {title}
                    </Typography>
                )}
                <TableContainer component={Paper} sx={ marginBottom ? { marginBottom: 4, marginTop: 2 } : { marginTop: 2 }}>
                    <Table aria-label='two col small table'>
                        <TableBody>
                            <TableRow>
                                { icons && <TableCell ><CheckCircleSharpIcon /></TableCell>}
                                <TableCell >Received</TableCell>
                                <TableCell >1789</TableCell>
                            </TableRow>
                            <TableRow>
                                { icons && <TableCell ><CheckCircleSharpIcon /></TableCell>}
                                <TableCell >Sent</TableCell>
                                <TableCell >1789</TableCell>
                            </TableRow>
                            <TableRow>
                                { icons && <TableCell ><CheckCircleSharpIcon /></TableCell>}
                                <TableCell >Explicitly dropped</TableCell>
                                <TableCell >1789</TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </TableContainer>
            </>
        );
}