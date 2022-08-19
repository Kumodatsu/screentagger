"use strict";

let invokeCommand = function(arg) {
  window.external.invoke(JSON.stringify(arg));
}

let updateQuery = function(queryString) {
  invokeCommand({ command: "updateQuery", queryString: queryString });
}

let displayMatches = function(matches) {
  let ul = document.getElementById("file-display-list");
  while (ul.firstChild) {
    ul.removeChild(ul.lastChild);
  }
  matches.forEach(function (match) {
    let li = document.createElement("li");
    li.innerHTML = match;
    ul.appendChild(li);
  });
}
