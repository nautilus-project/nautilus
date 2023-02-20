import assert from "assert"
import { describe, it } from "mocha"
import { Nautilus } from "../src"
import { CONNECTION, PROGRAM_ID } from "./main.test"

export function tests() {

    describe("[Unit Tests]:   SQL Parsing", () => {

        const nautilus = new Nautilus(CONNECTION, PROGRAM_ID);

        function testParseSql(input: string) {
            it(`   -- Can parse:    ${input}`, () => assert(input = nautilus.sql(input).dumpSql()))
        }

        testParseSql(
            "SELECT * FROM person"
        )
        testParseSql(
            "SELECT id, name FROM person"
        )
        testParseSql(
            "SELECT * FROM person WHERE name = 'Joe'"
        )
        testParseSql(
            "SELECT (id, name) FROM person WHERE name = 'Joe'"
        )
        testParseSql(
            "SELECT * FROM person WHERE id = 1 AND name = 'Joe'"
        )
        testParseSql(
            "SELECT (id, name) FROM person WHERE id = 1 AND name = 'Joe'"
        )
        testParseSql(
            "SELECT * FROM person WHERE id = 1 AND name = 'Joe' AND authority = 'Joe'"
        )
        testParseSql(
            "SELECT (id, name) FROM person WHERE id = 1 AND name = 'Joe' AND authority = 'Joe'"
        )
        testParseSql(
            "SELECT * FROM person ORDER BY name ASC"
        )
        testParseSql(
            "SELECT (id, name) FROM person ORDER BY name ASC"
        )
        testParseSql(
            "SELECT * FROM person WHERE name = 'Joe' ORDER BY name ASC"
        )
        testParseSql(
            "SELECT (id, name) FROM person WHERE name = 'Joe' ORDER BY name ASC"
        )
        testParseSql(
            "SELECT * FROM person WHERE id = 1 AND name = 'Joe' ORDER BY name ASC"
        )
        testParseSql(
            "SELECT (id, name) FROM person WHERE id = 1 AND name = 'Joe' ORDER BY name ASC"
        )
        testParseSql(
            "SELECT * FROM person ORDER BY id DESC, name ASC"
        )
        testParseSql(
            "SELECT (id, name) FROM person ORDER BY id DESC, name ASC"
        )
        testParseSql(
            "SELECT * FROM person WHERE name = 'Joe' ORDER BY id DESC, name ASC"
        )
        testParseSql(
            "SELECT (id, name) FROM person WHERE name = 'Joe' ORDER BY id DESC, name ASC"
        )
        testParseSql(
            "SELECT * FROM person WHERE id = 1 AND name = 'Joe' ORDER BY id DESC, name ASC"
        )
        testParseSql(
            "SELECT (id, name) FROM person WHERE id = 1 AND name = 'Joe' ORDER BY id DESC, name ASC"
        )
        testParseSql(
            "SELECT id, name FROM person; SELECT id, name from heroes"
        )
        testParseSql(
            "INSERT INTO person VALUES ('Paul', 'none')"
        )
        testParseSql(
            "INSERT INTO person (name, authority) VALUES ('Paul', 'none')"
        )
        testParseSql(
            "INSERT INTO person VALUES ('Paul', 'none'), ('John', 'none')"
        )
        testParseSql(
            "INSERT INTO person (name, authority) VALUES ('Paul', 'none'), ('John', 'none')"
        )
        // Can un-comment when autoincrement config comes from IDL
        //
        // testParseSql(
        //     "INSERT INTO person VALUES (3, 'Paul', 'none')"
        // )
        // testParseSql(
        //     "INSERT INTO person (id, name, authority) VALUES (3, 'Paul', 'none')"
        // )
        // testParseSql(
        //     "INSERT INTO person VALUES (3, 'Paul', 'none'), (4, 'John', 'none')"
        // )
        // testParseSql(
        //     "INSERT INTO person (id, name, authority) VALUES (3, 'Paul', 'none'), (4, 'John', 'none')"
        // )
        //
        testParseSql(
            "DELETE FROM person"
        )
        testParseSql(
            "DELETE FROM person WHERE name = 'Joe'"
        )
        testParseSql(
            "DELETE FROM person WHERE id = 1 AND name = 'Joe'"
        )
        testParseSql(
            "UPDATE person SET name = 'Paul' WHERE id = 1"
        )
        testParseSql(
            "UPDATE person SET name = 'Paul' WHERE name = 'Joe'"
        )
        testParseSql(
            "UPDATE person SET name = 'Paul' WHERE id = 1 AND name = 'Joe'"
        )
        testParseSql(
            "UPDATE person SET name = 'Paul', authority = 'none' WHERE id = 1"
        )
        testParseSql(
            "UPDATE person SET name = 'Paul', authority = 'none' WHERE name = 'Joe'"
        )
        testParseSql(
            "UPDATE person SET name = 'Paul', authority = 'none' WHERE id = 1 AND name = 'Joe'"
        )
    })

}