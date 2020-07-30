import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import GitHub from "@material-ui/icons/GitHub";
import { makeStyles, createStyles, ThemeProvider } from "@material-ui/styles";
import { createMuiTheme, Theme } from "@material-ui/core/styles";
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
const tryParse = (input: string): [string[], boolean] => {
  try {
    const tokens = JSON.parse(input);
    return [tokens, false];
  } catch {
    return [[], true];
  }
};

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    textField: {
      fontSize: "1.3rem",
    },
    tokenBox: {
      padding: 10,
      border: "1px solid black",
      borderRadius: 10,
    },
    tokensContainer: {
      display: "flex",
      padding: theme.spacing(3),
      margin: theme.spacing(3),
      backgroundColor: theme.palette.background.paper,
    },
    titleBox: {
      display: "flex",
      justifyContent: "center",
      margin: 3,
      alignItems: "baseline",
    },
    githubIcon: {
      color: "black",
      marginLeft: 20,
    },
    gridContainer: {
      padding: 30,
    },
    container: {
      marginTop: 20,
    },
  })
);

interface InputProps {
  text: string;
  setText: (text: string) => void;
  error: boolean;
}

const theme = createMuiTheme();
const Index = () => (
  <ThemeProvider theme={theme}>
    <App />
  </ThemeProvider>
);

const App = () => {
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
  const [a2b]: number[][][] = tokenization
    ? tokenization.get_alignment(tokensA, tokensB)
    : [[], []];
  console.log(a2b);
  return (
    <Container maxWidth="md" className={classes.container}>
      <Paper>
        <Box className={classes.titleBox}>
          <Typography variant="h3">Tokenizations Demo</Typography>
          <Link href={repoURL}>
            <GitHub className={classes.githubIcon} />
          </Link>
        </Box>
        <Grid container spacing={3} className={classes.gridContainer}>
          <Grid item xs={12}>
            <Typography>
              <Link href={repoURL}>Tokenization</Link> is a token alignment
              library for rust and Python. Feel free to change the below texts.
            </Typography>
          </Grid>
          <Grid item xs={12}>
            <Input text={inputA} setText={setInputA} error={errorA} />
          </Grid>
          <Grid item xs={12}>
            <Input text={inputB} setText={setInputB} error={errorB} />
          </Grid>
          <div className="tokens">
            <Grid item xs={12}>
              <Box className={classes.tokensContainer}>
                {tokensA.map((token, i) => (
                  <Box key={i} className={`a${i} ` + classes.tokenBox} m={1}>
                    <Typography>{token}</Typography>
                  </Box>
                ))}
              </Box>
            </Grid>
            <Grid item xs={12}>
              <Box className={classes.tokensContainer}>
                {tokensB.map((token, i) => {
                  return (
                    <Box key={i} className={`b${i} ` + classes.tokenBox} m={1}>
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

const Input = ({ text, setText, error }: InputProps) => {
  const classes = useStyles();
  return (
    <TextField
      value={text}
      onChange={(e) => setText(e.target.value)}
      error={error}
      fullWidth
      InputProps={{
        classes: {
          input: classes.textField,
        },
      }}
      helperText={error ? "Invalid JSON array" : ""}
    />
  );
};

ReactDOM.render(<Index />, document.getElementById("container"));
