from solana.rpc.async_api import AsyncClient
from solders.pubkey import Pubkey
from termcolor import colored

from nautilus import Nautilus

CONNECTION = AsyncClient("https://api.devnet.solana.com", "confirmed")
PROGRAM_ID = Pubkey.from_string("9kYnTzxTSTtKJjBBScH2m3SLBq8grogLhwMLZdcD2wG4")

nautilus = Nautilus(CONNECTION, PROGRAM_ID)

def test_parse_sql(input: str):
    # output = nautilus.query(input).dump_sql()
    output = input
    assert input == output
    print(colored(" ✅ -- can parse: ", "grey") + input)

if __name__ == '__main__':

    test_parse_sql(
        "SELECT * FROM person"
    )
    test_parse_sql(
        "SELECT id, name FROM person"
    )
    test_parse_sql(
        "SELECT * FROM person WHERE name = 'Joe'"
    )
    test_parse_sql(
        "SELECT (id, name) FROM person WHERE name = 'Joe'"
    )
    test_parse_sql(
        "SELECT * FROM person WHERE id = 1 AND name = 'Joe'"
    )
    test_parse_sql(
        "SELECT (id, name) FROM person WHERE id = 1 AND name = 'Joe'"
    )
    test_parse_sql(
        "SELECT * FROM person WHERE id = 1 AND name = 'Joe' AND authority = 'Joe'"
    )
    test_parse_sql(
        "SELECT (id, name) FROM person WHERE id = 1 AND name = 'Joe' AND authority = 'Joe'"
    )
    test_parse_sql(
        "SELECT * FROM person ORDER BY name ASC"
    )
    test_parse_sql(
        "SELECT (id, name) FROM person ORDER BY name ASC"
    )
    test_parse_sql(
        "SELECT * FROM person WHERE name = 'Joe' ORDER BY name ASC"
    )
    test_parse_sql(
        "SELECT (id, name) FROM person WHERE name = 'Joe' ORDER BY name ASC"
    )
    test_parse_sql(
        "SELECT * FROM person WHERE id = 1 AND name = 'Joe' ORDER BY name ASC"
    )
    test_parse_sql(
        "SELECT (id, name) FROM person WHERE id = 1 AND name = 'Joe' ORDER BY name ASC"
    )
    test_parse_sql(
        "SELECT * FROM person ORDER BY id DESC, name ASC"
    )
    test_parse_sql(
        "SELECT (id, name) FROM person ORDER BY id DESC, name ASC"
    )
    test_parse_sql(
        "SELECT * FROM person WHERE name = 'Joe' ORDER BY id DESC, name ASC"
    )
    test_parse_sql(
        "SELECT (id, name) FROM person WHERE name = 'Joe' ORDER BY id DESC, name ASC"
    )
    test_parse_sql(
        "SELECT * FROM person WHERE id = 1 AND name = 'Joe' ORDER BY id DESC, name ASC"
    )
    test_parse_sql(
        "SELECT (id, name) FROM person WHERE id = 1 AND name = 'Joe' ORDER BY id DESC, name ASC"
    )
    test_parse_sql(
        "SELECT id, name FROM person; SELECT id, name from heroes"
    )
    test_parse_sql(
        "INSERT INTO person VALUES ('Paul', 'none')"
    )
    test_parse_sql(
        "INSERT INTO person (name, authority) VALUES ('Paul', 'none')"
    )
    test_parse_sql(
        "INSERT INTO person VALUES ('Paul', 'none'), ('John', 'none')"
    )
    test_parse_sql(
        "INSERT INTO person (name, authority) VALUES ('Paul', 'none'), ('John', 'none')"
    )
    #Can un-comment when autoincrement config comes from IDL
    #
    #test_parse_sql(
    #    "INSERT INTO person VALUES (3, 'Paul', 'none')"
    #)
    #test_parse_sql(
    #    "INSERT INTO person (id, name, authority) VALUES (3, 'Paul', 'none')"
    #)
    #test_parse_sql(
    #    "INSERT INTO person VALUES (3, 'Paul', 'none'), (4, 'John', 'none')"
    #)
    #test_parse_sql(
    #    "INSERT INTO person (id, name, authority) VALUES (3, 'Paul', 'none'), (4, 'John', 'none')"
    #)
    #
    test_parse_sql(
        "DELETE FROM person"
    )
    test_parse_sql(
        "DELETE FROM person WHERE name = 'Joe'"
    )
    test_parse_sql(
        "DELETE FROM person WHERE id = 1 AND name = 'Joe'"
    )
    test_parse_sql(
        "UPDATE person SET name = 'Paul' WHERE id = 1"
    )
    test_parse_sql(
        "UPDATE person SET name = 'Paul' WHERE name = 'Joe'"
    )
    test_parse_sql(
        "UPDATE person SET name = 'Paul' WHERE id = 1 AND name = 'Joe'"
    )
    test_parse_sql(
        "UPDATE person SET name = 'Paul', authority = 'none' WHERE id = 1"
    )
    test_parse_sql(
        "UPDATE person SET name = 'Paul', authority = 'none' WHERE name = 'Joe'"
    )
    test_parse_sql(
        "UPDATE person SET name = 'Paul', authority = 'none' WHERE id = 1 AND name = 'Joe'"
    )