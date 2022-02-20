import "regenerator-runtime";
import { useEffect, useState } from "react";
import React from "react";
import ListLessons from "./component/ListLessons";
import ListStudents from "./component/ListStudents";
import ListParents from "./component/ListParents";
import { login, logout } from "./utils";
import "./global.css";

import getConfig from "./config";

const { networkId } = getConfig(process.env.NODE_ENV || "development");

export default function App() {
  // use React Hooks to store greeting in component state
  const [lessons, setAttendance] = useState([]);

  // The useEffect hook can be used to fire side-effects during render
  // Learn more: https://reactjs.org/docs/hooks-intro.html
  useEffect(
    () => {
      // in this case, we only care to query the contract when signed in
      if (window.walletConnection.isSignedIn()) {
        // window.contract is set by initContract in index.js
        window.contract.list_lessons().then((res) => {
          setAttendance(res);
        });
      }
    },
    []
  );

  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) {
    return (
      <main className="signin">
        <h1>Welcome to AttendanceChain</h1>
        <p style={{ textAlign: "center" }}>
          Click the button below to sign in:
        </p>
        <p style={{ textAlign: "center", marginTop: "2.5em" }}>
          <button onClick={login}>Sign in</button>
        </p>
      </main>
    );
  }
  return (
    // use React Fragment, <>, to avoid wrapping elements in unnecessary divs
    <>
      <header>
        <div className="logo"></div>
        <button className="link" style={{ float: "right" }} onClick={logout}>
          Sign out <span className="id">{window.accountId}</span>
        </button>
      </header>
      <main>
        <ListLessons lessons={lessons} />
        <ListStudents />
        <ListParents />
      </main>
    </>
  );
}
