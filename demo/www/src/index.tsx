import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import GitHub from "@material-ui/icons/GitHub";
import { makeStyles, createStyles } from "@material-ui/styles";
import Container from "@material-ui/core/Container";
import Paper from "@material-ui/core/Paper";
import Grid from "@material-ui/core/Grid";
import TextField from "@material-ui/core/TextField";
import Typography from "@material-ui/core/Typography";
import Box from "@material-ui/core/Box";
import Link from "@material-ui/core/Link";
import LineTo from "react-lineto";

const repoURL = "https://github.com/tamuhey/tokenizations";
const repoWWWURL = "https://github.com/tamuhey/tokenizations/tree/master/demo";
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

const useStyles = makeStyles((theme) =>
  createStyles({
    textField: {
      fontSize: "1.3rem",
    },
  })
);

interface InputProps {
  text: string;
  setText: (text: string) => void;
  isError: boolean;
}
const Input = ({ text, setText, isError }: InputProps) => {
  const classes = useStyles();
  return (
    <Grid item xs={12}>
      <TextField
        value={text}
        onChange={(e) => setText(e.target.value)}
        error={isError}
        fullWidth
        InputProps={{
          classes: {
            input: classes.textField,
          },
        }}
      />
    </Grid>
  );
};

export const Index = () => {
  const [inputA, setInputA] = useState(`["John", "Johanson", "'s", "house"]`);
  const [inputB, setInputB] = useState(
    `["john", "johan", "##son", "'", "s", "house"]`
  );
  const [tokensA, errorA] = tryParse(inputA);
  const [tokensB, errorB] = tryParse(inputB);
  const [tokenization, setTokenization] = useState(null);
  const loadWasm = async () => setTokenization(await import("tokenization"));
  const classes = useStyles();

  useEffect(() => {
    loadWasm();
  });
  const [a2b, b2a]: number[][][] = tokenization
    ? tokenization.get_alignment(tokensA, tokensB)
    : [[], []];
  console.log(a2b);
  return (
    <Container maxWidth="md" style={{ marginTop: 20 }}>
      <Paper>
        <Box display="flex" justifyContent="center" m={3} alignItems="center">
          <Typography variant="h3">Tokenizations Demo</Typography>
          <Link href={repoURL} style={{ marginLeft: "20px" }}>
            <GitHub />
          </Link>
        </Box>
        <Grid container spacing={3} style={{ padding: "30px" }}>
          <Grid item xs={12}>
            <Typography>
              <Link href={repoURL}>Tokenization</Link> is a token alignment
              library for rust and Python. Feel free to change the below texts.
            </Typography>
          </Grid>
          <Grid item xs={12}>
            <Input text={inputA} setText={setInputA} isError={errorA} />
          </Grid>
          <Grid item xs={12}>
            <Input text={inputB} setText={setInputB} isError={errorB} />
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
                delay={100}
                key={`${i} ${j}`}
                from={`a${i}`}
                to={`b${j}`}
                zIndex={1}
                fromAnchor="bottom"
                toAnchor="top"
                borderColor="black"
              />
            ));
          })}
          <Grid item xs={12}>
            <Typography>
              This site is built with React and Wasm. The source is{" "}
              <Link href={repoWWWURL}>here</Link>.
            </Typography>
          </Grid>
        </Grid>
      </Paper>
    </Container>
  );
};

ReactDOM.render(<Index />, document.getElementById("container"));
