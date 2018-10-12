var MemoryFileSystem = require("memory-fs");
var fs = new MemoryFileSystem(); // Optionally pass a javascript object

(() => {
    'use strict';

    fs.mkdirpSync("/a/test/dir");
    fs.writeFileSync("/a/test/dir/file.txt", "Hello World");
    let data = fs.readFileSync("/a/test/dir/file.txt"); 

    console.log(data.toString());
})();

