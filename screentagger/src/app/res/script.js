"use strict";

let invokeCommand = function(arg) {
  window.external.invoke(JSON.stringify(arg));
}

let updateQuery = function(queryString) {
  invokeCommand({ command: "updateQuery", queryString: queryString });
}

let addFolder = function() {
  invokeCommand({ command: "addFolder" });
}

let revealFile = function(filePath) {
  invokeCommand({ command: "revealFile", filePath: filePath });
}

let makeThumbnail = function(file) {
  let thumbnail = document.createElement("div");
  thumbnail.setAttribute("class", "thumbnail");
  let img = document.createElement("img");
  img.setAttribute("src", file);
  img.onclick = function() { revealFile(file); }
  let p = document.createElement("p");
  p.innerText = file;

  thumbnail.appendChild(img).appendChild(p);

  return thumbnail;
}

let displayMatches = function(matches) {
  let container = document.getElementById("file-display-container");
  while (container.firstChild) {
    container.removeChild(container.lastChild);
  }
  matches.forEach(function (match) {
    let thumbnail = makeThumbnail(match);
    container.appendChild(thumbnail);
  });
}
