// eslint-disable-next-line @typescript-eslint/triple-slash-reference
/// <reference path="../types/env.d.ts" />

import React from "react";
import ReactDOM from "react-dom";
import "./core/sentry";
import App from "./App";

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);
