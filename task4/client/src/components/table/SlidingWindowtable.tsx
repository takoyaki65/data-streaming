import type { FC } from "react";
import type { WindowDataType } from "../../model/slidingWindowData";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Paper from "@mui/material/Paper";

type Props = {
  result: WindowDataType[];
};

const SlidingWindowTable: FC<Props> = (props) => {
  return (
    <div>
      <h1>Sliding Window Table</h1>
      <TableContainer component={Paper}>
        <Table sx={{ minWidth: 650 }} aria-label="simple table">
          <TableHead>
            <TableRow>
              <TableCell align="left">Stock</TableCell>
              <TableCell align="left">Open Stock Price</TableCell>
              <TableCell align="left">Highest Stock Price</TableCell>
              <TableCell align="left">Lowest Stock Price</TableCell>
              <TableCell align="left">Close Stock Price</TableCell>
              <TableCell align="left">Timestamp</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {props.result.map((window_data) => (
              <TableRow
                key={window_data.id}
                sx={{ "&:last-child td, &:last-child th": { border: 0 } }}
              >
                <TableCell component="th" scope="row">
                  {window_data.stock_data.stock}
                </TableCell>
                <TableCell align="left">
                  {window_data.stock_data.open}
                </TableCell>
                <TableCell align="left">
                  {window_data.stock_data.high}
                </TableCell>
                <TableCell align="left">{window_data.stock_data.low}</TableCell>
                <TableCell align="left">
                  {window_data.stock_data.close}
                </TableCell>
                <TableCell align="left">
                  {window_data.stock_data.timestamp}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </div>
  );
};

export default SlidingWindowTable;
