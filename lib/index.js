var addon = require('../native/index.node');
const { Connection } = addon;
var conn = new Connection("file:test.db")
console.log(conn.all_test())
