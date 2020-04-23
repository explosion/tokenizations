import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import { ArcherContainer, ArcherElement } from "react-archer";
import {
  Container,
  Paper,
  Grid,
  TextField,
  Card,
  CardContent,
  Typography,
  Box,
} from "@material-ui/core";
import LineTo from "react-lineto";

const boxStyle = {
  padding: "10px",
  border: "1px solid black",
  borderRadius: "10px",
};
const tryParse = (input: string): [string[], boolean] => {
  try {
    const tokens = JSON.parse(input);
    return [tokens, false];
  } catch {
    return [[], true];
  }
};

export const Index = () => {
  const [inputA, setInputA] = useState('["fo","o","bar"]');
  const [inputB, setInputB] = useState('["foo","bar"]');
  const [tokensA, errorA] = tryParse(inputA);
  const [tokensB, errorB] = tryParse(inputB);
  const [tokenization, setTokenization] = useState(null);
  const loadWasm = async () => setTokenization(await import("tokenization"));

  useEffect(() => {
    loadWasm();
  });
  const [a2b, b2a]: number[][][] = tokenization
    ? tokenization.get_alignment(tokensA, tokensB)
    : [[], []];
  console.log(a2b);
  console.log(b2a);
  return (
    <Container>
      <Grid container>
        <Grid item xs={12}>
          <TextField
            value={inputA}
            onChange={(e) => setInputA(e.target.value)}
            error={errorA}
          />
        </Grid>
        <Grid item xs={12}>
          <TextField
            value={inputB}
            onChange={(e) => setInputB(e.target.value)}
            error={errorB}
          />
        </Grid>
        <div className="tokens">
          <Grid item xs={12}>
            <Box display="flex" bgcolor="background.paper" p={3} m={3}>
              {tokensA.map((token, i) => (
                <Box style={boxStyle} key={i} className={`a${i}`} m={1}>
                  <Typography>{token}</Typography>
                </Box>
              ))}
            </Box>
          </Grid>
          <Grid item xs={12}>
            <Box display="flex" bgcolor="background.paper" p={3} m={3}>
              {tokensB.map((token, i) => {
                return (
                  <Box style={boxStyle} key={i} className={`b${i}`} m={1}>
                    {token}
                  </Box>
                );
              })}
            </Box>
          </Grid>
        </div>
        {a2b.map((l, i) => {
          return l.map((j) => (
            <LineTo
              key={i * j}
              from={`a${i}`}
              to={`b${j}`}
              zIndex={1}
              fromAnchor="bottom"
              toAnchor="top"
              borderColor="black"
            />
          ));
        })}
      </Grid>
    </Container>
  );
};

ReactDOM.render(<Index />, document.getElementById("container"));
