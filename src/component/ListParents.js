import "regenerator-runtime/runtime";
import React from "react";
import { useEffect, useState } from "react";

import getConfig from "../config";

const { networkId } = getConfig(process.env.NODE_ENV || "development");

export default function ListLessons() {
  const [parents, setParents] = useState([]);

  useEffect(() => {
    window.contract.list_parents().then((res) => {
      setParents(res);
    });
  }, []);
  function data() {
    return parents.map((parent) => {
      return (
        <tr key={parent.user_id}>
          <td>{parent.user_id}</td>
          <td>{new Date(parent.logged_minutes).toDateString()}</td>
        </tr>
      );
    });
  }
  return (
    <div>
      <h4>List of Parents</h4>
      <table>
        <thead>
          <tr>
            <th>Student ID</th>
            <th>Logged minutes</th>
          </tr>
        </thead>
        <tbody>{data()}</tbody>
      </table>
    </div>
  );
}
