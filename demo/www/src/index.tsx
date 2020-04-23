import React, { useState } from "react";
import ReactDOM from "react-dom";
import { ArcherContainer, ArcherElement } from "react-archer";
import { get_alignment } from "tokenization";
import {
  Container,
  Paper,
  Grid,
  TextField,
  Card,
  CardContent,
  Typography,
} from "@material-ui/core";

const boxStyle = { padding: "10px", border: "1px solid black" };
const tryParse = (input: string): [string[], boolean] => {
  try {
    const tokens = JSON.parse(input);
    return [tokens, false];
  } catch {
    return [[], true];
  }
};

export const Index = () => {
  const [inputA, setInputA] = useState("[]");
  const [inputB, setInputB] = useState("[]");
  const [tokensA, errorA] = tryParse(inputA);
  const [tokensB, errorB] = tryParse(inputB);
  const [a2b, b2a] = get_alignment(tokensA, tokensB);
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
        <ArcherContainer>
          <Grid item xs={12}>
            {tokensA.map((token, i) => {
              <ArcherElement id={`a${i}`}>
                <Card>
                  <CardContent>
                    <Typography>{token}</Typography>
                  </CardContent>
                </Card>
              </ArcherElement>;
            })}
          </Grid>
          <Grid item xs={12}></Grid>
        </ArcherContainer>
      </Grid>
    </Container>
  );
};

ReactDOM.render(<Index />, document.getElementById("container"));
