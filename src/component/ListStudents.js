import "regenerator-runtime";
import React from "react";
import { useEffect, useState } from "react";

import getConfig from "../config";

const { networkId } = getConfig(process.env.NODE_ENV || "development");

export default function ListLessons() {
  const [students, setStudents] = useState([]);

  useEffect(() => {
    window.contract.list_students().then((res) => {
      setStudents(res);
    });
  }, []);
  function data() {
    return students.map((student) => {
      return (
        <tr key={student.user_id}>
          <td>{student.user_id}</td>
          <td>{new Date(1564732216488).toDateString()}</td>
          <td>{student.student_name}</td>
          <td>{student.school_name}</td>
        </tr>
      );
    });
  }
  return (
    <div>
      <h4>List of Students</h4>
      <table>
        <thead>
          <tr>
            <th>Student ID</th>
            <th>Logged minutes</th>
            <th>Student name</th>
            <th>School name</th>
          </tr>
        </thead>
        <tbody>{data()}</tbody>
      </table>
    </div>
  );
}
