"use strict";

let invokeCommand = function(arg) {
  window.external.invoke(JSON.stringify(arg));
}

let updateQuery = function(queryString) {
  invokeCommand({ command: "updateQuery", queryString: queryString });
}
