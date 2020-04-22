import * as wasm from "tokenization";

const tokensAInput = document.getElementById("tokens_a") as HTMLInputElement;
const tokensBInput = document.getElementById("tokens_b") as HTMLInputElement;
const outputA = document.getElementById("outputA");
const outputB = document.getElementById("outputB");

const getTokens = (elem: HTMLInputElement): string[]=> {
    return JSON.parse(elem.value)
}

const tokensToHTML = (tokens: string[]): string => {
    return "<h2>" + tokens.map(t => `<span class="badge badge-light m-3">${t}</span>`).join("") + "</h2>"
}

const update = () => {
    const tokensa = getTokens(tokensAInput)
    const tokensb = getTokens(tokensBInput)
    const [a2b,b2a]= wasm.get_alignment(tokensa, tokensb)
    console.log(a2b)
    outputA.innerHTML = tokensToHTML(tokensa)
    outputB.innerHTML = tokensToHTML(tokensb)
}
tokensAInput.addEventListener("change", update);
tokensBInput.addEventListener("change", update);
update()

