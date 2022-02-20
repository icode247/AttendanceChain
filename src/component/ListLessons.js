import "regenerator-runtime";
import React from "react";
import { useEffect, useState } from "react";

import getConfig from "../config";

const { networkId } = getConfig(process.env.NODE_ENV || "development");

export default function ListLessons({ lessons }) {

  function data() {
    const approvedLessons = lessons.filter(
      (lesson) => lesson.is_approved === true
    );
    return approvedLessons.map((lesson) => {
      return (
        <tr key={lesson.id}>
          <td>{lesson.creator}</td>
          <td>{lesson.id}</td>
          <td>{lesson.subject}</td>
          <td>{lesson.task}</td>
          <td>{lesson.is_approved ? "True" : "False"}</td>
          <td>{lesson.is_complete ? "True" : "False"}</td>
        </tr>
      );
    });
  }
  return (
    <div>
      <h4>List of lessons</h4>
      <table>
        <thead>
          <tr>
            <th>Creator</th>
            <th>ID</th>
            <th>Subject</th>
            <th>Task</th>
            <th>Approved</th>
            <th>Complated</th>
          </tr>
        </thead>
        <tbody>{data()}</tbody>
      </table>
    </div>
  );
}
