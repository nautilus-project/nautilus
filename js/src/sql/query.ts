import {
    AST,
    Delete,
    Insert_Replace,
    Select,
    Update,
} from 'node-sql-parser';
import { AllProgramTableNames, ProgramTables } from '../types';
import NodeSQLParser, { From } from 'node-sql-parser';

import { NautilusIdl } from '../idl';
import { NautilusProgram } from '..';
import { NautilusTable } from './table';

const SUPPORTED_ACTIONS = [
    "select",
    "insert",
    "delete",
    "update",
]

export class NautilusQuery<Programs extends NautilusIdl[] = NautilusIdl[]> {

    programs: NautilusProgram<Programs[number]>[];
    nautilusTables: NautilusTable[] = [];
    ast: AST | AST[];

    constructor (
        programs: NautilusProgram<Programs[number]>[],
        query: string,
    ) {
        this.programs = programs
        const parser = new NodeSQLParser.Parser()
        const ast = parser.astify(query)
        this.ast = ast
        const addTable = (astObj: AST) => {
            if (
                astObj.type &&
                SUPPORTED_ACTIONS.includes(astObj.type)
            ) {
                if (astObj.type === "delete") this.nautilusTables.push(
                    this.buildDeleteOperation(astObj)
                )
                if (astObj.type === "insert") this.nautilusTables.push(
                    this.buildInsertOperation(astObj)
                )
                if (astObj.type === "select") this.nautilusTables.push(
                    this.buildSelectOperation(astObj)
                )
                if (astObj.type === "update") this.nautilusTables.push(
                    this.buildUpdateOperation(astObj)
                )
            } else {
                unsupportedSqlError()
            }
        }
        if (Array.isArray(ast)) {
            ast.forEach((astObj) => addTable(astObj))
        } else {
            addTable(ast)
        }
    }

    dumpSql(): string {
        return new NodeSQLParser.Parser().sqlify(this.ast)
    }

    protected buildDeleteOperation(ast: Delete): NautilusTable {
        if (ast.table && Array.isArray(ast.table)) {
            const table = this.findValidTable(ast.table[0].table)
            if (ast.where) parseWhere(ast.where).forEach((w) => table.where(w[0], w[1], w[2]))
            return table.delete()
        } else {
            return sqlMissingError("source table")
        }
    }

    protected buildInsertOperation(ast: Insert_Replace): NautilusTable {
        if (ast.table && Array.isArray(ast.table)) {
            const table = this.findValidTable(ast.table[0].table)
            const columns = ast.columns
            const values: any[][] = ast.values.map((valueObj) => valueObj.value.map((v) => v.value))

            const data: any[] = []
            if (table.idl.config.autoincrement && columns && columns.includes(table.idl.config.primaryKey)) autoincrementBreachError()
            for (const val of values) {
                if (table.idl.type.fields.length == val.length) autoincrementBreachError()
                const entries: any[][] = []
                const fieldsWithoutPrimaryKey = table.idl.type.fields.filter(e => e.name !== table.idl.config.primaryKey).map(e => e.name)
                val.forEach((v, i) => entries.push([fieldsWithoutPrimaryKey[i], v]))
                data.push(Object.fromEntries(entries))
            }

            return table.create(data)
        } else {
            return sqlMissingError("source table")
        }
    }

    protected buildSelectOperation(ast: Select): NautilusTable {
        if (ast.from && Array.isArray(ast.from)) {
            console.log(this.programs.map(e => e.tables), this.programs.map(e => Object.keys(e.tables)), ast.from, ast.from[0].table)
            const table = this.findValidTable(ast.from[0].table)
            const returnFields = ast.columns == "*" ?
                undefined
                :
                ast.columns.map((c) => c.expr.column)
            if (returnFields) table.fields(returnFields)
            if (ast.where) parseWhere(ast.where).forEach((w) => table.where(w[0], w[1], w[2]))
            if (ast.orderby) ast.orderby.forEach((o) => table.orderBy(o.expr.column, o.type))
            return table
        } else {
            return sqlMissingError("source table")
        }
    }

    protected buildUpdateOperation(ast: Update): NautilusTable {
        if (ast.table && Array.isArray(ast.table)) {
            if (!(ast.table[0] as From)) unsupportedSqlError()
            const table = this.findValidTable((ast.table[0] as From).table)
            if (ast.where) parseWhere(ast.where).forEach((w) => table.where(w[0], w[1], w[2]))
            const data = Object.fromEntries(ast.set.map((s) => {
                if (s.column == table.idl.config.primaryKey) immutablePrimaryKeyError()
                return [s.column, s.value.value]
            }))
            return table.update(data)
        } else {
            return sqlMissingError("source table")
        }
    }

    protected findValidTable(tableName: string): NautilusTable {
        // TODO: parse multiple parts, e.g. program-a.car
        // Currently only getting the first program that has the table
        const typedName = tableName as keyof ProgramTables<Programs[number]>
        const table = this.programs.find(program => program.tables[typedName])?.tables[typedName]
        if (!table) unknownProgramError()

        return table as NautilusTable
    }
}

// TODO: Does not support "OR" yet 
function parseWhere(statement: any): [string, string, string][] {
    const where: [string, string, string][] = []
    if (statement.operator === "AND") {
        where.concat(parseWhere(statement.left))
        where.concat(parseWhere(statement.right))
    }
    else {
        where.push([statement.left.column, statement.operator, statement.right.value])
    }
    return where
}

const unsupportedSqlError = () => {
    throw Error("SQL operation error: Operation not supported")
}

const sqlMissingError = (missing: string) => {
    throw Error(`SQL operation error: SQL missing the following information: ${missing}`)
}

const autoincrementBreachError = () => {
    throw Error("You cannot provide a value for the primary key if autoincrement is enabled")
}

const immutablePrimaryKeyError = () => {
    throw Error("You cannot change a primary key with an UPDATE operation")
}

const unknownProgramError = () => {
    throw Error("No program matches your query")
}