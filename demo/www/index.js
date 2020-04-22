"use strict";
exports.__esModule = true;
var wasm = require("tokenization");
var tokensAInput = document.getElementById("tokens_a");
var tokensBInput = document.getElementById("tokens_b");
var outputA = document.getElementById("outputA");
var outputB = document.getElementById("outputB");
var getTokens = function (elem) {
    return JSON.parse(elem.value);
};
var tokensToHTML = function (tokens) {
    return "<h2>" + tokens.map(function (t) { return "<span class=\"badge badge-light m-3\">" + t + "</span>"; }).join("") + "</h2>";
};
var update = function () {
    var tokensa = getTokens(tokensAInput);
    var tokensb = getTokens(tokensBInput);
    var _a = wasm.get_alignment(tokensa, tokensb), a2b = _a[0], b2a = _a[1];
    console.log(a2b);
    outputA.innerHTML = tokensToHTML(tokensa);
    outputB.innerHTML = tokensToHTML(tokensb);
};
tokensAInput.addEventListener("change", update);
tokensBInput.addEventListener("change", update);
update();
