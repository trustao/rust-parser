#![allow(nonstandard_style)]
// Generated from HiveSqlParser.g4 by ANTLR 4.8
use crate::tree::ParseTreeListener;
use super::hivesqlparser::*;

pub trait HiveSqlParserListener<'input> : ParseTreeListener<'input,HiveSqlParserContextType>{
/**
 * Enter a parse tree produced by {@link HiveSqlParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#explainStatement}.
 * @param ctx the parse tree
 */
fn enter_explainStatement(&mut self, _ctx: &ExplainStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#explainStatement}.
 * @param ctx the parse tree
 */
fn exit_explainStatement(&mut self, _ctx: &ExplainStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#explainOption}.
 * @param ctx the parse tree
 */
fn enter_explainOption(&mut self, _ctx: &ExplainOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#explainOption}.
 * @param ctx the parse tree
 */
fn exit_explainOption(&mut self, _ctx: &ExplainOptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#vectorizationOnly}.
 * @param ctx the parse tree
 */
fn enter_vectorizationOnly(&mut self, _ctx: &VectorizationOnlyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#vectorizationOnly}.
 * @param ctx the parse tree
 */
fn exit_vectorizationOnly(&mut self, _ctx: &VectorizationOnlyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#vectorizatonDetail}.
 * @param ctx the parse tree
 */
fn enter_vectorizatonDetail(&mut self, _ctx: &VectorizatonDetailContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#vectorizatonDetail}.
 * @param ctx the parse tree
 */
fn exit_vectorizatonDetail(&mut self, _ctx: &VectorizatonDetailContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#execStatement}.
 * @param ctx the parse tree
 */
fn enter_execStatement(&mut self, _ctx: &ExecStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#execStatement}.
 * @param ctx the parse tree
 */
fn exit_execStatement(&mut self, _ctx: &ExecStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#loadStatement}.
 * @param ctx the parse tree
 */
fn enter_loadStatement(&mut self, _ctx: &LoadStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#loadStatement}.
 * @param ctx the parse tree
 */
fn exit_loadStatement(&mut self, _ctx: &LoadStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#replicationClause}.
 * @param ctx the parse tree
 */
fn enter_replicationClause(&mut self, _ctx: &ReplicationClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#replicationClause}.
 * @param ctx the parse tree
 */
fn exit_replicationClause(&mut self, _ctx: &ReplicationClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#exportStatement}.
 * @param ctx the parse tree
 */
fn enter_exportStatement(&mut self, _ctx: &ExportStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#exportStatement}.
 * @param ctx the parse tree
 */
fn exit_exportStatement(&mut self, _ctx: &ExportStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#importStatement}.
 * @param ctx the parse tree
 */
fn enter_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#importStatement}.
 * @param ctx the parse tree
 */
fn exit_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#replDumpStatement}.
 * @param ctx the parse tree
 */
fn enter_replDumpStatement(&mut self, _ctx: &ReplDumpStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#replDumpStatement}.
 * @param ctx the parse tree
 */
fn exit_replDumpStatement(&mut self, _ctx: &ReplDumpStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#replDbPolicy}.
 * @param ctx the parse tree
 */
fn enter_replDbPolicy(&mut self, _ctx: &ReplDbPolicyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#replDbPolicy}.
 * @param ctx the parse tree
 */
fn exit_replDbPolicy(&mut self, _ctx: &ReplDbPolicyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#replLoadStatement}.
 * @param ctx the parse tree
 */
fn enter_replLoadStatement(&mut self, _ctx: &ReplLoadStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#replLoadStatement}.
 * @param ctx the parse tree
 */
fn exit_replLoadStatement(&mut self, _ctx: &ReplLoadStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#replConfigs}.
 * @param ctx the parse tree
 */
fn enter_replConfigs(&mut self, _ctx: &ReplConfigsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#replConfigs}.
 * @param ctx the parse tree
 */
fn exit_replConfigs(&mut self, _ctx: &ReplConfigsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#replConfigsList}.
 * @param ctx the parse tree
 */
fn enter_replConfigsList(&mut self, _ctx: &ReplConfigsListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#replConfigsList}.
 * @param ctx the parse tree
 */
fn exit_replConfigsList(&mut self, _ctx: &ReplConfigsListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#replTableLevelPolicy}.
 * @param ctx the parse tree
 */
fn enter_replTableLevelPolicy(&mut self, _ctx: &ReplTableLevelPolicyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#replTableLevelPolicy}.
 * @param ctx the parse tree
 */
fn exit_replTableLevelPolicy(&mut self, _ctx: &ReplTableLevelPolicyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#replStatusStatement}.
 * @param ctx the parse tree
 */
fn enter_replStatusStatement(&mut self, _ctx: &ReplStatusStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#replStatusStatement}.
 * @param ctx the parse tree
 */
fn exit_replStatusStatement(&mut self, _ctx: &ReplStatusStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#ddlStatement}.
 * @param ctx the parse tree
 */
fn enter_ddlStatement(&mut self, _ctx: &DdlStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#ddlStatement}.
 * @param ctx the parse tree
 */
fn exit_ddlStatement(&mut self, _ctx: &DdlStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#ifExists}.
 * @param ctx the parse tree
 */
fn enter_ifExists(&mut self, _ctx: &IfExistsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#ifExists}.
 * @param ctx the parse tree
 */
fn exit_ifExists(&mut self, _ctx: &IfExistsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#restrictOrCascade}.
 * @param ctx the parse tree
 */
fn enter_restrictOrCascade(&mut self, _ctx: &RestrictOrCascadeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#restrictOrCascade}.
 * @param ctx the parse tree
 */
fn exit_restrictOrCascade(&mut self, _ctx: &RestrictOrCascadeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#ifNotExists}.
 * @param ctx the parse tree
 */
fn enter_ifNotExists(&mut self, _ctx: &IfNotExistsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#ifNotExists}.
 * @param ctx the parse tree
 */
fn exit_ifNotExists(&mut self, _ctx: &IfNotExistsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#force}.
 * @param ctx the parse tree
 */
fn enter_force(&mut self, _ctx: &ForceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#force}.
 * @param ctx the parse tree
 */
fn exit_force(&mut self, _ctx: &ForceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#rewriteEnabled}.
 * @param ctx the parse tree
 */
fn enter_rewriteEnabled(&mut self, _ctx: &RewriteEnabledContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#rewriteEnabled}.
 * @param ctx the parse tree
 */
fn exit_rewriteEnabled(&mut self, _ctx: &RewriteEnabledContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#rewriteDisabled}.
 * @param ctx the parse tree
 */
fn enter_rewriteDisabled(&mut self, _ctx: &RewriteDisabledContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#rewriteDisabled}.
 * @param ctx the parse tree
 */
fn exit_rewriteDisabled(&mut self, _ctx: &RewriteDisabledContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#storedAsDirs}.
 * @param ctx the parse tree
 */
fn enter_storedAsDirs(&mut self, _ctx: &StoredAsDirsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#storedAsDirs}.
 * @param ctx the parse tree
 */
fn exit_storedAsDirs(&mut self, _ctx: &StoredAsDirsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#orReplace}.
 * @param ctx the parse tree
 */
fn enter_orReplace(&mut self, _ctx: &OrReplaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#orReplace}.
 * @param ctx the parse tree
 */
fn exit_orReplace(&mut self, _ctx: &OrReplaceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createDatabaseStatement}.
 * @param ctx the parse tree
 */
fn enter_createDatabaseStatement(&mut self, _ctx: &CreateDatabaseStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createDatabaseStatement}.
 * @param ctx the parse tree
 */
fn exit_createDatabaseStatement(&mut self, _ctx: &CreateDatabaseStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dbLocation}.
 * @param ctx the parse tree
 */
fn enter_dbLocation(&mut self, _ctx: &DbLocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dbLocation}.
 * @param ctx the parse tree
 */
fn exit_dbLocation(&mut self, _ctx: &DbLocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dbManagedLocation}.
 * @param ctx the parse tree
 */
fn enter_dbManagedLocation(&mut self, _ctx: &DbManagedLocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dbManagedLocation}.
 * @param ctx the parse tree
 */
fn exit_dbManagedLocation(&mut self, _ctx: &DbManagedLocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dbProperties}.
 * @param ctx the parse tree
 */
fn enter_dbProperties(&mut self, _ctx: &DbPropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dbProperties}.
 * @param ctx the parse tree
 */
fn exit_dbProperties(&mut self, _ctx: &DbPropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dbPropertiesList}.
 * @param ctx the parse tree
 */
fn enter_dbPropertiesList(&mut self, _ctx: &DbPropertiesListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dbPropertiesList}.
 * @param ctx the parse tree
 */
fn exit_dbPropertiesList(&mut self, _ctx: &DbPropertiesListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dbConnectorName}.
 * @param ctx the parse tree
 */
fn enter_dbConnectorName(&mut self, _ctx: &DbConnectorNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dbConnectorName}.
 * @param ctx the parse tree
 */
fn exit_dbConnectorName(&mut self, _ctx: &DbConnectorNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#switchDatabaseStatement}.
 * @param ctx the parse tree
 */
fn enter_switchDatabaseStatement(&mut self, _ctx: &SwitchDatabaseStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#switchDatabaseStatement}.
 * @param ctx the parse tree
 */
fn exit_switchDatabaseStatement(&mut self, _ctx: &SwitchDatabaseStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropDatabaseStatement}.
 * @param ctx the parse tree
 */
fn enter_dropDatabaseStatement(&mut self, _ctx: &DropDatabaseStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropDatabaseStatement}.
 * @param ctx the parse tree
 */
fn exit_dropDatabaseStatement(&mut self, _ctx: &DropDatabaseStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#databaseComment}.
 * @param ctx the parse tree
 */
fn enter_databaseComment(&mut self, _ctx: &DatabaseCommentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#databaseComment}.
 * @param ctx the parse tree
 */
fn exit_databaseComment(&mut self, _ctx: &DatabaseCommentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#truncateTableStatement}.
 * @param ctx the parse tree
 */
fn enter_truncateTableStatement(&mut self, _ctx: &TruncateTableStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#truncateTableStatement}.
 * @param ctx the parse tree
 */
fn exit_truncateTableStatement(&mut self, _ctx: &TruncateTableStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropTableStatement}.
 * @param ctx the parse tree
 */
fn enter_dropTableStatement(&mut self, _ctx: &DropTableStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropTableStatement}.
 * @param ctx the parse tree
 */
fn exit_dropTableStatement(&mut self, _ctx: &DropTableStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#inputFileFormat}.
 * @param ctx the parse tree
 */
fn enter_inputFileFormat(&mut self, _ctx: &InputFileFormatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#inputFileFormat}.
 * @param ctx the parse tree
 */
fn exit_inputFileFormat(&mut self, _ctx: &InputFileFormatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tabTypeExpr}.
 * @param ctx the parse tree
 */
fn enter_tabTypeExpr(&mut self, _ctx: &TabTypeExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tabTypeExpr}.
 * @param ctx the parse tree
 */
fn exit_tabTypeExpr(&mut self, _ctx: &TabTypeExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partTypeExpr}.
 * @param ctx the parse tree
 */
fn enter_partTypeExpr(&mut self, _ctx: &PartTypeExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partTypeExpr}.
 * @param ctx the parse tree
 */
fn exit_partTypeExpr(&mut self, _ctx: &PartTypeExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tabPartColTypeExpr}.
 * @param ctx the parse tree
 */
fn enter_tabPartColTypeExpr(&mut self, _ctx: &TabPartColTypeExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tabPartColTypeExpr}.
 * @param ctx the parse tree
 */
fn exit_tabPartColTypeExpr(&mut self, _ctx: &TabPartColTypeExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#descStatement}.
 * @param ctx the parse tree
 */
fn enter_descStatement(&mut self, _ctx: &DescStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#descStatement}.
 * @param ctx the parse tree
 */
fn exit_descStatement(&mut self, _ctx: &DescStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#analyzeStatement}.
 * @param ctx the parse tree
 */
fn enter_analyzeStatement(&mut self, _ctx: &AnalyzeStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#analyzeStatement}.
 * @param ctx the parse tree
 */
fn exit_analyzeStatement(&mut self, _ctx: &AnalyzeStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#from_in}.
 * @param ctx the parse tree
 */
fn enter_from_in(&mut self, _ctx: &From_inContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#from_in}.
 * @param ctx the parse tree
 */
fn exit_from_in(&mut self, _ctx: &From_inContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#db_schema}.
 * @param ctx the parse tree
 */
fn enter_db_schema(&mut self, _ctx: &Db_schemaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#db_schema}.
 * @param ctx the parse tree
 */
fn exit_db_schema(&mut self, _ctx: &Db_schemaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#showStatement}.
 * @param ctx the parse tree
 */
fn enter_showStatement(&mut self, _ctx: &ShowStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#showStatement}.
 * @param ctx the parse tree
 */
fn exit_showStatement(&mut self, _ctx: &ShowStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#showTablesFilterExpr}.
 * @param ctx the parse tree
 */
fn enter_showTablesFilterExpr(&mut self, _ctx: &ShowTablesFilterExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#showTablesFilterExpr}.
 * @param ctx the parse tree
 */
fn exit_showTablesFilterExpr(&mut self, _ctx: &ShowTablesFilterExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#lockStatement}.
 * @param ctx the parse tree
 */
fn enter_lockStatement(&mut self, _ctx: &LockStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#lockStatement}.
 * @param ctx the parse tree
 */
fn exit_lockStatement(&mut self, _ctx: &LockStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#lockDatabase}.
 * @param ctx the parse tree
 */
fn enter_lockDatabase(&mut self, _ctx: &LockDatabaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#lockDatabase}.
 * @param ctx the parse tree
 */
fn exit_lockDatabase(&mut self, _ctx: &LockDatabaseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#lockMode}.
 * @param ctx the parse tree
 */
fn enter_lockMode(&mut self, _ctx: &LockModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#lockMode}.
 * @param ctx the parse tree
 */
fn exit_lockMode(&mut self, _ctx: &LockModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#unlockStatement}.
 * @param ctx the parse tree
 */
fn enter_unlockStatement(&mut self, _ctx: &UnlockStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#unlockStatement}.
 * @param ctx the parse tree
 */
fn exit_unlockStatement(&mut self, _ctx: &UnlockStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#unlockDatabase}.
 * @param ctx the parse tree
 */
fn enter_unlockDatabase(&mut self, _ctx: &UnlockDatabaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#unlockDatabase}.
 * @param ctx the parse tree
 */
fn exit_unlockDatabase(&mut self, _ctx: &UnlockDatabaseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createRoleStatement}.
 * @param ctx the parse tree
 */
fn enter_createRoleStatement(&mut self, _ctx: &CreateRoleStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createRoleStatement}.
 * @param ctx the parse tree
 */
fn exit_createRoleStatement(&mut self, _ctx: &CreateRoleStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropRoleStatement}.
 * @param ctx the parse tree
 */
fn enter_dropRoleStatement(&mut self, _ctx: &DropRoleStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropRoleStatement}.
 * @param ctx the parse tree
 */
fn exit_dropRoleStatement(&mut self, _ctx: &DropRoleStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#grantPrivileges}.
 * @param ctx the parse tree
 */
fn enter_grantPrivileges(&mut self, _ctx: &GrantPrivilegesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#grantPrivileges}.
 * @param ctx the parse tree
 */
fn exit_grantPrivileges(&mut self, _ctx: &GrantPrivilegesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#revokePrivileges}.
 * @param ctx the parse tree
 */
fn enter_revokePrivileges(&mut self, _ctx: &RevokePrivilegesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#revokePrivileges}.
 * @param ctx the parse tree
 */
fn exit_revokePrivileges(&mut self, _ctx: &RevokePrivilegesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#grantRole}.
 * @param ctx the parse tree
 */
fn enter_grantRole(&mut self, _ctx: &GrantRoleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#grantRole}.
 * @param ctx the parse tree
 */
fn exit_grantRole(&mut self, _ctx: &GrantRoleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#revokeRole}.
 * @param ctx the parse tree
 */
fn enter_revokeRole(&mut self, _ctx: &RevokeRoleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#revokeRole}.
 * @param ctx the parse tree
 */
fn exit_revokeRole(&mut self, _ctx: &RevokeRoleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#showRoleGrants}.
 * @param ctx the parse tree
 */
fn enter_showRoleGrants(&mut self, _ctx: &ShowRoleGrantsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#showRoleGrants}.
 * @param ctx the parse tree
 */
fn exit_showRoleGrants(&mut self, _ctx: &ShowRoleGrantsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#showRoles}.
 * @param ctx the parse tree
 */
fn enter_showRoles(&mut self, _ctx: &ShowRolesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#showRoles}.
 * @param ctx the parse tree
 */
fn exit_showRoles(&mut self, _ctx: &ShowRolesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#showCurrentRole}.
 * @param ctx the parse tree
 */
fn enter_showCurrentRole(&mut self, _ctx: &ShowCurrentRoleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#showCurrentRole}.
 * @param ctx the parse tree
 */
fn exit_showCurrentRole(&mut self, _ctx: &ShowCurrentRoleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#setRole}.
 * @param ctx the parse tree
 */
fn enter_setRole(&mut self, _ctx: &SetRoleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#setRole}.
 * @param ctx the parse tree
 */
fn exit_setRole(&mut self, _ctx: &SetRoleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#showGrants}.
 * @param ctx the parse tree
 */
fn enter_showGrants(&mut self, _ctx: &ShowGrantsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#showGrants}.
 * @param ctx the parse tree
 */
fn exit_showGrants(&mut self, _ctx: &ShowGrantsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#showRolePrincipals}.
 * @param ctx the parse tree
 */
fn enter_showRolePrincipals(&mut self, _ctx: &ShowRolePrincipalsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#showRolePrincipals}.
 * @param ctx the parse tree
 */
fn exit_showRolePrincipals(&mut self, _ctx: &ShowRolePrincipalsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#privilegeIncludeColObject}.
 * @param ctx the parse tree
 */
fn enter_privilegeIncludeColObject(&mut self, _ctx: &PrivilegeIncludeColObjectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#privilegeIncludeColObject}.
 * @param ctx the parse tree
 */
fn exit_privilegeIncludeColObject(&mut self, _ctx: &PrivilegeIncludeColObjectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#privilegeObject}.
 * @param ctx the parse tree
 */
fn enter_privilegeObject(&mut self, _ctx: &PrivilegeObjectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#privilegeObject}.
 * @param ctx the parse tree
 */
fn exit_privilegeObject(&mut self, _ctx: &PrivilegeObjectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#privObject}.
 * @param ctx the parse tree
 */
fn enter_privObject(&mut self, _ctx: &PrivObjectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#privObject}.
 * @param ctx the parse tree
 */
fn exit_privObject(&mut self, _ctx: &PrivObjectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#privObjectCols}.
 * @param ctx the parse tree
 */
fn enter_privObjectCols(&mut self, _ctx: &PrivObjectColsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#privObjectCols}.
 * @param ctx the parse tree
 */
fn exit_privObjectCols(&mut self, _ctx: &PrivObjectColsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#privilegeList}.
 * @param ctx the parse tree
 */
fn enter_privilegeList(&mut self, _ctx: &PrivilegeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#privilegeList}.
 * @param ctx the parse tree
 */
fn exit_privilegeList(&mut self, _ctx: &PrivilegeListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#privlegeDef}.
 * @param ctx the parse tree
 */
fn enter_privlegeDef(&mut self, _ctx: &PrivlegeDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#privlegeDef}.
 * @param ctx the parse tree
 */
fn exit_privlegeDef(&mut self, _ctx: &PrivlegeDefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#privilegeType}.
 * @param ctx the parse tree
 */
fn enter_privilegeType(&mut self, _ctx: &PrivilegeTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#privilegeType}.
 * @param ctx the parse tree
 */
fn exit_privilegeType(&mut self, _ctx: &PrivilegeTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#principalSpecification}.
 * @param ctx the parse tree
 */
fn enter_principalSpecification(&mut self, _ctx: &PrincipalSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#principalSpecification}.
 * @param ctx the parse tree
 */
fn exit_principalSpecification(&mut self, _ctx: &PrincipalSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#principalName}.
 * @param ctx the parse tree
 */
fn enter_principalName(&mut self, _ctx: &PrincipalNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#principalName}.
 * @param ctx the parse tree
 */
fn exit_principalName(&mut self, _ctx: &PrincipalNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#withGrantOption}.
 * @param ctx the parse tree
 */
fn enter_withGrantOption(&mut self, _ctx: &WithGrantOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#withGrantOption}.
 * @param ctx the parse tree
 */
fn exit_withGrantOption(&mut self, _ctx: &WithGrantOptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#grantOptionFor}.
 * @param ctx the parse tree
 */
fn enter_grantOptionFor(&mut self, _ctx: &GrantOptionForContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#grantOptionFor}.
 * @param ctx the parse tree
 */
fn exit_grantOptionFor(&mut self, _ctx: &GrantOptionForContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#adminOptionFor}.
 * @param ctx the parse tree
 */
fn enter_adminOptionFor(&mut self, _ctx: &AdminOptionForContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#adminOptionFor}.
 * @param ctx the parse tree
 */
fn exit_adminOptionFor(&mut self, _ctx: &AdminOptionForContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#withAdminOption}.
 * @param ctx the parse tree
 */
fn enter_withAdminOption(&mut self, _ctx: &WithAdminOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#withAdminOption}.
 * @param ctx the parse tree
 */
fn exit_withAdminOption(&mut self, _ctx: &WithAdminOptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#metastoreCheck}.
 * @param ctx the parse tree
 */
fn enter_metastoreCheck(&mut self, _ctx: &MetastoreCheckContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#metastoreCheck}.
 * @param ctx the parse tree
 */
fn exit_metastoreCheck(&mut self, _ctx: &MetastoreCheckContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#resourceList}.
 * @param ctx the parse tree
 */
fn enter_resourceList(&mut self, _ctx: &ResourceListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#resourceList}.
 * @param ctx the parse tree
 */
fn exit_resourceList(&mut self, _ctx: &ResourceListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#resource}.
 * @param ctx the parse tree
 */
fn enter_resource(&mut self, _ctx: &ResourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#resource}.
 * @param ctx the parse tree
 */
fn exit_resource(&mut self, _ctx: &ResourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#resourceType}.
 * @param ctx the parse tree
 */
fn enter_resourceType(&mut self, _ctx: &ResourceTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#resourceType}.
 * @param ctx the parse tree
 */
fn exit_resourceType(&mut self, _ctx: &ResourceTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createFunctionStatement}.
 * @param ctx the parse tree
 */
fn enter_createFunctionStatement(&mut self, _ctx: &CreateFunctionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createFunctionStatement}.
 * @param ctx the parse tree
 */
fn exit_createFunctionStatement(&mut self, _ctx: &CreateFunctionStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropFunctionStatement}.
 * @param ctx the parse tree
 */
fn enter_dropFunctionStatement(&mut self, _ctx: &DropFunctionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropFunctionStatement}.
 * @param ctx the parse tree
 */
fn exit_dropFunctionStatement(&mut self, _ctx: &DropFunctionStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#reloadFunctionsStatement}.
 * @param ctx the parse tree
 */
fn enter_reloadFunctionsStatement(&mut self, _ctx: &ReloadFunctionsStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#reloadFunctionsStatement}.
 * @param ctx the parse tree
 */
fn exit_reloadFunctionsStatement(&mut self, _ctx: &ReloadFunctionsStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createMacroStatement}.
 * @param ctx the parse tree
 */
fn enter_createMacroStatement(&mut self, _ctx: &CreateMacroStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createMacroStatement}.
 * @param ctx the parse tree
 */
fn exit_createMacroStatement(&mut self, _ctx: &CreateMacroStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropMacroStatement}.
 * @param ctx the parse tree
 */
fn enter_dropMacroStatement(&mut self, _ctx: &DropMacroStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropMacroStatement}.
 * @param ctx the parse tree
 */
fn exit_dropMacroStatement(&mut self, _ctx: &DropMacroStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createIndexStatement}.
 * @param ctx the parse tree
 */
fn enter_createIndexStatement(&mut self, _ctx: &CreateIndexStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createIndexStatement}.
 * @param ctx the parse tree
 */
fn exit_createIndexStatement(&mut self, _ctx: &CreateIndexStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropIndexStatement}.
 * @param ctx the parse tree
 */
fn enter_dropIndexStatement(&mut self, _ctx: &DropIndexStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropIndexStatement}.
 * @param ctx the parse tree
 */
fn exit_dropIndexStatement(&mut self, _ctx: &DropIndexStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createViewStatement}.
 * @param ctx the parse tree
 */
fn enter_createViewStatement(&mut self, _ctx: &CreateViewStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createViewStatement}.
 * @param ctx the parse tree
 */
fn exit_createViewStatement(&mut self, _ctx: &CreateViewStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#viewPartition}.
 * @param ctx the parse tree
 */
fn enter_viewPartition(&mut self, _ctx: &ViewPartitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#viewPartition}.
 * @param ctx the parse tree
 */
fn exit_viewPartition(&mut self, _ctx: &ViewPartitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#viewOrganization}.
 * @param ctx the parse tree
 */
fn enter_viewOrganization(&mut self, _ctx: &ViewOrganizationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#viewOrganization}.
 * @param ctx the parse tree
 */
fn exit_viewOrganization(&mut self, _ctx: &ViewOrganizationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#viewClusterSpec}.
 * @param ctx the parse tree
 */
fn enter_viewClusterSpec(&mut self, _ctx: &ViewClusterSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#viewClusterSpec}.
 * @param ctx the parse tree
 */
fn exit_viewClusterSpec(&mut self, _ctx: &ViewClusterSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#viewComplexSpec}.
 * @param ctx the parse tree
 */
fn enter_viewComplexSpec(&mut self, _ctx: &ViewComplexSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#viewComplexSpec}.
 * @param ctx the parse tree
 */
fn exit_viewComplexSpec(&mut self, _ctx: &ViewComplexSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#viewDistSpec}.
 * @param ctx the parse tree
 */
fn enter_viewDistSpec(&mut self, _ctx: &ViewDistSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#viewDistSpec}.
 * @param ctx the parse tree
 */
fn exit_viewDistSpec(&mut self, _ctx: &ViewDistSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#viewSortSpec}.
 * @param ctx the parse tree
 */
fn enter_viewSortSpec(&mut self, _ctx: &ViewSortSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#viewSortSpec}.
 * @param ctx the parse tree
 */
fn exit_viewSortSpec(&mut self, _ctx: &ViewSortSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropViewStatement}.
 * @param ctx the parse tree
 */
fn enter_dropViewStatement(&mut self, _ctx: &DropViewStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropViewStatement}.
 * @param ctx the parse tree
 */
fn exit_dropViewStatement(&mut self, _ctx: &DropViewStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createMaterializedViewStatement}.
 * @param ctx the parse tree
 */
fn enter_createMaterializedViewStatement(&mut self, _ctx: &CreateMaterializedViewStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createMaterializedViewStatement}.
 * @param ctx the parse tree
 */
fn exit_createMaterializedViewStatement(&mut self, _ctx: &CreateMaterializedViewStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropMaterializedViewStatement}.
 * @param ctx the parse tree
 */
fn enter_dropMaterializedViewStatement(&mut self, _ctx: &DropMaterializedViewStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropMaterializedViewStatement}.
 * @param ctx the parse tree
 */
fn exit_dropMaterializedViewStatement(&mut self, _ctx: &DropMaterializedViewStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createScheduledQueryStatement}.
 * @param ctx the parse tree
 */
fn enter_createScheduledQueryStatement(&mut self, _ctx: &CreateScheduledQueryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createScheduledQueryStatement}.
 * @param ctx the parse tree
 */
fn exit_createScheduledQueryStatement(&mut self, _ctx: &CreateScheduledQueryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropScheduledQueryStatement}.
 * @param ctx the parse tree
 */
fn enter_dropScheduledQueryStatement(&mut self, _ctx: &DropScheduledQueryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropScheduledQueryStatement}.
 * @param ctx the parse tree
 */
fn exit_dropScheduledQueryStatement(&mut self, _ctx: &DropScheduledQueryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterScheduledQueryStatement}.
 * @param ctx the parse tree
 */
fn enter_alterScheduledQueryStatement(&mut self, _ctx: &AlterScheduledQueryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterScheduledQueryStatement}.
 * @param ctx the parse tree
 */
fn exit_alterScheduledQueryStatement(&mut self, _ctx: &AlterScheduledQueryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterScheduledQueryChange}.
 * @param ctx the parse tree
 */
fn enter_alterScheduledQueryChange(&mut self, _ctx: &AlterScheduledQueryChangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterScheduledQueryChange}.
 * @param ctx the parse tree
 */
fn exit_alterScheduledQueryChange(&mut self, _ctx: &AlterScheduledQueryChangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#scheduleSpec}.
 * @param ctx the parse tree
 */
fn enter_scheduleSpec(&mut self, _ctx: &ScheduleSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#scheduleSpec}.
 * @param ctx the parse tree
 */
fn exit_scheduleSpec(&mut self, _ctx: &ScheduleSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#executedAsSpec}.
 * @param ctx the parse tree
 */
fn enter_executedAsSpec(&mut self, _ctx: &ExecutedAsSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#executedAsSpec}.
 * @param ctx the parse tree
 */
fn exit_executedAsSpec(&mut self, _ctx: &ExecutedAsSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#definedAsSpec}.
 * @param ctx the parse tree
 */
fn enter_definedAsSpec(&mut self, _ctx: &DefinedAsSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#definedAsSpec}.
 * @param ctx the parse tree
 */
fn exit_definedAsSpec(&mut self, _ctx: &DefinedAsSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#showFunctionIdentifier}.
 * @param ctx the parse tree
 */
fn enter_showFunctionIdentifier(&mut self, _ctx: &ShowFunctionIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#showFunctionIdentifier}.
 * @param ctx the parse tree
 */
fn exit_showFunctionIdentifier(&mut self, _ctx: &ShowFunctionIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#showStmtIdentifier}.
 * @param ctx the parse tree
 */
fn enter_showStmtIdentifier(&mut self, _ctx: &ShowStmtIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#showStmtIdentifier}.
 * @param ctx the parse tree
 */
fn exit_showStmtIdentifier(&mut self, _ctx: &ShowStmtIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableComment}.
 * @param ctx the parse tree
 */
fn enter_tableComment(&mut self, _ctx: &TableCommentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableComment}.
 * @param ctx the parse tree
 */
fn exit_tableComment(&mut self, _ctx: &TableCommentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createTablePartitionSpec}.
 * @param ctx the parse tree
 */
fn enter_createTablePartitionSpec(&mut self, _ctx: &CreateTablePartitionSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createTablePartitionSpec}.
 * @param ctx the parse tree
 */
fn exit_createTablePartitionSpec(&mut self, _ctx: &CreateTablePartitionSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createTablePartitionColumnTypeSpec}.
 * @param ctx the parse tree
 */
fn enter_createTablePartitionColumnTypeSpec(&mut self, _ctx: &CreateTablePartitionColumnTypeSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createTablePartitionColumnTypeSpec}.
 * @param ctx the parse tree
 */
fn exit_createTablePartitionColumnTypeSpec(&mut self, _ctx: &CreateTablePartitionColumnTypeSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createTablePartitionColumnSpec}.
 * @param ctx the parse tree
 */
fn enter_createTablePartitionColumnSpec(&mut self, _ctx: &CreateTablePartitionColumnSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createTablePartitionColumnSpec}.
 * @param ctx the parse tree
 */
fn exit_createTablePartitionColumnSpec(&mut self, _ctx: &CreateTablePartitionColumnSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partitionTransformSpec}.
 * @param ctx the parse tree
 */
fn enter_partitionTransformSpec(&mut self, _ctx: &PartitionTransformSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partitionTransformSpec}.
 * @param ctx the parse tree
 */
fn exit_partitionTransformSpec(&mut self, _ctx: &PartitionTransformSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameTransformConstraint}.
 * @param ctx the parse tree
 */
fn enter_columnNameTransformConstraint(&mut self, _ctx: &ColumnNameTransformConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameTransformConstraint}.
 * @param ctx the parse tree
 */
fn exit_columnNameTransformConstraint(&mut self, _ctx: &ColumnNameTransformConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partitionTransformType}.
 * @param ctx the parse tree
 */
fn enter_partitionTransformType(&mut self, _ctx: &PartitionTransformTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partitionTransformType}.
 * @param ctx the parse tree
 */
fn exit_partitionTransformType(&mut self, _ctx: &PartitionTransformTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableBuckets}.
 * @param ctx the parse tree
 */
fn enter_tableBuckets(&mut self, _ctx: &TableBucketsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableBuckets}.
 * @param ctx the parse tree
 */
fn exit_tableBuckets(&mut self, _ctx: &TableBucketsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableImplBuckets}.
 * @param ctx the parse tree
 */
fn enter_tableImplBuckets(&mut self, _ctx: &TableImplBucketsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableImplBuckets}.
 * @param ctx the parse tree
 */
fn exit_tableImplBuckets(&mut self, _ctx: &TableImplBucketsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableSkewed}.
 * @param ctx the parse tree
 */
fn enter_tableSkewed(&mut self, _ctx: &TableSkewedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableSkewed}.
 * @param ctx the parse tree
 */
fn exit_tableSkewed(&mut self, _ctx: &TableSkewedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#rowFormat}.
 * @param ctx the parse tree
 */
fn enter_rowFormat(&mut self, _ctx: &RowFormatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#rowFormat}.
 * @param ctx the parse tree
 */
fn exit_rowFormat(&mut self, _ctx: &RowFormatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#recordReader}.
 * @param ctx the parse tree
 */
fn enter_recordReader(&mut self, _ctx: &RecordReaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#recordReader}.
 * @param ctx the parse tree
 */
fn exit_recordReader(&mut self, _ctx: &RecordReaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#recordWriter}.
 * @param ctx the parse tree
 */
fn enter_recordWriter(&mut self, _ctx: &RecordWriterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#recordWriter}.
 * @param ctx the parse tree
 */
fn exit_recordWriter(&mut self, _ctx: &RecordWriterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#rowFormatSerde}.
 * @param ctx the parse tree
 */
fn enter_rowFormatSerde(&mut self, _ctx: &RowFormatSerdeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#rowFormatSerde}.
 * @param ctx the parse tree
 */
fn exit_rowFormatSerde(&mut self, _ctx: &RowFormatSerdeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#rowFormatDelimited}.
 * @param ctx the parse tree
 */
fn enter_rowFormatDelimited(&mut self, _ctx: &RowFormatDelimitedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#rowFormatDelimited}.
 * @param ctx the parse tree
 */
fn exit_rowFormatDelimited(&mut self, _ctx: &RowFormatDelimitedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableRowFormat}.
 * @param ctx the parse tree
 */
fn enter_tableRowFormat(&mut self, _ctx: &TableRowFormatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableRowFormat}.
 * @param ctx the parse tree
 */
fn exit_tableRowFormat(&mut self, _ctx: &TableRowFormatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tablePropertiesPrefixed}.
 * @param ctx the parse tree
 */
fn enter_tablePropertiesPrefixed(&mut self, _ctx: &TablePropertiesPrefixedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tablePropertiesPrefixed}.
 * @param ctx the parse tree
 */
fn exit_tablePropertiesPrefixed(&mut self, _ctx: &TablePropertiesPrefixedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableProperties}.
 * @param ctx the parse tree
 */
fn enter_tableProperties(&mut self, _ctx: &TablePropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableProperties}.
 * @param ctx the parse tree
 */
fn exit_tableProperties(&mut self, _ctx: &TablePropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tablePropertiesList}.
 * @param ctx the parse tree
 */
fn enter_tablePropertiesList(&mut self, _ctx: &TablePropertiesListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tablePropertiesList}.
 * @param ctx the parse tree
 */
fn exit_tablePropertiesList(&mut self, _ctx: &TablePropertiesListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#keyValueProperty}.
 * @param ctx the parse tree
 */
fn enter_keyValueProperty(&mut self, _ctx: &KeyValuePropertyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#keyValueProperty}.
 * @param ctx the parse tree
 */
fn exit_keyValueProperty(&mut self, _ctx: &KeyValuePropertyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#keyProperty}.
 * @param ctx the parse tree
 */
fn enter_keyProperty(&mut self, _ctx: &KeyPropertyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#keyProperty}.
 * @param ctx the parse tree
 */
fn exit_keyProperty(&mut self, _ctx: &KeyPropertyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableRowFormatFieldIdentifier}.
 * @param ctx the parse tree
 */
fn enter_tableRowFormatFieldIdentifier(&mut self, _ctx: &TableRowFormatFieldIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableRowFormatFieldIdentifier}.
 * @param ctx the parse tree
 */
fn exit_tableRowFormatFieldIdentifier(&mut self, _ctx: &TableRowFormatFieldIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableRowFormatCollItemsIdentifier}.
 * @param ctx the parse tree
 */
fn enter_tableRowFormatCollItemsIdentifier(&mut self, _ctx: &TableRowFormatCollItemsIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableRowFormatCollItemsIdentifier}.
 * @param ctx the parse tree
 */
fn exit_tableRowFormatCollItemsIdentifier(&mut self, _ctx: &TableRowFormatCollItemsIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableRowFormatMapKeysIdentifier}.
 * @param ctx the parse tree
 */
fn enter_tableRowFormatMapKeysIdentifier(&mut self, _ctx: &TableRowFormatMapKeysIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableRowFormatMapKeysIdentifier}.
 * @param ctx the parse tree
 */
fn exit_tableRowFormatMapKeysIdentifier(&mut self, _ctx: &TableRowFormatMapKeysIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableRowFormatLinesIdentifier}.
 * @param ctx the parse tree
 */
fn enter_tableRowFormatLinesIdentifier(&mut self, _ctx: &TableRowFormatLinesIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableRowFormatLinesIdentifier}.
 * @param ctx the parse tree
 */
fn exit_tableRowFormatLinesIdentifier(&mut self, _ctx: &TableRowFormatLinesIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableRowNullFormat}.
 * @param ctx the parse tree
 */
fn enter_tableRowNullFormat(&mut self, _ctx: &TableRowNullFormatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableRowNullFormat}.
 * @param ctx the parse tree
 */
fn exit_tableRowNullFormat(&mut self, _ctx: &TableRowNullFormatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableFileFormat}.
 * @param ctx the parse tree
 */
fn enter_tableFileFormat(&mut self, _ctx: &TableFileFormatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableFileFormat}.
 * @param ctx the parse tree
 */
fn exit_tableFileFormat(&mut self, _ctx: &TableFileFormatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableLocation}.
 * @param ctx the parse tree
 */
fn enter_tableLocation(&mut self, _ctx: &TableLocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableLocation}.
 * @param ctx the parse tree
 */
fn exit_tableLocation(&mut self, _ctx: &TableLocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameTypeList}.
 * @param ctx the parse tree
 */
fn enter_columnNameTypeList(&mut self, _ctx: &ColumnNameTypeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameTypeList}.
 * @param ctx the parse tree
 */
fn exit_columnNameTypeList(&mut self, _ctx: &ColumnNameTypeListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameTypeOrConstraintList}.
 * @param ctx the parse tree
 */
fn enter_columnNameTypeOrConstraintList(&mut self, _ctx: &ColumnNameTypeOrConstraintListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameTypeOrConstraintList}.
 * @param ctx the parse tree
 */
fn exit_columnNameTypeOrConstraintList(&mut self, _ctx: &ColumnNameTypeOrConstraintListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameColonTypeList}.
 * @param ctx the parse tree
 */
fn enter_columnNameColonTypeList(&mut self, _ctx: &ColumnNameColonTypeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameColonTypeList}.
 * @param ctx the parse tree
 */
fn exit_columnNameColonTypeList(&mut self, _ctx: &ColumnNameColonTypeListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameList}.
 * @param ctx the parse tree
 */
fn enter_columnNameList(&mut self, _ctx: &ColumnNameListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameList}.
 * @param ctx the parse tree
 */
fn exit_columnNameList(&mut self, _ctx: &ColumnNameListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnName}.
 * @param ctx the parse tree
 */
fn enter_columnName(&mut self, _ctx: &ColumnNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnName}.
 * @param ctx the parse tree
 */
fn exit_columnName(&mut self, _ctx: &ColumnNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#extColumnName}.
 * @param ctx the parse tree
 */
fn enter_extColumnName(&mut self, _ctx: &ExtColumnNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#extColumnName}.
 * @param ctx the parse tree
 */
fn exit_extColumnName(&mut self, _ctx: &ExtColumnNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameOrderList}.
 * @param ctx the parse tree
 */
fn enter_columnNameOrderList(&mut self, _ctx: &ColumnNameOrderListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameOrderList}.
 * @param ctx the parse tree
 */
fn exit_columnNameOrderList(&mut self, _ctx: &ColumnNameOrderListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnParenthesesList}.
 * @param ctx the parse tree
 */
fn enter_columnParenthesesList(&mut self, _ctx: &ColumnParenthesesListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnParenthesesList}.
 * @param ctx the parse tree
 */
fn exit_columnParenthesesList(&mut self, _ctx: &ColumnParenthesesListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#enableValidateSpecification}.
 * @param ctx the parse tree
 */
fn enter_enableValidateSpecification(&mut self, _ctx: &EnableValidateSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#enableValidateSpecification}.
 * @param ctx the parse tree
 */
fn exit_enableValidateSpecification(&mut self, _ctx: &EnableValidateSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#enableSpecification}.
 * @param ctx the parse tree
 */
fn enter_enableSpecification(&mut self, _ctx: &EnableSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#enableSpecification}.
 * @param ctx the parse tree
 */
fn exit_enableSpecification(&mut self, _ctx: &EnableSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#validateSpecification}.
 * @param ctx the parse tree
 */
fn enter_validateSpecification(&mut self, _ctx: &ValidateSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#validateSpecification}.
 * @param ctx the parse tree
 */
fn exit_validateSpecification(&mut self, _ctx: &ValidateSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#enforcedSpecification}.
 * @param ctx the parse tree
 */
fn enter_enforcedSpecification(&mut self, _ctx: &EnforcedSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#enforcedSpecification}.
 * @param ctx the parse tree
 */
fn exit_enforcedSpecification(&mut self, _ctx: &EnforcedSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#relySpecification}.
 * @param ctx the parse tree
 */
fn enter_relySpecification(&mut self, _ctx: &RelySpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#relySpecification}.
 * @param ctx the parse tree
 */
fn exit_relySpecification(&mut self, _ctx: &RelySpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createConstraint}.
 * @param ctx the parse tree
 */
fn enter_createConstraint(&mut self, _ctx: &CreateConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createConstraint}.
 * @param ctx the parse tree
 */
fn exit_createConstraint(&mut self, _ctx: &CreateConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterConstraintWithName}.
 * @param ctx the parse tree
 */
fn enter_alterConstraintWithName(&mut self, _ctx: &AlterConstraintWithNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterConstraintWithName}.
 * @param ctx the parse tree
 */
fn exit_alterConstraintWithName(&mut self, _ctx: &AlterConstraintWithNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableLevelConstraint}.
 * @param ctx the parse tree
 */
fn enter_tableLevelConstraint(&mut self, _ctx: &TableLevelConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableLevelConstraint}.
 * @param ctx the parse tree
 */
fn exit_tableLevelConstraint(&mut self, _ctx: &TableLevelConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#pkUkConstraint}.
 * @param ctx the parse tree
 */
fn enter_pkUkConstraint(&mut self, _ctx: &PkUkConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#pkUkConstraint}.
 * @param ctx the parse tree
 */
fn exit_pkUkConstraint(&mut self, _ctx: &PkUkConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#checkConstraint}.
 * @param ctx the parse tree
 */
fn enter_checkConstraint(&mut self, _ctx: &CheckConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#checkConstraint}.
 * @param ctx the parse tree
 */
fn exit_checkConstraint(&mut self, _ctx: &CheckConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createForeignKey}.
 * @param ctx the parse tree
 */
fn enter_createForeignKey(&mut self, _ctx: &CreateForeignKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createForeignKey}.
 * @param ctx the parse tree
 */
fn exit_createForeignKey(&mut self, _ctx: &CreateForeignKeyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterForeignKeyWithName}.
 * @param ctx the parse tree
 */
fn enter_alterForeignKeyWithName(&mut self, _ctx: &AlterForeignKeyWithNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterForeignKeyWithName}.
 * @param ctx the parse tree
 */
fn exit_alterForeignKeyWithName(&mut self, _ctx: &AlterForeignKeyWithNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#skewedValueElement}.
 * @param ctx the parse tree
 */
fn enter_skewedValueElement(&mut self, _ctx: &SkewedValueElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#skewedValueElement}.
 * @param ctx the parse tree
 */
fn exit_skewedValueElement(&mut self, _ctx: &SkewedValueElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#skewedColumnValuePairList}.
 * @param ctx the parse tree
 */
fn enter_skewedColumnValuePairList(&mut self, _ctx: &SkewedColumnValuePairListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#skewedColumnValuePairList}.
 * @param ctx the parse tree
 */
fn exit_skewedColumnValuePairList(&mut self, _ctx: &SkewedColumnValuePairListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#skewedColumnValuePair}.
 * @param ctx the parse tree
 */
fn enter_skewedColumnValuePair(&mut self, _ctx: &SkewedColumnValuePairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#skewedColumnValuePair}.
 * @param ctx the parse tree
 */
fn exit_skewedColumnValuePair(&mut self, _ctx: &SkewedColumnValuePairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#skewedColumnValues}.
 * @param ctx the parse tree
 */
fn enter_skewedColumnValues(&mut self, _ctx: &SkewedColumnValuesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#skewedColumnValues}.
 * @param ctx the parse tree
 */
fn exit_skewedColumnValues(&mut self, _ctx: &SkewedColumnValuesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#skewedColumnValue}.
 * @param ctx the parse tree
 */
fn enter_skewedColumnValue(&mut self, _ctx: &SkewedColumnValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#skewedColumnValue}.
 * @param ctx the parse tree
 */
fn exit_skewedColumnValue(&mut self, _ctx: &SkewedColumnValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#skewedValueLocationElement}.
 * @param ctx the parse tree
 */
fn enter_skewedValueLocationElement(&mut self, _ctx: &SkewedValueLocationElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#skewedValueLocationElement}.
 * @param ctx the parse tree
 */
fn exit_skewedValueLocationElement(&mut self, _ctx: &SkewedValueLocationElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#orderSpecification}.
 * @param ctx the parse tree
 */
fn enter_orderSpecification(&mut self, _ctx: &OrderSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#orderSpecification}.
 * @param ctx the parse tree
 */
fn exit_orderSpecification(&mut self, _ctx: &OrderSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#nullOrdering}.
 * @param ctx the parse tree
 */
fn enter_nullOrdering(&mut self, _ctx: &NullOrderingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#nullOrdering}.
 * @param ctx the parse tree
 */
fn exit_nullOrdering(&mut self, _ctx: &NullOrderingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameOrder}.
 * @param ctx the parse tree
 */
fn enter_columnNameOrder(&mut self, _ctx: &ColumnNameOrderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameOrder}.
 * @param ctx the parse tree
 */
fn exit_columnNameOrder(&mut self, _ctx: &ColumnNameOrderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameCommentList}.
 * @param ctx the parse tree
 */
fn enter_columnNameCommentList(&mut self, _ctx: &ColumnNameCommentListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameCommentList}.
 * @param ctx the parse tree
 */
fn exit_columnNameCommentList(&mut self, _ctx: &ColumnNameCommentListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameComment}.
 * @param ctx the parse tree
 */
fn enter_columnNameComment(&mut self, _ctx: &ColumnNameCommentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameComment}.
 * @param ctx the parse tree
 */
fn exit_columnNameComment(&mut self, _ctx: &ColumnNameCommentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#orderSpecificationRewrite}.
 * @param ctx the parse tree
 */
fn enter_orderSpecificationRewrite(&mut self, _ctx: &OrderSpecificationRewriteContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#orderSpecificationRewrite}.
 * @param ctx the parse tree
 */
fn exit_orderSpecificationRewrite(&mut self, _ctx: &OrderSpecificationRewriteContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnRefOrder}.
 * @param ctx the parse tree
 */
fn enter_columnRefOrder(&mut self, _ctx: &ColumnRefOrderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnRefOrder}.
 * @param ctx the parse tree
 */
fn exit_columnRefOrder(&mut self, _ctx: &ColumnRefOrderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameType}.
 * @param ctx the parse tree
 */
fn enter_columnNameType(&mut self, _ctx: &ColumnNameTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameType}.
 * @param ctx the parse tree
 */
fn exit_columnNameType(&mut self, _ctx: &ColumnNameTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameTypeOrConstraint}.
 * @param ctx the parse tree
 */
fn enter_columnNameTypeOrConstraint(&mut self, _ctx: &ColumnNameTypeOrConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameTypeOrConstraint}.
 * @param ctx the parse tree
 */
fn exit_columnNameTypeOrConstraint(&mut self, _ctx: &ColumnNameTypeOrConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableConstraint}.
 * @param ctx the parse tree
 */
fn enter_tableConstraint(&mut self, _ctx: &TableConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableConstraint}.
 * @param ctx the parse tree
 */
fn exit_tableConstraint(&mut self, _ctx: &TableConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameTypeConstraint}.
 * @param ctx the parse tree
 */
fn enter_columnNameTypeConstraint(&mut self, _ctx: &ColumnNameTypeConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameTypeConstraint}.
 * @param ctx the parse tree
 */
fn exit_columnNameTypeConstraint(&mut self, _ctx: &ColumnNameTypeConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnConstraint}.
 * @param ctx the parse tree
 */
fn enter_columnConstraint(&mut self, _ctx: &ColumnConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnConstraint}.
 * @param ctx the parse tree
 */
fn exit_columnConstraint(&mut self, _ctx: &ColumnConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#foreignKeyConstraint}.
 * @param ctx the parse tree
 */
fn enter_foreignKeyConstraint(&mut self, _ctx: &ForeignKeyConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#foreignKeyConstraint}.
 * @param ctx the parse tree
 */
fn exit_foreignKeyConstraint(&mut self, _ctx: &ForeignKeyConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#colConstraint}.
 * @param ctx the parse tree
 */
fn enter_colConstraint(&mut self, _ctx: &ColConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#colConstraint}.
 * @param ctx the parse tree
 */
fn exit_colConstraint(&mut self, _ctx: &ColConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterColumnConstraint}.
 * @param ctx the parse tree
 */
fn enter_alterColumnConstraint(&mut self, _ctx: &AlterColumnConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterColumnConstraint}.
 * @param ctx the parse tree
 */
fn exit_alterColumnConstraint(&mut self, _ctx: &AlterColumnConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterForeignKeyConstraint}.
 * @param ctx the parse tree
 */
fn enter_alterForeignKeyConstraint(&mut self, _ctx: &AlterForeignKeyConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterForeignKeyConstraint}.
 * @param ctx the parse tree
 */
fn exit_alterForeignKeyConstraint(&mut self, _ctx: &AlterForeignKeyConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterColConstraint}.
 * @param ctx the parse tree
 */
fn enter_alterColConstraint(&mut self, _ctx: &AlterColConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterColConstraint}.
 * @param ctx the parse tree
 */
fn exit_alterColConstraint(&mut self, _ctx: &AlterColConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnConstraintType}.
 * @param ctx the parse tree
 */
fn enter_columnConstraintType(&mut self, _ctx: &ColumnConstraintTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnConstraintType}.
 * @param ctx the parse tree
 */
fn exit_columnConstraintType(&mut self, _ctx: &ColumnConstraintTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#defaultVal}.
 * @param ctx the parse tree
 */
fn enter_defaultVal(&mut self, _ctx: &DefaultValContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#defaultVal}.
 * @param ctx the parse tree
 */
fn exit_defaultVal(&mut self, _ctx: &DefaultValContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableConstraintType}.
 * @param ctx the parse tree
 */
fn enter_tableConstraintType(&mut self, _ctx: &TableConstraintTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableConstraintType}.
 * @param ctx the parse tree
 */
fn exit_tableConstraintType(&mut self, _ctx: &TableConstraintTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#constraintOptsCreate}.
 * @param ctx the parse tree
 */
fn enter_constraintOptsCreate(&mut self, _ctx: &ConstraintOptsCreateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#constraintOptsCreate}.
 * @param ctx the parse tree
 */
fn exit_constraintOptsCreate(&mut self, _ctx: &ConstraintOptsCreateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#constraintOptsAlter}.
 * @param ctx the parse tree
 */
fn enter_constraintOptsAlter(&mut self, _ctx: &ConstraintOptsAlterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#constraintOptsAlter}.
 * @param ctx the parse tree
 */
fn exit_constraintOptsAlter(&mut self, _ctx: &ConstraintOptsAlterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnNameColonType}.
 * @param ctx the parse tree
 */
fn enter_columnNameColonType(&mut self, _ctx: &ColumnNameColonTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnNameColonType}.
 * @param ctx the parse tree
 */
fn exit_columnNameColonType(&mut self, _ctx: &ColumnNameColonTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#colType}.
 * @param ctx the parse tree
 */
fn enter_colType(&mut self, _ctx: &ColTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#colType}.
 * @param ctx the parse tree
 */
fn exit_colType(&mut self, _ctx: &ColTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#colTypeList}.
 * @param ctx the parse tree
 */
fn enter_colTypeList(&mut self, _ctx: &ColTypeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#colTypeList}.
 * @param ctx the parse tree
 */
fn exit_colTypeList(&mut self, _ctx: &ColTypeListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#type}.
 * @param ctx the parse tree
 */
fn enter_type(&mut self, _ctx: &TypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#type}.
 * @param ctx the parse tree
 */
fn exit_type(&mut self, _ctx: &TypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#primitiveType}.
 * @param ctx the parse tree
 */
fn enter_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#primitiveType}.
 * @param ctx the parse tree
 */
fn exit_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#listType}.
 * @param ctx the parse tree
 */
fn enter_listType(&mut self, _ctx: &ListTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#listType}.
 * @param ctx the parse tree
 */
fn exit_listType(&mut self, _ctx: &ListTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#structType}.
 * @param ctx the parse tree
 */
fn enter_structType(&mut self, _ctx: &StructTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#structType}.
 * @param ctx the parse tree
 */
fn exit_structType(&mut self, _ctx: &StructTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#mapType}.
 * @param ctx the parse tree
 */
fn enter_mapType(&mut self, _ctx: &MapTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#mapType}.
 * @param ctx the parse tree
 */
fn exit_mapType(&mut self, _ctx: &MapTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#unionType}.
 * @param ctx the parse tree
 */
fn enter_unionType(&mut self, _ctx: &UnionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#unionType}.
 * @param ctx the parse tree
 */
fn exit_unionType(&mut self, _ctx: &UnionTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#setOperator}.
 * @param ctx the parse tree
 */
fn enter_setOperator(&mut self, _ctx: &SetOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#setOperator}.
 * @param ctx the parse tree
 */
fn exit_setOperator(&mut self, _ctx: &SetOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#queryStatementExpression}.
 * @param ctx the parse tree
 */
fn enter_queryStatementExpression(&mut self, _ctx: &QueryStatementExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#queryStatementExpression}.
 * @param ctx the parse tree
 */
fn exit_queryStatementExpression(&mut self, _ctx: &QueryStatementExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#queryStatementExpressionBody}.
 * @param ctx the parse tree
 */
fn enter_queryStatementExpressionBody(&mut self, _ctx: &QueryStatementExpressionBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#queryStatementExpressionBody}.
 * @param ctx the parse tree
 */
fn exit_queryStatementExpressionBody(&mut self, _ctx: &QueryStatementExpressionBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#withClause}.
 * @param ctx the parse tree
 */
fn enter_withClause(&mut self, _ctx: &WithClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#withClause}.
 * @param ctx the parse tree
 */
fn exit_withClause(&mut self, _ctx: &WithClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#cteStatement}.
 * @param ctx the parse tree
 */
fn enter_cteStatement(&mut self, _ctx: &CteStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#cteStatement}.
 * @param ctx the parse tree
 */
fn exit_cteStatement(&mut self, _ctx: &CteStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#fromStatement}.
 * @param ctx the parse tree
 */
fn enter_fromStatement(&mut self, _ctx: &FromStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#fromStatement}.
 * @param ctx the parse tree
 */
fn exit_fromStatement(&mut self, _ctx: &FromStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#singleFromStatement}.
 * @param ctx the parse tree
 */
fn enter_singleFromStatement(&mut self, _ctx: &SingleFromStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#singleFromStatement}.
 * @param ctx the parse tree
 */
fn exit_singleFromStatement(&mut self, _ctx: &SingleFromStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#regularBody}.
 * @param ctx the parse tree
 */
fn enter_regularBody(&mut self, _ctx: &RegularBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#regularBody}.
 * @param ctx the parse tree
 */
fn exit_regularBody(&mut self, _ctx: &RegularBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#atomSelectStatement}.
 * @param ctx the parse tree
 */
fn enter_atomSelectStatement(&mut self, _ctx: &AtomSelectStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#atomSelectStatement}.
 * @param ctx the parse tree
 */
fn exit_atomSelectStatement(&mut self, _ctx: &AtomSelectStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#selectStatement}.
 * @param ctx the parse tree
 */
fn enter_selectStatement(&mut self, _ctx: &SelectStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#selectStatement}.
 * @param ctx the parse tree
 */
fn exit_selectStatement(&mut self, _ctx: &SelectStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#setOpSelectStatement}.
 * @param ctx the parse tree
 */
fn enter_setOpSelectStatement(&mut self, _ctx: &SetOpSelectStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#setOpSelectStatement}.
 * @param ctx the parse tree
 */
fn exit_setOpSelectStatement(&mut self, _ctx: &SetOpSelectStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#selectStatementWithCTE}.
 * @param ctx the parse tree
 */
fn enter_selectStatementWithCTE(&mut self, _ctx: &SelectStatementWithCTEContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#selectStatementWithCTE}.
 * @param ctx the parse tree
 */
fn exit_selectStatementWithCTE(&mut self, _ctx: &SelectStatementWithCTEContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#body}.
 * @param ctx the parse tree
 */
fn enter_body(&mut self, _ctx: &BodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#body}.
 * @param ctx the parse tree
 */
fn exit_body(&mut self, _ctx: &BodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#insertClause}.
 * @param ctx the parse tree
 */
fn enter_insertClause(&mut self, _ctx: &InsertClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#insertClause}.
 * @param ctx the parse tree
 */
fn exit_insertClause(&mut self, _ctx: &InsertClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#destination}.
 * @param ctx the parse tree
 */
fn enter_destination(&mut self, _ctx: &DestinationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#destination}.
 * @param ctx the parse tree
 */
fn exit_destination(&mut self, _ctx: &DestinationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#limitClause}.
 * @param ctx the parse tree
 */
fn enter_limitClause(&mut self, _ctx: &LimitClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#limitClause}.
 * @param ctx the parse tree
 */
fn exit_limitClause(&mut self, _ctx: &LimitClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#deleteStatement}.
 * @param ctx the parse tree
 */
fn enter_deleteStatement(&mut self, _ctx: &DeleteStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#deleteStatement}.
 * @param ctx the parse tree
 */
fn exit_deleteStatement(&mut self, _ctx: &DeleteStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnAssignmentClause}.
 * @param ctx the parse tree
 */
fn enter_columnAssignmentClause(&mut self, _ctx: &ColumnAssignmentClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnAssignmentClause}.
 * @param ctx the parse tree
 */
fn exit_columnAssignmentClause(&mut self, _ctx: &ColumnAssignmentClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedencePlusExpressionOrDefault}.
 * @param ctx the parse tree
 */
fn enter_precedencePlusExpressionOrDefault(&mut self, _ctx: &PrecedencePlusExpressionOrDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedencePlusExpressionOrDefault}.
 * @param ctx the parse tree
 */
fn exit_precedencePlusExpressionOrDefault(&mut self, _ctx: &PrecedencePlusExpressionOrDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#setColumnsClause}.
 * @param ctx the parse tree
 */
fn enter_setColumnsClause(&mut self, _ctx: &SetColumnsClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#setColumnsClause}.
 * @param ctx the parse tree
 */
fn exit_setColumnsClause(&mut self, _ctx: &SetColumnsClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#updateStatement}.
 * @param ctx the parse tree
 */
fn enter_updateStatement(&mut self, _ctx: &UpdateStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#updateStatement}.
 * @param ctx the parse tree
 */
fn exit_updateStatement(&mut self, _ctx: &UpdateStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#sqlTransactionStatement}.
 * @param ctx the parse tree
 */
fn enter_sqlTransactionStatement(&mut self, _ctx: &SqlTransactionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#sqlTransactionStatement}.
 * @param ctx the parse tree
 */
fn exit_sqlTransactionStatement(&mut self, _ctx: &SqlTransactionStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#startTransactionStatement}.
 * @param ctx the parse tree
 */
fn enter_startTransactionStatement(&mut self, _ctx: &StartTransactionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#startTransactionStatement}.
 * @param ctx the parse tree
 */
fn exit_startTransactionStatement(&mut self, _ctx: &StartTransactionStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#transactionMode}.
 * @param ctx the parse tree
 */
fn enter_transactionMode(&mut self, _ctx: &TransactionModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#transactionMode}.
 * @param ctx the parse tree
 */
fn exit_transactionMode(&mut self, _ctx: &TransactionModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#transactionAccessMode}.
 * @param ctx the parse tree
 */
fn enter_transactionAccessMode(&mut self, _ctx: &TransactionAccessModeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#transactionAccessMode}.
 * @param ctx the parse tree
 */
fn exit_transactionAccessMode(&mut self, _ctx: &TransactionAccessModeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#isolationLevel}.
 * @param ctx the parse tree
 */
fn enter_isolationLevel(&mut self, _ctx: &IsolationLevelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#isolationLevel}.
 * @param ctx the parse tree
 */
fn exit_isolationLevel(&mut self, _ctx: &IsolationLevelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#levelOfIsolation}.
 * @param ctx the parse tree
 */
fn enter_levelOfIsolation(&mut self, _ctx: &LevelOfIsolationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#levelOfIsolation}.
 * @param ctx the parse tree
 */
fn exit_levelOfIsolation(&mut self, _ctx: &LevelOfIsolationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#commitStatement}.
 * @param ctx the parse tree
 */
fn enter_commitStatement(&mut self, _ctx: &CommitStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#commitStatement}.
 * @param ctx the parse tree
 */
fn exit_commitStatement(&mut self, _ctx: &CommitStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#rollbackStatement}.
 * @param ctx the parse tree
 */
fn enter_rollbackStatement(&mut self, _ctx: &RollbackStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#rollbackStatement}.
 * @param ctx the parse tree
 */
fn exit_rollbackStatement(&mut self, _ctx: &RollbackStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#setAutoCommitStatement}.
 * @param ctx the parse tree
 */
fn enter_setAutoCommitStatement(&mut self, _ctx: &SetAutoCommitStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#setAutoCommitStatement}.
 * @param ctx the parse tree
 */
fn exit_setAutoCommitStatement(&mut self, _ctx: &SetAutoCommitStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#abortTransactionStatement}.
 * @param ctx the parse tree
 */
fn enter_abortTransactionStatement(&mut self, _ctx: &AbortTransactionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#abortTransactionStatement}.
 * @param ctx the parse tree
 */
fn exit_abortTransactionStatement(&mut self, _ctx: &AbortTransactionStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#abortCompactionStatement}.
 * @param ctx the parse tree
 */
fn enter_abortCompactionStatement(&mut self, _ctx: &AbortCompactionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#abortCompactionStatement}.
 * @param ctx the parse tree
 */
fn exit_abortCompactionStatement(&mut self, _ctx: &AbortCompactionStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#mergeStatement}.
 * @param ctx the parse tree
 */
fn enter_mergeStatement(&mut self, _ctx: &MergeStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#mergeStatement}.
 * @param ctx the parse tree
 */
fn exit_mergeStatement(&mut self, _ctx: &MergeStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#whenClauses}.
 * @param ctx the parse tree
 */
fn enter_whenClauses(&mut self, _ctx: &WhenClausesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#whenClauses}.
 * @param ctx the parse tree
 */
fn exit_whenClauses(&mut self, _ctx: &WhenClausesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#whenNotMatchedClause}.
 * @param ctx the parse tree
 */
fn enter_whenNotMatchedClause(&mut self, _ctx: &WhenNotMatchedClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#whenNotMatchedClause}.
 * @param ctx the parse tree
 */
fn exit_whenNotMatchedClause(&mut self, _ctx: &WhenNotMatchedClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#whenMatchedAndClause}.
 * @param ctx the parse tree
 */
fn enter_whenMatchedAndClause(&mut self, _ctx: &WhenMatchedAndClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#whenMatchedAndClause}.
 * @param ctx the parse tree
 */
fn exit_whenMatchedAndClause(&mut self, _ctx: &WhenMatchedAndClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#whenMatchedThenClause}.
 * @param ctx the parse tree
 */
fn enter_whenMatchedThenClause(&mut self, _ctx: &WhenMatchedThenClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#whenMatchedThenClause}.
 * @param ctx the parse tree
 */
fn exit_whenMatchedThenClause(&mut self, _ctx: &WhenMatchedThenClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#updateOrDelete}.
 * @param ctx the parse tree
 */
fn enter_updateOrDelete(&mut self, _ctx: &UpdateOrDeleteContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#updateOrDelete}.
 * @param ctx the parse tree
 */
fn exit_updateOrDelete(&mut self, _ctx: &UpdateOrDeleteContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#killQueryStatement}.
 * @param ctx the parse tree
 */
fn enter_killQueryStatement(&mut self, _ctx: &KillQueryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#killQueryStatement}.
 * @param ctx the parse tree
 */
fn exit_killQueryStatement(&mut self, _ctx: &KillQueryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#compactionId}.
 * @param ctx the parse tree
 */
fn enter_compactionId(&mut self, _ctx: &CompactionIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#compactionId}.
 * @param ctx the parse tree
 */
fn exit_compactionId(&mut self, _ctx: &CompactionIdContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#compactionPool}.
 * @param ctx the parse tree
 */
fn enter_compactionPool(&mut self, _ctx: &CompactionPoolContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#compactionPool}.
 * @param ctx the parse tree
 */
fn exit_compactionPool(&mut self, _ctx: &CompactionPoolContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#compactionType}.
 * @param ctx the parse tree
 */
fn enter_compactionType(&mut self, _ctx: &CompactionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#compactionType}.
 * @param ctx the parse tree
 */
fn exit_compactionType(&mut self, _ctx: &CompactionTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#compactionStatus}.
 * @param ctx the parse tree
 */
fn enter_compactionStatus(&mut self, _ctx: &CompactionStatusContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#compactionStatus}.
 * @param ctx the parse tree
 */
fn exit_compactionStatus(&mut self, _ctx: &CompactionStatusContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatement}.
 * @param ctx the parse tree
 */
fn enter_alterStatement(&mut self, _ctx: &AlterStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatement}.
 * @param ctx the parse tree
 */
fn exit_alterStatement(&mut self, _ctx: &AlterStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterTableStatementSuffix}.
 * @param ctx the parse tree
 */
fn enter_alterTableStatementSuffix(&mut self, _ctx: &AlterTableStatementSuffixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterTableStatementSuffix}.
 * @param ctx the parse tree
 */
fn exit_alterTableStatementSuffix(&mut self, _ctx: &AlterTableStatementSuffixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterTblPartitionStatementSuffix}.
 * @param ctx the parse tree
 */
fn enter_alterTblPartitionStatementSuffix(&mut self, _ctx: &AlterTblPartitionStatementSuffixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterTblPartitionStatementSuffix}.
 * @param ctx the parse tree
 */
fn exit_alterTblPartitionStatementSuffix(&mut self, _ctx: &AlterTblPartitionStatementSuffixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementPartitionKeyType}.
 * @param ctx the parse tree
 */
fn enter_alterStatementPartitionKeyType(&mut self, _ctx: &AlterStatementPartitionKeyTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementPartitionKeyType}.
 * @param ctx the parse tree
 */
fn exit_alterStatementPartitionKeyType(&mut self, _ctx: &AlterStatementPartitionKeyTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterViewStatementSuffix}.
 * @param ctx the parse tree
 */
fn enter_alterViewStatementSuffix(&mut self, _ctx: &AlterViewStatementSuffixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterViewStatementSuffix}.
 * @param ctx the parse tree
 */
fn exit_alterViewStatementSuffix(&mut self, _ctx: &AlterViewStatementSuffixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterMaterializedViewStatementSuffix}.
 * @param ctx the parse tree
 */
fn enter_alterMaterializedViewStatementSuffix(&mut self, _ctx: &AlterMaterializedViewStatementSuffixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterMaterializedViewStatementSuffix}.
 * @param ctx the parse tree
 */
fn exit_alterMaterializedViewStatementSuffix(&mut self, _ctx: &AlterMaterializedViewStatementSuffixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterMaterializedViewSuffixRewrite}.
 * @param ctx the parse tree
 */
fn enter_alterMaterializedViewSuffixRewrite(&mut self, _ctx: &AlterMaterializedViewSuffixRewriteContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterMaterializedViewSuffixRewrite}.
 * @param ctx the parse tree
 */
fn exit_alterMaterializedViewSuffixRewrite(&mut self, _ctx: &AlterMaterializedViewSuffixRewriteContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterMaterializedViewSuffixRebuild}.
 * @param ctx the parse tree
 */
fn enter_alterMaterializedViewSuffixRebuild(&mut self, _ctx: &AlterMaterializedViewSuffixRebuildContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterMaterializedViewSuffixRebuild}.
 * @param ctx the parse tree
 */
fn exit_alterMaterializedViewSuffixRebuild(&mut self, _ctx: &AlterMaterializedViewSuffixRebuildContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterDatabaseStatementSuffix}.
 * @param ctx the parse tree
 */
fn enter_alterDatabaseStatementSuffix(&mut self, _ctx: &AlterDatabaseStatementSuffixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterDatabaseStatementSuffix}.
 * @param ctx the parse tree
 */
fn exit_alterDatabaseStatementSuffix(&mut self, _ctx: &AlterDatabaseStatementSuffixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterDatabaseSuffixProperties}.
 * @param ctx the parse tree
 */
fn enter_alterDatabaseSuffixProperties(&mut self, _ctx: &AlterDatabaseSuffixPropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterDatabaseSuffixProperties}.
 * @param ctx the parse tree
 */
fn exit_alterDatabaseSuffixProperties(&mut self, _ctx: &AlterDatabaseSuffixPropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterDatabaseSuffixSetOwner}.
 * @param ctx the parse tree
 */
fn enter_alterDatabaseSuffixSetOwner(&mut self, _ctx: &AlterDatabaseSuffixSetOwnerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterDatabaseSuffixSetOwner}.
 * @param ctx the parse tree
 */
fn exit_alterDatabaseSuffixSetOwner(&mut self, _ctx: &AlterDatabaseSuffixSetOwnerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterDatabaseSuffixSetLocation}.
 * @param ctx the parse tree
 */
fn enter_alterDatabaseSuffixSetLocation(&mut self, _ctx: &AlterDatabaseSuffixSetLocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterDatabaseSuffixSetLocation}.
 * @param ctx the parse tree
 */
fn exit_alterDatabaseSuffixSetLocation(&mut self, _ctx: &AlterDatabaseSuffixSetLocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterDatabaseSuffixSetManagedLocation}.
 * @param ctx the parse tree
 */
fn enter_alterDatabaseSuffixSetManagedLocation(&mut self, _ctx: &AlterDatabaseSuffixSetManagedLocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterDatabaseSuffixSetManagedLocation}.
 * @param ctx the parse tree
 */
fn exit_alterDatabaseSuffixSetManagedLocation(&mut self, _ctx: &AlterDatabaseSuffixSetManagedLocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixRename}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixRename(&mut self, _ctx: &AlterStatementSuffixRenameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixRename}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixRename(&mut self, _ctx: &AlterStatementSuffixRenameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixAddCol}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixAddCol(&mut self, _ctx: &AlterStatementSuffixAddColContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixAddCol}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixAddCol(&mut self, _ctx: &AlterStatementSuffixAddColContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixAddConstraint}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixAddConstraint(&mut self, _ctx: &AlterStatementSuffixAddConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixAddConstraint}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixAddConstraint(&mut self, _ctx: &AlterStatementSuffixAddConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixUpdateColumns}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixUpdateColumns(&mut self, _ctx: &AlterStatementSuffixUpdateColumnsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixUpdateColumns}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixUpdateColumns(&mut self, _ctx: &AlterStatementSuffixUpdateColumnsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixProtections}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixProtections(&mut self, _ctx: &AlterStatementSuffixProtectionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixProtections}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixProtections(&mut self, _ctx: &AlterStatementSuffixProtectionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixDropConstraint}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixDropConstraint(&mut self, _ctx: &AlterStatementSuffixDropConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixDropConstraint}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixDropConstraint(&mut self, _ctx: &AlterStatementSuffixDropConstraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixRenameCol}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixRenameCol(&mut self, _ctx: &AlterStatementSuffixRenameColContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixRenameCol}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixRenameCol(&mut self, _ctx: &AlterStatementSuffixRenameColContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixUpdateStatsCol}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixUpdateStatsCol(&mut self, _ctx: &AlterStatementSuffixUpdateStatsColContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixUpdateStatsCol}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixUpdateStatsCol(&mut self, _ctx: &AlterStatementSuffixUpdateStatsColContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixUpdateStats}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixUpdateStats(&mut self, _ctx: &AlterStatementSuffixUpdateStatsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixUpdateStats}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixUpdateStats(&mut self, _ctx: &AlterStatementSuffixUpdateStatsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementChangeColPosition}.
 * @param ctx the parse tree
 */
fn enter_alterStatementChangeColPosition(&mut self, _ctx: &AlterStatementChangeColPositionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementChangeColPosition}.
 * @param ctx the parse tree
 */
fn exit_alterStatementChangeColPosition(&mut self, _ctx: &AlterStatementChangeColPositionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixAddPartitions}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixAddPartitions(&mut self, _ctx: &AlterStatementSuffixAddPartitionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixAddPartitions}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixAddPartitions(&mut self, _ctx: &AlterStatementSuffixAddPartitionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixAddPartitionsElement}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixAddPartitionsElement(&mut self, _ctx: &AlterStatementSuffixAddPartitionsElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixAddPartitionsElement}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixAddPartitionsElement(&mut self, _ctx: &AlterStatementSuffixAddPartitionsElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixTouch}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixTouch(&mut self, _ctx: &AlterStatementSuffixTouchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixTouch}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixTouch(&mut self, _ctx: &AlterStatementSuffixTouchContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixArchive}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixArchive(&mut self, _ctx: &AlterStatementSuffixArchiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixArchive}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixArchive(&mut self, _ctx: &AlterStatementSuffixArchiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixUnArchive}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixUnArchive(&mut self, _ctx: &AlterStatementSuffixUnArchiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixUnArchive}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixUnArchive(&mut self, _ctx: &AlterStatementSuffixUnArchiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partitionLocation}.
 * @param ctx the parse tree
 */
fn enter_partitionLocation(&mut self, _ctx: &PartitionLocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partitionLocation}.
 * @param ctx the parse tree
 */
fn exit_partitionLocation(&mut self, _ctx: &PartitionLocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixDropPartitions}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixDropPartitions(&mut self, _ctx: &AlterStatementSuffixDropPartitionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixDropPartitions}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixDropPartitions(&mut self, _ctx: &AlterStatementSuffixDropPartitionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixProperties}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixProperties(&mut self, _ctx: &AlterStatementSuffixPropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixProperties}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixProperties(&mut self, _ctx: &AlterStatementSuffixPropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterViewSuffixProperties}.
 * @param ctx the parse tree
 */
fn enter_alterViewSuffixProperties(&mut self, _ctx: &AlterViewSuffixPropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterViewSuffixProperties}.
 * @param ctx the parse tree
 */
fn exit_alterViewSuffixProperties(&mut self, _ctx: &AlterViewSuffixPropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixSerdeProperties}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixSerdeProperties(&mut self, _ctx: &AlterStatementSuffixSerdePropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixSerdeProperties}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixSerdeProperties(&mut self, _ctx: &AlterStatementSuffixSerdePropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tablePartitionPrefix}.
 * @param ctx the parse tree
 */
fn enter_tablePartitionPrefix(&mut self, _ctx: &TablePartitionPrefixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tablePartitionPrefix}.
 * @param ctx the parse tree
 */
fn exit_tablePartitionPrefix(&mut self, _ctx: &TablePartitionPrefixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixFileFormat}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixFileFormat(&mut self, _ctx: &AlterStatementSuffixFileFormatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixFileFormat}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixFileFormat(&mut self, _ctx: &AlterStatementSuffixFileFormatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixClusterbySortby}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixClusterbySortby(&mut self, _ctx: &AlterStatementSuffixClusterbySortbyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixClusterbySortby}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixClusterbySortby(&mut self, _ctx: &AlterStatementSuffixClusterbySortbyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterTblPartitionStatementSuffixSkewedLocation}.
 * @param ctx the parse tree
 */
fn enter_alterTblPartitionStatementSuffixSkewedLocation(&mut self, _ctx: &AlterTblPartitionStatementSuffixSkewedLocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterTblPartitionStatementSuffixSkewedLocation}.
 * @param ctx the parse tree
 */
fn exit_alterTblPartitionStatementSuffixSkewedLocation(&mut self, _ctx: &AlterTblPartitionStatementSuffixSkewedLocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#skewedLocations}.
 * @param ctx the parse tree
 */
fn enter_skewedLocations(&mut self, _ctx: &SkewedLocationsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#skewedLocations}.
 * @param ctx the parse tree
 */
fn exit_skewedLocations(&mut self, _ctx: &SkewedLocationsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#skewedLocationsList}.
 * @param ctx the parse tree
 */
fn enter_skewedLocationsList(&mut self, _ctx: &SkewedLocationsListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#skewedLocationsList}.
 * @param ctx the parse tree
 */
fn exit_skewedLocationsList(&mut self, _ctx: &SkewedLocationsListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#skewedLocationMap}.
 * @param ctx the parse tree
 */
fn enter_skewedLocationMap(&mut self, _ctx: &SkewedLocationMapContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#skewedLocationMap}.
 * @param ctx the parse tree
 */
fn exit_skewedLocationMap(&mut self, _ctx: &SkewedLocationMapContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixLocation}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixLocation(&mut self, _ctx: &AlterStatementSuffixLocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixLocation}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixLocation(&mut self, _ctx: &AlterStatementSuffixLocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixSkewedby}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixSkewedby(&mut self, _ctx: &AlterStatementSuffixSkewedbyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixSkewedby}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixSkewedby(&mut self, _ctx: &AlterStatementSuffixSkewedbyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixExchangePartition}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixExchangePartition(&mut self, _ctx: &AlterStatementSuffixExchangePartitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixExchangePartition}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixExchangePartition(&mut self, _ctx: &AlterStatementSuffixExchangePartitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixRenamePart}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixRenamePart(&mut self, _ctx: &AlterStatementSuffixRenamePartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixRenamePart}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixRenamePart(&mut self, _ctx: &AlterStatementSuffixRenamePartContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixStatsPart}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixStatsPart(&mut self, _ctx: &AlterStatementSuffixStatsPartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixStatsPart}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixStatsPart(&mut self, _ctx: &AlterStatementSuffixStatsPartContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixMergeFiles}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixMergeFiles(&mut self, _ctx: &AlterStatementSuffixMergeFilesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixMergeFiles}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixMergeFiles(&mut self, _ctx: &AlterStatementSuffixMergeFilesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixBucketNum}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixBucketNum(&mut self, _ctx: &AlterStatementSuffixBucketNumContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixBucketNum}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixBucketNum(&mut self, _ctx: &AlterStatementSuffixBucketNumContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#blocking}.
 * @param ctx the parse tree
 */
fn enter_blocking(&mut self, _ctx: &BlockingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#blocking}.
 * @param ctx the parse tree
 */
fn exit_blocking(&mut self, _ctx: &BlockingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#compactPool}.
 * @param ctx the parse tree
 */
fn enter_compactPool(&mut self, _ctx: &CompactPoolContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#compactPool}.
 * @param ctx the parse tree
 */
fn exit_compactPool(&mut self, _ctx: &CompactPoolContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixCompact}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixCompact(&mut self, _ctx: &AlterStatementSuffixCompactContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixCompact}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixCompact(&mut self, _ctx: &AlterStatementSuffixCompactContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixSetOwner}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixSetOwner(&mut self, _ctx: &AlterStatementSuffixSetOwnerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixSetOwner}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixSetOwner(&mut self, _ctx: &AlterStatementSuffixSetOwnerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixSetPartSpec}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixSetPartSpec(&mut self, _ctx: &AlterStatementSuffixSetPartSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixSetPartSpec}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixSetPartSpec(&mut self, _ctx: &AlterStatementSuffixSetPartSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterStatementSuffixExecute}.
 * @param ctx the parse tree
 */
fn enter_alterStatementSuffixExecute(&mut self, _ctx: &AlterStatementSuffixExecuteContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterStatementSuffixExecute}.
 * @param ctx the parse tree
 */
fn exit_alterStatementSuffixExecute(&mut self, _ctx: &AlterStatementSuffixExecuteContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterIndexStatementSuffix}.
 * @param ctx the parse tree
 */
fn enter_alterIndexStatementSuffix(&mut self, _ctx: &AlterIndexStatementSuffixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterIndexStatementSuffix}.
 * @param ctx the parse tree
 */
fn exit_alterIndexStatementSuffix(&mut self, _ctx: &AlterIndexStatementSuffixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#fileFormat}.
 * @param ctx the parse tree
 */
fn enter_fileFormat(&mut self, _ctx: &FileFormatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#fileFormat}.
 * @param ctx the parse tree
 */
fn exit_fileFormat(&mut self, _ctx: &FileFormatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterDataConnectorStatementSuffix}.
 * @param ctx the parse tree
 */
fn enter_alterDataConnectorStatementSuffix(&mut self, _ctx: &AlterDataConnectorStatementSuffixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterDataConnectorStatementSuffix}.
 * @param ctx the parse tree
 */
fn exit_alterDataConnectorStatementSuffix(&mut self, _ctx: &AlterDataConnectorStatementSuffixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterDataConnectorSuffixProperties}.
 * @param ctx the parse tree
 */
fn enter_alterDataConnectorSuffixProperties(&mut self, _ctx: &AlterDataConnectorSuffixPropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterDataConnectorSuffixProperties}.
 * @param ctx the parse tree
 */
fn exit_alterDataConnectorSuffixProperties(&mut self, _ctx: &AlterDataConnectorSuffixPropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterDataConnectorSuffixSetOwner}.
 * @param ctx the parse tree
 */
fn enter_alterDataConnectorSuffixSetOwner(&mut self, _ctx: &AlterDataConnectorSuffixSetOwnerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterDataConnectorSuffixSetOwner}.
 * @param ctx the parse tree
 */
fn exit_alterDataConnectorSuffixSetOwner(&mut self, _ctx: &AlterDataConnectorSuffixSetOwnerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterDataConnectorSuffixSetUrl}.
 * @param ctx the parse tree
 */
fn enter_alterDataConnectorSuffixSetUrl(&mut self, _ctx: &AlterDataConnectorSuffixSetUrlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterDataConnectorSuffixSetUrl}.
 * @param ctx the parse tree
 */
fn exit_alterDataConnectorSuffixSetUrl(&mut self, _ctx: &AlterDataConnectorSuffixSetUrlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#likeTableOrFile}.
 * @param ctx the parse tree
 */
fn enter_likeTableOrFile(&mut self, _ctx: &LikeTableOrFileContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#likeTableOrFile}.
 * @param ctx the parse tree
 */
fn exit_likeTableOrFile(&mut self, _ctx: &LikeTableOrFileContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createTableStatement}.
 * @param ctx the parse tree
 */
fn enter_createTableStatement(&mut self, _ctx: &CreateTableStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createTableStatement}.
 * @param ctx the parse tree
 */
fn exit_createTableStatement(&mut self, _ctx: &CreateTableStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createDataConnectorStatement}.
 * @param ctx the parse tree
 */
fn enter_createDataConnectorStatement(&mut self, _ctx: &CreateDataConnectorStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createDataConnectorStatement}.
 * @param ctx the parse tree
 */
fn exit_createDataConnectorStatement(&mut self, _ctx: &CreateDataConnectorStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dataConnectorComment}.
 * @param ctx the parse tree
 */
fn enter_dataConnectorComment(&mut self, _ctx: &DataConnectorCommentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dataConnectorComment}.
 * @param ctx the parse tree
 */
fn exit_dataConnectorComment(&mut self, _ctx: &DataConnectorCommentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dataConnectorUrl}.
 * @param ctx the parse tree
 */
fn enter_dataConnectorUrl(&mut self, _ctx: &DataConnectorUrlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dataConnectorUrl}.
 * @param ctx the parse tree
 */
fn exit_dataConnectorUrl(&mut self, _ctx: &DataConnectorUrlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dataConnectorType}.
 * @param ctx the parse tree
 */
fn enter_dataConnectorType(&mut self, _ctx: &DataConnectorTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dataConnectorType}.
 * @param ctx the parse tree
 */
fn exit_dataConnectorType(&mut self, _ctx: &DataConnectorTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dcProperties}.
 * @param ctx the parse tree
 */
fn enter_dcProperties(&mut self, _ctx: &DcPropertiesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dcProperties}.
 * @param ctx the parse tree
 */
fn exit_dcProperties(&mut self, _ctx: &DcPropertiesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropDataConnectorStatement}.
 * @param ctx the parse tree
 */
fn enter_dropDataConnectorStatement(&mut self, _ctx: &DropDataConnectorStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropDataConnectorStatement}.
 * @param ctx the parse tree
 */
fn exit_dropDataConnectorStatement(&mut self, _ctx: &DropDataConnectorStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableAllColumns}.
 * @param ctx the parse tree
 */
fn enter_tableAllColumns(&mut self, _ctx: &TableAllColumnsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableAllColumns}.
 * @param ctx the parse tree
 */
fn exit_tableAllColumns(&mut self, _ctx: &TableAllColumnsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableOrColumn}.
 * @param ctx the parse tree
 */
fn enter_tableOrColumn(&mut self, _ctx: &TableOrColumnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableOrColumn}.
 * @param ctx the parse tree
 */
fn exit_tableOrColumn(&mut self, _ctx: &TableOrColumnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#defaultValue}.
 * @param ctx the parse tree
 */
fn enter_defaultValue(&mut self, _ctx: &DefaultValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#defaultValue}.
 * @param ctx the parse tree
 */
fn exit_defaultValue(&mut self, _ctx: &DefaultValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#expressionList}.
 * @param ctx the parse tree
 */
fn enter_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#expressionList}.
 * @param ctx the parse tree
 */
fn exit_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#aliasList}.
 * @param ctx the parse tree
 */
fn enter_aliasList(&mut self, _ctx: &AliasListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#aliasList}.
 * @param ctx the parse tree
 */
fn exit_aliasList(&mut self, _ctx: &AliasListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#fromClause}.
 * @param ctx the parse tree
 */
fn enter_fromClause(&mut self, _ctx: &FromClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#fromClause}.
 * @param ctx the parse tree
 */
fn exit_fromClause(&mut self, _ctx: &FromClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#fromSource}.
 * @param ctx the parse tree
 */
fn enter_fromSource(&mut self, _ctx: &FromSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#fromSource}.
 * @param ctx the parse tree
 */
fn exit_fromSource(&mut self, _ctx: &FromSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#atomjoinSource}.
 * @param ctx the parse tree
 */
fn enter_atomjoinSource(&mut self, _ctx: &AtomjoinSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#atomjoinSource}.
 * @param ctx the parse tree
 */
fn exit_atomjoinSource(&mut self, _ctx: &AtomjoinSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#joinSource}.
 * @param ctx the parse tree
 */
fn enter_joinSource(&mut self, _ctx: &JoinSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#joinSource}.
 * @param ctx the parse tree
 */
fn exit_joinSource(&mut self, _ctx: &JoinSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#joinSourcePart}.
 * @param ctx the parse tree
 */
fn enter_joinSourcePart(&mut self, _ctx: &JoinSourcePartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#joinSourcePart}.
 * @param ctx the parse tree
 */
fn exit_joinSourcePart(&mut self, _ctx: &JoinSourcePartContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#uniqueJoinSource}.
 * @param ctx the parse tree
 */
fn enter_uniqueJoinSource(&mut self, _ctx: &UniqueJoinSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#uniqueJoinSource}.
 * @param ctx the parse tree
 */
fn exit_uniqueJoinSource(&mut self, _ctx: &UniqueJoinSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#uniqueJoinExpr}.
 * @param ctx the parse tree
 */
fn enter_uniqueJoinExpr(&mut self, _ctx: &UniqueJoinExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#uniqueJoinExpr}.
 * @param ctx the parse tree
 */
fn exit_uniqueJoinExpr(&mut self, _ctx: &UniqueJoinExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#uniqueJoinToken}.
 * @param ctx the parse tree
 */
fn enter_uniqueJoinToken(&mut self, _ctx: &UniqueJoinTokenContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#uniqueJoinToken}.
 * @param ctx the parse tree
 */
fn exit_uniqueJoinToken(&mut self, _ctx: &UniqueJoinTokenContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#joinToken}.
 * @param ctx the parse tree
 */
fn enter_joinToken(&mut self, _ctx: &JoinTokenContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#joinToken}.
 * @param ctx the parse tree
 */
fn exit_joinToken(&mut self, _ctx: &JoinTokenContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#lateralView}.
 * @param ctx the parse tree
 */
fn enter_lateralView(&mut self, _ctx: &LateralViewContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#lateralView}.
 * @param ctx the parse tree
 */
fn exit_lateralView(&mut self, _ctx: &LateralViewContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableAlias}.
 * @param ctx the parse tree
 */
fn enter_tableAlias(&mut self, _ctx: &TableAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableAlias}.
 * @param ctx the parse tree
 */
fn exit_tableAlias(&mut self, _ctx: &TableAliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableBucketSample}.
 * @param ctx the parse tree
 */
fn enter_tableBucketSample(&mut self, _ctx: &TableBucketSampleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableBucketSample}.
 * @param ctx the parse tree
 */
fn exit_tableBucketSample(&mut self, _ctx: &TableBucketSampleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#splitSample}.
 * @param ctx the parse tree
 */
fn enter_splitSample(&mut self, _ctx: &SplitSampleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#splitSample}.
 * @param ctx the parse tree
 */
fn exit_splitSample(&mut self, _ctx: &SplitSampleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableSample}.
 * @param ctx the parse tree
 */
fn enter_tableSample(&mut self, _ctx: &TableSampleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableSample}.
 * @param ctx the parse tree
 */
fn exit_tableSample(&mut self, _ctx: &TableSampleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableSource}.
 * @param ctx the parse tree
 */
fn enter_tableSource(&mut self, _ctx: &TableSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableSource}.
 * @param ctx the parse tree
 */
fn exit_tableSource(&mut self, _ctx: &TableSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#asOfClause}.
 * @param ctx the parse tree
 */
fn enter_asOfClause(&mut self, _ctx: &AsOfClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#asOfClause}.
 * @param ctx the parse tree
 */
fn exit_asOfClause(&mut self, _ctx: &AsOfClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#uniqueJoinTableSource}.
 * @param ctx the parse tree
 */
fn enter_uniqueJoinTableSource(&mut self, _ctx: &UniqueJoinTableSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#uniqueJoinTableSource}.
 * @param ctx the parse tree
 */
fn exit_uniqueJoinTableSource(&mut self, _ctx: &UniqueJoinTableSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableName}.
 * @param ctx the parse tree
 */
fn enter_tableName(&mut self, _ctx: &TableNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableName}.
 * @param ctx the parse tree
 */
fn exit_tableName(&mut self, _ctx: &TableNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#viewName}.
 * @param ctx the parse tree
 */
fn enter_viewName(&mut self, _ctx: &ViewNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#viewName}.
 * @param ctx the parse tree
 */
fn exit_viewName(&mut self, _ctx: &ViewNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#subQuerySource}.
 * @param ctx the parse tree
 */
fn enter_subQuerySource(&mut self, _ctx: &SubQuerySourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#subQuerySource}.
 * @param ctx the parse tree
 */
fn exit_subQuerySource(&mut self, _ctx: &SubQuerySourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partitioningSpec}.
 * @param ctx the parse tree
 */
fn enter_partitioningSpec(&mut self, _ctx: &PartitioningSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partitioningSpec}.
 * @param ctx the parse tree
 */
fn exit_partitioningSpec(&mut self, _ctx: &PartitioningSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partitionTableFunctionSource}.
 * @param ctx the parse tree
 */
fn enter_partitionTableFunctionSource(&mut self, _ctx: &PartitionTableFunctionSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partitionTableFunctionSource}.
 * @param ctx the parse tree
 */
fn exit_partitionTableFunctionSource(&mut self, _ctx: &PartitionTableFunctionSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partitionedTableFunction}.
 * @param ctx the parse tree
 */
fn enter_partitionedTableFunction(&mut self, _ctx: &PartitionedTableFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partitionedTableFunction}.
 * @param ctx the parse tree
 */
fn exit_partitionedTableFunction(&mut self, _ctx: &PartitionedTableFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#whereClause}.
 * @param ctx the parse tree
 */
fn enter_whereClause(&mut self, _ctx: &WhereClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#whereClause}.
 * @param ctx the parse tree
 */
fn exit_whereClause(&mut self, _ctx: &WhereClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#searchCondition}.
 * @param ctx the parse tree
 */
fn enter_searchCondition(&mut self, _ctx: &SearchConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#searchCondition}.
 * @param ctx the parse tree
 */
fn exit_searchCondition(&mut self, _ctx: &SearchConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#valuesSource}.
 * @param ctx the parse tree
 */
fn enter_valuesSource(&mut self, _ctx: &ValuesSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#valuesSource}.
 * @param ctx the parse tree
 */
fn exit_valuesSource(&mut self, _ctx: &ValuesSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#valuesClause}.
 * @param ctx the parse tree
 */
fn enter_valuesClause(&mut self, _ctx: &ValuesClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#valuesClause}.
 * @param ctx the parse tree
 */
fn exit_valuesClause(&mut self, _ctx: &ValuesClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#valuesTableConstructor}.
 * @param ctx the parse tree
 */
fn enter_valuesTableConstructor(&mut self, _ctx: &ValuesTableConstructorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#valuesTableConstructor}.
 * @param ctx the parse tree
 */
fn exit_valuesTableConstructor(&mut self, _ctx: &ValuesTableConstructorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#valueRowConstructor}.
 * @param ctx the parse tree
 */
fn enter_valueRowConstructor(&mut self, _ctx: &ValueRowConstructorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#valueRowConstructor}.
 * @param ctx the parse tree
 */
fn exit_valueRowConstructor(&mut self, _ctx: &ValueRowConstructorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#firstValueRowConstructor}.
 * @param ctx the parse tree
 */
fn enter_firstValueRowConstructor(&mut self, _ctx: &FirstValueRowConstructorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#firstValueRowConstructor}.
 * @param ctx the parse tree
 */
fn exit_firstValueRowConstructor(&mut self, _ctx: &FirstValueRowConstructorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#virtualTableSource}.
 * @param ctx the parse tree
 */
fn enter_virtualTableSource(&mut self, _ctx: &VirtualTableSourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#virtualTableSource}.
 * @param ctx the parse tree
 */
fn exit_virtualTableSource(&mut self, _ctx: &VirtualTableSourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#selectClause}.
 * @param ctx the parse tree
 */
fn enter_selectClause(&mut self, _ctx: &SelectClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#selectClause}.
 * @param ctx the parse tree
 */
fn exit_selectClause(&mut self, _ctx: &SelectClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#all_distinct}.
 * @param ctx the parse tree
 */
fn enter_all_distinct(&mut self, _ctx: &All_distinctContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#all_distinct}.
 * @param ctx the parse tree
 */
fn exit_all_distinct(&mut self, _ctx: &All_distinctContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#selectList}.
 * @param ctx the parse tree
 */
fn enter_selectList(&mut self, _ctx: &SelectListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#selectList}.
 * @param ctx the parse tree
 */
fn exit_selectList(&mut self, _ctx: &SelectListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#selectTrfmClause}.
 * @param ctx the parse tree
 */
fn enter_selectTrfmClause(&mut self, _ctx: &SelectTrfmClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#selectTrfmClause}.
 * @param ctx the parse tree
 */
fn exit_selectTrfmClause(&mut self, _ctx: &SelectTrfmClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#selectItem}.
 * @param ctx the parse tree
 */
fn enter_selectItem(&mut self, _ctx: &SelectItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#selectItem}.
 * @param ctx the parse tree
 */
fn exit_selectItem(&mut self, _ctx: &SelectItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#trfmClause}.
 * @param ctx the parse tree
 */
fn enter_trfmClause(&mut self, _ctx: &TrfmClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#trfmClause}.
 * @param ctx the parse tree
 */
fn exit_trfmClause(&mut self, _ctx: &TrfmClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#selectExpression}.
 * @param ctx the parse tree
 */
fn enter_selectExpression(&mut self, _ctx: &SelectExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#selectExpression}.
 * @param ctx the parse tree
 */
fn exit_selectExpression(&mut self, _ctx: &SelectExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#selectExpressionList}.
 * @param ctx the parse tree
 */
fn enter_selectExpressionList(&mut self, _ctx: &SelectExpressionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#selectExpressionList}.
 * @param ctx the parse tree
 */
fn exit_selectExpressionList(&mut self, _ctx: &SelectExpressionListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#window_clause}.
 * @param ctx the parse tree
 */
fn enter_window_clause(&mut self, _ctx: &Window_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#window_clause}.
 * @param ctx the parse tree
 */
fn exit_window_clause(&mut self, _ctx: &Window_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#window_defn}.
 * @param ctx the parse tree
 */
fn enter_window_defn(&mut self, _ctx: &Window_defnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#window_defn}.
 * @param ctx the parse tree
 */
fn exit_window_defn(&mut self, _ctx: &Window_defnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#window_specification}.
 * @param ctx the parse tree
 */
fn enter_window_specification(&mut self, _ctx: &Window_specificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#window_specification}.
 * @param ctx the parse tree
 */
fn exit_window_specification(&mut self, _ctx: &Window_specificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#window_frame}.
 * @param ctx the parse tree
 */
fn enter_window_frame(&mut self, _ctx: &Window_frameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#window_frame}.
 * @param ctx the parse tree
 */
fn exit_window_frame(&mut self, _ctx: &Window_frameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#window_range_expression}.
 * @param ctx the parse tree
 */
fn enter_window_range_expression(&mut self, _ctx: &Window_range_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#window_range_expression}.
 * @param ctx the parse tree
 */
fn exit_window_range_expression(&mut self, _ctx: &Window_range_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#window_value_expression}.
 * @param ctx the parse tree
 */
fn enter_window_value_expression(&mut self, _ctx: &Window_value_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#window_value_expression}.
 * @param ctx the parse tree
 */
fn exit_window_value_expression(&mut self, _ctx: &Window_value_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#window_frame_start_boundary}.
 * @param ctx the parse tree
 */
fn enter_window_frame_start_boundary(&mut self, _ctx: &Window_frame_start_boundaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#window_frame_start_boundary}.
 * @param ctx the parse tree
 */
fn exit_window_frame_start_boundary(&mut self, _ctx: &Window_frame_start_boundaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#window_frame_boundary}.
 * @param ctx the parse tree
 */
fn enter_window_frame_boundary(&mut self, _ctx: &Window_frame_boundaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#window_frame_boundary}.
 * @param ctx the parse tree
 */
fn exit_window_frame_boundary(&mut self, _ctx: &Window_frame_boundaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#groupByClause}.
 * @param ctx the parse tree
 */
fn enter_groupByClause(&mut self, _ctx: &GroupByClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#groupByClause}.
 * @param ctx the parse tree
 */
fn exit_groupByClause(&mut self, _ctx: &GroupByClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#groupby_expression}.
 * @param ctx the parse tree
 */
fn enter_groupby_expression(&mut self, _ctx: &Groupby_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#groupby_expression}.
 * @param ctx the parse tree
 */
fn exit_groupby_expression(&mut self, _ctx: &Groupby_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#groupByEmpty}.
 * @param ctx the parse tree
 */
fn enter_groupByEmpty(&mut self, _ctx: &GroupByEmptyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#groupByEmpty}.
 * @param ctx the parse tree
 */
fn exit_groupByEmpty(&mut self, _ctx: &GroupByEmptyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#rollupStandard}.
 * @param ctx the parse tree
 */
fn enter_rollupStandard(&mut self, _ctx: &RollupStandardContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#rollupStandard}.
 * @param ctx the parse tree
 */
fn exit_rollupStandard(&mut self, _ctx: &RollupStandardContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#rollupOldSyntax}.
 * @param ctx the parse tree
 */
fn enter_rollupOldSyntax(&mut self, _ctx: &RollupOldSyntaxContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#rollupOldSyntax}.
 * @param ctx the parse tree
 */
fn exit_rollupOldSyntax(&mut self, _ctx: &RollupOldSyntaxContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#groupingSetExpression}.
 * @param ctx the parse tree
 */
fn enter_groupingSetExpression(&mut self, _ctx: &GroupingSetExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#groupingSetExpression}.
 * @param ctx the parse tree
 */
fn exit_groupingSetExpression(&mut self, _ctx: &GroupingSetExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#groupingSetExpressionMultiple}.
 * @param ctx the parse tree
 */
fn enter_groupingSetExpressionMultiple(&mut self, _ctx: &GroupingSetExpressionMultipleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#groupingSetExpressionMultiple}.
 * @param ctx the parse tree
 */
fn exit_groupingSetExpressionMultiple(&mut self, _ctx: &GroupingSetExpressionMultipleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#groupingExpressionSingle}.
 * @param ctx the parse tree
 */
fn enter_groupingExpressionSingle(&mut self, _ctx: &GroupingExpressionSingleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#groupingExpressionSingle}.
 * @param ctx the parse tree
 */
fn exit_groupingExpressionSingle(&mut self, _ctx: &GroupingExpressionSingleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#havingClause}.
 * @param ctx the parse tree
 */
fn enter_havingClause(&mut self, _ctx: &HavingClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#havingClause}.
 * @param ctx the parse tree
 */
fn exit_havingClause(&mut self, _ctx: &HavingClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#qualifyClause}.
 * @param ctx the parse tree
 */
fn enter_qualifyClause(&mut self, _ctx: &QualifyClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#qualifyClause}.
 * @param ctx the parse tree
 */
fn exit_qualifyClause(&mut self, _ctx: &QualifyClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#havingCondition}.
 * @param ctx the parse tree
 */
fn enter_havingCondition(&mut self, _ctx: &HavingConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#havingCondition}.
 * @param ctx the parse tree
 */
fn exit_havingCondition(&mut self, _ctx: &HavingConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#expressionsInParenthesis}.
 * @param ctx the parse tree
 */
fn enter_expressionsInParenthesis(&mut self, _ctx: &ExpressionsInParenthesisContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#expressionsInParenthesis}.
 * @param ctx the parse tree
 */
fn exit_expressionsInParenthesis(&mut self, _ctx: &ExpressionsInParenthesisContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#expressionsNotInParenthesis}.
 * @param ctx the parse tree
 */
fn enter_expressionsNotInParenthesis(&mut self, _ctx: &ExpressionsNotInParenthesisContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#expressionsNotInParenthesis}.
 * @param ctx the parse tree
 */
fn exit_expressionsNotInParenthesis(&mut self, _ctx: &ExpressionsNotInParenthesisContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#expressionPart}.
 * @param ctx the parse tree
 */
fn enter_expressionPart(&mut self, _ctx: &ExpressionPartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#expressionPart}.
 * @param ctx the parse tree
 */
fn exit_expressionPart(&mut self, _ctx: &ExpressionPartContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#expressionOrDefault}.
 * @param ctx the parse tree
 */
fn enter_expressionOrDefault(&mut self, _ctx: &ExpressionOrDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#expressionOrDefault}.
 * @param ctx the parse tree
 */
fn exit_expressionOrDefault(&mut self, _ctx: &ExpressionOrDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#firstExpressionsWithAlias}.
 * @param ctx the parse tree
 */
fn enter_firstExpressionsWithAlias(&mut self, _ctx: &FirstExpressionsWithAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#firstExpressionsWithAlias}.
 * @param ctx the parse tree
 */
fn exit_firstExpressionsWithAlias(&mut self, _ctx: &FirstExpressionsWithAliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#expressionWithAlias}.
 * @param ctx the parse tree
 */
fn enter_expressionWithAlias(&mut self, _ctx: &ExpressionWithAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#expressionWithAlias}.
 * @param ctx the parse tree
 */
fn exit_expressionWithAlias(&mut self, _ctx: &ExpressionWithAliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#expressions}.
 * @param ctx the parse tree
 */
fn enter_expressions(&mut self, _ctx: &ExpressionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#expressions}.
 * @param ctx the parse tree
 */
fn exit_expressions(&mut self, _ctx: &ExpressionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnRefOrderInParenthesis}.
 * @param ctx the parse tree
 */
fn enter_columnRefOrderInParenthesis(&mut self, _ctx: &ColumnRefOrderInParenthesisContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnRefOrderInParenthesis}.
 * @param ctx the parse tree
 */
fn exit_columnRefOrderInParenthesis(&mut self, _ctx: &ColumnRefOrderInParenthesisContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#columnRefOrderNotInParenthesis}.
 * @param ctx the parse tree
 */
fn enter_columnRefOrderNotInParenthesis(&mut self, _ctx: &ColumnRefOrderNotInParenthesisContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#columnRefOrderNotInParenthesis}.
 * @param ctx the parse tree
 */
fn exit_columnRefOrderNotInParenthesis(&mut self, _ctx: &ColumnRefOrderNotInParenthesisContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#orderByClause}.
 * @param ctx the parse tree
 */
fn enter_orderByClause(&mut self, _ctx: &OrderByClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#orderByClause}.
 * @param ctx the parse tree
 */
fn exit_orderByClause(&mut self, _ctx: &OrderByClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#clusterByClause}.
 * @param ctx the parse tree
 */
fn enter_clusterByClause(&mut self, _ctx: &ClusterByClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#clusterByClause}.
 * @param ctx the parse tree
 */
fn exit_clusterByClause(&mut self, _ctx: &ClusterByClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partitionByClause}.
 * @param ctx the parse tree
 */
fn enter_partitionByClause(&mut self, _ctx: &PartitionByClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partitionByClause}.
 * @param ctx the parse tree
 */
fn exit_partitionByClause(&mut self, _ctx: &PartitionByClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#distributeByClause}.
 * @param ctx the parse tree
 */
fn enter_distributeByClause(&mut self, _ctx: &DistributeByClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#distributeByClause}.
 * @param ctx the parse tree
 */
fn exit_distributeByClause(&mut self, _ctx: &DistributeByClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#sortByClause}.
 * @param ctx the parse tree
 */
fn enter_sortByClause(&mut self, _ctx: &SortByClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#sortByClause}.
 * @param ctx the parse tree
 */
fn exit_sortByClause(&mut self, _ctx: &SortByClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#trimFunction}.
 * @param ctx the parse tree
 */
fn enter_trimFunction(&mut self, _ctx: &TrimFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#trimFunction}.
 * @param ctx the parse tree
 */
fn exit_trimFunction(&mut self, _ctx: &TrimFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#function_}.
 * @param ctx the parse tree
 */
fn enter_function_(&mut self, _ctx: &Function_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#function_}.
 * @param ctx the parse tree
 */
fn exit_function_(&mut self, _ctx: &Function_Context<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#null_treatment}.
 * @param ctx the parse tree
 */
fn enter_null_treatment(&mut self, _ctx: &Null_treatmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#null_treatment}.
 * @param ctx the parse tree
 */
fn exit_null_treatment(&mut self, _ctx: &Null_treatmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#functionName}.
 * @param ctx the parse tree
 */
fn enter_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#functionName}.
 * @param ctx the parse tree
 */
fn exit_functionName(&mut self, _ctx: &FunctionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#castExpression}.
 * @param ctx the parse tree
 */
fn enter_castExpression(&mut self, _ctx: &CastExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#castExpression}.
 * @param ctx the parse tree
 */
fn exit_castExpression(&mut self, _ctx: &CastExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#caseExpression}.
 * @param ctx the parse tree
 */
fn enter_caseExpression(&mut self, _ctx: &CaseExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#caseExpression}.
 * @param ctx the parse tree
 */
fn exit_caseExpression(&mut self, _ctx: &CaseExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#whenExpression}.
 * @param ctx the parse tree
 */
fn enter_whenExpression(&mut self, _ctx: &WhenExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#whenExpression}.
 * @param ctx the parse tree
 */
fn exit_whenExpression(&mut self, _ctx: &WhenExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#floorExpression}.
 * @param ctx the parse tree
 */
fn enter_floorExpression(&mut self, _ctx: &FloorExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#floorExpression}.
 * @param ctx the parse tree
 */
fn exit_floorExpression(&mut self, _ctx: &FloorExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#floorDateQualifiers}.
 * @param ctx the parse tree
 */
fn enter_floorDateQualifiers(&mut self, _ctx: &FloorDateQualifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#floorDateQualifiers}.
 * @param ctx the parse tree
 */
fn exit_floorDateQualifiers(&mut self, _ctx: &FloorDateQualifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#extractExpression}.
 * @param ctx the parse tree
 */
fn enter_extractExpression(&mut self, _ctx: &ExtractExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#extractExpression}.
 * @param ctx the parse tree
 */
fn exit_extractExpression(&mut self, _ctx: &ExtractExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#timeQualifiers}.
 * @param ctx the parse tree
 */
fn enter_timeQualifiers(&mut self, _ctx: &TimeQualifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#timeQualifiers}.
 * @param ctx the parse tree
 */
fn exit_timeQualifiers(&mut self, _ctx: &TimeQualifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#constant}.
 * @param ctx the parse tree
 */
fn enter_constant(&mut self, _ctx: &ConstantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#constant}.
 * @param ctx the parse tree
 */
fn exit_constant(&mut self, _ctx: &ConstantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#prepareStmtParam}.
 * @param ctx the parse tree
 */
fn enter_prepareStmtParam(&mut self, _ctx: &PrepareStmtParamContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#prepareStmtParam}.
 * @param ctx the parse tree
 */
fn exit_prepareStmtParam(&mut self, _ctx: &PrepareStmtParamContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#parameterIdx}.
 * @param ctx the parse tree
 */
fn enter_parameterIdx(&mut self, _ctx: &ParameterIdxContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#parameterIdx}.
 * @param ctx the parse tree
 */
fn exit_parameterIdx(&mut self, _ctx: &ParameterIdxContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#stringLiteralSequence}.
 * @param ctx the parse tree
 */
fn enter_stringLiteralSequence(&mut self, _ctx: &StringLiteralSequenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#stringLiteralSequence}.
 * @param ctx the parse tree
 */
fn exit_stringLiteralSequence(&mut self, _ctx: &StringLiteralSequenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#charSetStringLiteral}.
 * @param ctx the parse tree
 */
fn enter_charSetStringLiteral(&mut self, _ctx: &CharSetStringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#charSetStringLiteral}.
 * @param ctx the parse tree
 */
fn exit_charSetStringLiteral(&mut self, _ctx: &CharSetStringLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dateLiteral}.
 * @param ctx the parse tree
 */
fn enter_dateLiteral(&mut self, _ctx: &DateLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dateLiteral}.
 * @param ctx the parse tree
 */
fn exit_dateLiteral(&mut self, _ctx: &DateLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#timestampLiteral}.
 * @param ctx the parse tree
 */
fn enter_timestampLiteral(&mut self, _ctx: &TimestampLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#timestampLiteral}.
 * @param ctx the parse tree
 */
fn exit_timestampLiteral(&mut self, _ctx: &TimestampLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#timestampLocalTZLiteral}.
 * @param ctx the parse tree
 */
fn enter_timestampLocalTZLiteral(&mut self, _ctx: &TimestampLocalTZLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#timestampLocalTZLiteral}.
 * @param ctx the parse tree
 */
fn exit_timestampLocalTZLiteral(&mut self, _ctx: &TimestampLocalTZLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#intervalValue}.
 * @param ctx the parse tree
 */
fn enter_intervalValue(&mut self, _ctx: &IntervalValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#intervalValue}.
 * @param ctx the parse tree
 */
fn exit_intervalValue(&mut self, _ctx: &IntervalValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#intervalLiteral}.
 * @param ctx the parse tree
 */
fn enter_intervalLiteral(&mut self, _ctx: &IntervalLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#intervalLiteral}.
 * @param ctx the parse tree
 */
fn exit_intervalLiteral(&mut self, _ctx: &IntervalLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#intervalExpression}.
 * @param ctx the parse tree
 */
fn enter_intervalExpression(&mut self, _ctx: &IntervalExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#intervalExpression}.
 * @param ctx the parse tree
 */
fn exit_intervalExpression(&mut self, _ctx: &IntervalExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#intervalQualifiers}.
 * @param ctx the parse tree
 */
fn enter_intervalQualifiers(&mut self, _ctx: &IntervalQualifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#intervalQualifiers}.
 * @param ctx the parse tree
 */
fn exit_intervalQualifiers(&mut self, _ctx: &IntervalQualifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#atomExpression}.
 * @param ctx the parse tree
 */
fn enter_atomExpression(&mut self, _ctx: &AtomExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#atomExpression}.
 * @param ctx the parse tree
 */
fn exit_atomExpression(&mut self, _ctx: &AtomExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceFieldExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceFieldExpression(&mut self, _ctx: &PrecedenceFieldExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceFieldExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceFieldExpression(&mut self, _ctx: &PrecedenceFieldExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceUnaryOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceUnaryOperator(&mut self, _ctx: &PrecedenceUnaryOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceUnaryOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceUnaryOperator(&mut self, _ctx: &PrecedenceUnaryOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceUnaryPrefixExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceUnaryPrefixExpression(&mut self, _ctx: &PrecedenceUnaryPrefixExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceUnaryPrefixExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceUnaryPrefixExpression(&mut self, _ctx: &PrecedenceUnaryPrefixExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceBitwiseXorOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceBitwiseXorOperator(&mut self, _ctx: &PrecedenceBitwiseXorOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceBitwiseXorOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceBitwiseXorOperator(&mut self, _ctx: &PrecedenceBitwiseXorOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceBitwiseXorExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceBitwiseXorExpression(&mut self, _ctx: &PrecedenceBitwiseXorExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceBitwiseXorExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceBitwiseXorExpression(&mut self, _ctx: &PrecedenceBitwiseXorExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceStarOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceStarOperator(&mut self, _ctx: &PrecedenceStarOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceStarOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceStarOperator(&mut self, _ctx: &PrecedenceStarOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceStarExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceStarExpression(&mut self, _ctx: &PrecedenceStarExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceStarExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceStarExpression(&mut self, _ctx: &PrecedenceStarExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedencePlusOperator}.
 * @param ctx the parse tree
 */
fn enter_precedencePlusOperator(&mut self, _ctx: &PrecedencePlusOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedencePlusOperator}.
 * @param ctx the parse tree
 */
fn exit_precedencePlusOperator(&mut self, _ctx: &PrecedencePlusOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedencePlusExpression}.
 * @param ctx the parse tree
 */
fn enter_precedencePlusExpression(&mut self, _ctx: &PrecedencePlusExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedencePlusExpression}.
 * @param ctx the parse tree
 */
fn exit_precedencePlusExpression(&mut self, _ctx: &PrecedencePlusExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceConcatenateOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceConcatenateOperator(&mut self, _ctx: &PrecedenceConcatenateOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceConcatenateOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceConcatenateOperator(&mut self, _ctx: &PrecedenceConcatenateOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceConcatenateExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceConcatenateExpression(&mut self, _ctx: &PrecedenceConcatenateExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceConcatenateExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceConcatenateExpression(&mut self, _ctx: &PrecedenceConcatenateExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceAmpersandOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceAmpersandOperator(&mut self, _ctx: &PrecedenceAmpersandOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceAmpersandOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceAmpersandOperator(&mut self, _ctx: &PrecedenceAmpersandOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceAmpersandExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceAmpersandExpression(&mut self, _ctx: &PrecedenceAmpersandExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceAmpersandExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceAmpersandExpression(&mut self, _ctx: &PrecedenceAmpersandExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceBitwiseOrOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceBitwiseOrOperator(&mut self, _ctx: &PrecedenceBitwiseOrOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceBitwiseOrOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceBitwiseOrOperator(&mut self, _ctx: &PrecedenceBitwiseOrOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceBitwiseOrExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceBitwiseOrExpression(&mut self, _ctx: &PrecedenceBitwiseOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceBitwiseOrExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceBitwiseOrExpression(&mut self, _ctx: &PrecedenceBitwiseOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceRegexpOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceRegexpOperator(&mut self, _ctx: &PrecedenceRegexpOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceRegexpOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceRegexpOperator(&mut self, _ctx: &PrecedenceRegexpOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceSimilarOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceSimilarOperator(&mut self, _ctx: &PrecedenceSimilarOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceSimilarOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceSimilarOperator(&mut self, _ctx: &PrecedenceSimilarOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#subQueryExpression}.
 * @param ctx the parse tree
 */
fn enter_subQueryExpression(&mut self, _ctx: &SubQueryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#subQueryExpression}.
 * @param ctx the parse tree
 */
fn exit_subQueryExpression(&mut self, _ctx: &SubQueryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceSimilarExpression(&mut self, _ctx: &PrecedenceSimilarExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceSimilarExpression(&mut self, _ctx: &PrecedenceSimilarExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpressionMain}.
 * @param ctx the parse tree
 */
fn enter_precedenceSimilarExpressionMain(&mut self, _ctx: &PrecedenceSimilarExpressionMainContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpressionMain}.
 * @param ctx the parse tree
 */
fn exit_precedenceSimilarExpressionMain(&mut self, _ctx: &PrecedenceSimilarExpressionMainContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpressionPart}.
 * @param ctx the parse tree
 */
fn enter_precedenceSimilarExpressionPart(&mut self, _ctx: &PrecedenceSimilarExpressionPartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpressionPart}.
 * @param ctx the parse tree
 */
fn exit_precedenceSimilarExpressionPart(&mut self, _ctx: &PrecedenceSimilarExpressionPartContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpressionAtom}.
 * @param ctx the parse tree
 */
fn enter_precedenceSimilarExpressionAtom(&mut self, _ctx: &PrecedenceSimilarExpressionAtomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpressionAtom}.
 * @param ctx the parse tree
 */
fn exit_precedenceSimilarExpressionAtom(&mut self, _ctx: &PrecedenceSimilarExpressionAtomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpressionQuantifierPredicate}.
 * @param ctx the parse tree
 */
fn enter_precedenceSimilarExpressionQuantifierPredicate(&mut self, _ctx: &PrecedenceSimilarExpressionQuantifierPredicateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpressionQuantifierPredicate}.
 * @param ctx the parse tree
 */
fn exit_precedenceSimilarExpressionQuantifierPredicate(&mut self, _ctx: &PrecedenceSimilarExpressionQuantifierPredicateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#quantifierType}.
 * @param ctx the parse tree
 */
fn enter_quantifierType(&mut self, _ctx: &QuantifierTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#quantifierType}.
 * @param ctx the parse tree
 */
fn exit_quantifierType(&mut self, _ctx: &QuantifierTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpressionIn}.
 * @param ctx the parse tree
 */
fn enter_precedenceSimilarExpressionIn(&mut self, _ctx: &PrecedenceSimilarExpressionInContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpressionIn}.
 * @param ctx the parse tree
 */
fn exit_precedenceSimilarExpressionIn(&mut self, _ctx: &PrecedenceSimilarExpressionInContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpressionPartNot}.
 * @param ctx the parse tree
 */
fn enter_precedenceSimilarExpressionPartNot(&mut self, _ctx: &PrecedenceSimilarExpressionPartNotContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceSimilarExpressionPartNot}.
 * @param ctx the parse tree
 */
fn exit_precedenceSimilarExpressionPartNot(&mut self, _ctx: &PrecedenceSimilarExpressionPartNotContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceDistinctOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceDistinctOperator(&mut self, _ctx: &PrecedenceDistinctOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceDistinctOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceDistinctOperator(&mut self, _ctx: &PrecedenceDistinctOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceEqualOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceEqualOperator(&mut self, _ctx: &PrecedenceEqualOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceEqualOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceEqualOperator(&mut self, _ctx: &PrecedenceEqualOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceEqualExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceEqualExpression(&mut self, _ctx: &PrecedenceEqualExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceEqualExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceEqualExpression(&mut self, _ctx: &PrecedenceEqualExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#isCondition}.
 * @param ctx the parse tree
 */
fn enter_isCondition(&mut self, _ctx: &IsConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#isCondition}.
 * @param ctx the parse tree
 */
fn exit_isCondition(&mut self, _ctx: &IsConditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceUnarySuffixExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceUnarySuffixExpression(&mut self, _ctx: &PrecedenceUnarySuffixExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceUnarySuffixExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceUnarySuffixExpression(&mut self, _ctx: &PrecedenceUnarySuffixExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceNotOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceNotOperator(&mut self, _ctx: &PrecedenceNotOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceNotOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceNotOperator(&mut self, _ctx: &PrecedenceNotOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceNotExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceNotExpression(&mut self, _ctx: &PrecedenceNotExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceNotExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceNotExpression(&mut self, _ctx: &PrecedenceNotExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceAndOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceAndOperator(&mut self, _ctx: &PrecedenceAndOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceAndOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceAndOperator(&mut self, _ctx: &PrecedenceAndOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceAndExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceAndExpression(&mut self, _ctx: &PrecedenceAndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceAndExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceAndExpression(&mut self, _ctx: &PrecedenceAndExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceOrOperator}.
 * @param ctx the parse tree
 */
fn enter_precedenceOrOperator(&mut self, _ctx: &PrecedenceOrOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceOrOperator}.
 * @param ctx the parse tree
 */
fn exit_precedenceOrOperator(&mut self, _ctx: &PrecedenceOrOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#precedenceOrExpression}.
 * @param ctx the parse tree
 */
fn enter_precedenceOrExpression(&mut self, _ctx: &PrecedenceOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#precedenceOrExpression}.
 * @param ctx the parse tree
 */
fn exit_precedenceOrExpression(&mut self, _ctx: &PrecedenceOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#booleanValue}.
 * @param ctx the parse tree
 */
fn enter_booleanValue(&mut self, _ctx: &BooleanValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#booleanValue}.
 * @param ctx the parse tree
 */
fn exit_booleanValue(&mut self, _ctx: &BooleanValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#booleanValueTok}.
 * @param ctx the parse tree
 */
fn enter_booleanValueTok(&mut self, _ctx: &BooleanValueTokContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#booleanValueTok}.
 * @param ctx the parse tree
 */
fn exit_booleanValueTok(&mut self, _ctx: &BooleanValueTokContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#tableOrPartition}.
 * @param ctx the parse tree
 */
fn enter_tableOrPartition(&mut self, _ctx: &TableOrPartitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#tableOrPartition}.
 * @param ctx the parse tree
 */
fn exit_tableOrPartition(&mut self, _ctx: &TableOrPartitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partitionSpec}.
 * @param ctx the parse tree
 */
fn enter_partitionSpec(&mut self, _ctx: &PartitionSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partitionSpec}.
 * @param ctx the parse tree
 */
fn exit_partitionSpec(&mut self, _ctx: &PartitionSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partitionVal}.
 * @param ctx the parse tree
 */
fn enter_partitionVal(&mut self, _ctx: &PartitionValContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partitionVal}.
 * @param ctx the parse tree
 */
fn exit_partitionVal(&mut self, _ctx: &PartitionValContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partitionSelectorSpec}.
 * @param ctx the parse tree
 */
fn enter_partitionSelectorSpec(&mut self, _ctx: &PartitionSelectorSpecContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partitionSelectorSpec}.
 * @param ctx the parse tree
 */
fn exit_partitionSelectorSpec(&mut self, _ctx: &PartitionSelectorSpecContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partitionSelectorVal}.
 * @param ctx the parse tree
 */
fn enter_partitionSelectorVal(&mut self, _ctx: &PartitionSelectorValContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partitionSelectorVal}.
 * @param ctx the parse tree
 */
fn exit_partitionSelectorVal(&mut self, _ctx: &PartitionSelectorValContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#partitionSelectorOperator}.
 * @param ctx the parse tree
 */
fn enter_partitionSelectorOperator(&mut self, _ctx: &PartitionSelectorOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#partitionSelectorOperator}.
 * @param ctx the parse tree
 */
fn exit_partitionSelectorOperator(&mut self, _ctx: &PartitionSelectorOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#subQuerySelectorOperator}.
 * @param ctx the parse tree
 */
fn enter_subQuerySelectorOperator(&mut self, _ctx: &SubQuerySelectorOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#subQuerySelectorOperator}.
 * @param ctx the parse tree
 */
fn exit_subQuerySelectorOperator(&mut self, _ctx: &SubQuerySelectorOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#sysFuncNames}.
 * @param ctx the parse tree
 */
fn enter_sysFuncNames(&mut self, _ctx: &SysFuncNamesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#sysFuncNames}.
 * @param ctx the parse tree
 */
fn exit_sysFuncNames(&mut self, _ctx: &SysFuncNamesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#descFuncNames}.
 * @param ctx the parse tree
 */
fn enter_descFuncNames(&mut self, _ctx: &DescFuncNamesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#descFuncNames}.
 * @param ctx the parse tree
 */
fn exit_descFuncNames(&mut self, _ctx: &DescFuncNamesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#id_}.
 * @param ctx the parse tree
 */
fn enter_id_(&mut self, _ctx: &Id_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#id_}.
 * @param ctx the parse tree
 */
fn exit_id_(&mut self, _ctx: &Id_Context<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#functionIdentifier}.
 * @param ctx the parse tree
 */
fn enter_functionIdentifier(&mut self, _ctx: &FunctionIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#functionIdentifier}.
 * @param ctx the parse tree
 */
fn exit_functionIdentifier(&mut self, _ctx: &FunctionIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#principalIdentifier}.
 * @param ctx the parse tree
 */
fn enter_principalIdentifier(&mut self, _ctx: &PrincipalIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#principalIdentifier}.
 * @param ctx the parse tree
 */
fn exit_principalIdentifier(&mut self, _ctx: &PrincipalIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#nonReserved}.
 * @param ctx the parse tree
 */
fn enter_nonReserved(&mut self, _ctx: &NonReservedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#nonReserved}.
 * @param ctx the parse tree
 */
fn exit_nonReserved(&mut self, _ctx: &NonReservedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#sql11ReservedKeywordsUsedAsFunctionName}.
 * @param ctx the parse tree
 */
fn enter_sql11ReservedKeywordsUsedAsFunctionName(&mut self, _ctx: &Sql11ReservedKeywordsUsedAsFunctionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#sql11ReservedKeywordsUsedAsFunctionName}.
 * @param ctx the parse tree
 */
fn exit_sql11ReservedKeywordsUsedAsFunctionName(&mut self, _ctx: &Sql11ReservedKeywordsUsedAsFunctionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#hint}.
 * @param ctx the parse tree
 */
fn enter_hint(&mut self, _ctx: &HintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#hint}.
 * @param ctx the parse tree
 */
fn exit_hint(&mut self, _ctx: &HintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#hintList}.
 * @param ctx the parse tree
 */
fn enter_hintList(&mut self, _ctx: &HintListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#hintList}.
 * @param ctx the parse tree
 */
fn exit_hintList(&mut self, _ctx: &HintListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#hintItem}.
 * @param ctx the parse tree
 */
fn enter_hintItem(&mut self, _ctx: &HintItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#hintItem}.
 * @param ctx the parse tree
 */
fn exit_hintItem(&mut self, _ctx: &HintItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#hintName}.
 * @param ctx the parse tree
 */
fn enter_hintName(&mut self, _ctx: &HintNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#hintName}.
 * @param ctx the parse tree
 */
fn exit_hintName(&mut self, _ctx: &HintNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#hintArgs}.
 * @param ctx the parse tree
 */
fn enter_hintArgs(&mut self, _ctx: &HintArgsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#hintArgs}.
 * @param ctx the parse tree
 */
fn exit_hintArgs(&mut self, _ctx: &HintArgsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#hintArgName}.
 * @param ctx the parse tree
 */
fn enter_hintArgName(&mut self, _ctx: &HintArgNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#hintArgName}.
 * @param ctx the parse tree
 */
fn exit_hintArgName(&mut self, _ctx: &HintArgNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#prepareStatement}.
 * @param ctx the parse tree
 */
fn enter_prepareStatement(&mut self, _ctx: &PrepareStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#prepareStatement}.
 * @param ctx the parse tree
 */
fn exit_prepareStatement(&mut self, _ctx: &PrepareStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#executeStatement}.
 * @param ctx the parse tree
 */
fn enter_executeStatement(&mut self, _ctx: &ExecuteStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#executeStatement}.
 * @param ctx the parse tree
 */
fn exit_executeStatement(&mut self, _ctx: &ExecuteStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#executeParamList}.
 * @param ctx the parse tree
 */
fn enter_executeParamList(&mut self, _ctx: &ExecuteParamListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#executeParamList}.
 * @param ctx the parse tree
 */
fn exit_executeParamList(&mut self, _ctx: &ExecuteParamListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#resourcePlanDdlStatements}.
 * @param ctx the parse tree
 */
fn enter_resourcePlanDdlStatements(&mut self, _ctx: &ResourcePlanDdlStatementsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#resourcePlanDdlStatements}.
 * @param ctx the parse tree
 */
fn exit_resourcePlanDdlStatements(&mut self, _ctx: &ResourcePlanDdlStatementsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#rpAssign}.
 * @param ctx the parse tree
 */
fn enter_rpAssign(&mut self, _ctx: &RpAssignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#rpAssign}.
 * @param ctx the parse tree
 */
fn exit_rpAssign(&mut self, _ctx: &RpAssignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#rpAssignList}.
 * @param ctx the parse tree
 */
fn enter_rpAssignList(&mut self, _ctx: &RpAssignListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#rpAssignList}.
 * @param ctx the parse tree
 */
fn exit_rpAssignList(&mut self, _ctx: &RpAssignListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#rpUnassign}.
 * @param ctx the parse tree
 */
fn enter_rpUnassign(&mut self, _ctx: &RpUnassignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#rpUnassign}.
 * @param ctx the parse tree
 */
fn exit_rpUnassign(&mut self, _ctx: &RpUnassignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#rpUnassignList}.
 * @param ctx the parse tree
 */
fn enter_rpUnassignList(&mut self, _ctx: &RpUnassignListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#rpUnassignList}.
 * @param ctx the parse tree
 */
fn exit_rpUnassignList(&mut self, _ctx: &RpUnassignListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createResourcePlanStatement}.
 * @param ctx the parse tree
 */
fn enter_createResourcePlanStatement(&mut self, _ctx: &CreateResourcePlanStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createResourcePlanStatement}.
 * @param ctx the parse tree
 */
fn exit_createResourcePlanStatement(&mut self, _ctx: &CreateResourcePlanStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#withReplace}.
 * @param ctx the parse tree
 */
fn enter_withReplace(&mut self, _ctx: &WithReplaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#withReplace}.
 * @param ctx the parse tree
 */
fn exit_withReplace(&mut self, _ctx: &WithReplaceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#activate}.
 * @param ctx the parse tree
 */
fn enter_activate(&mut self, _ctx: &ActivateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#activate}.
 * @param ctx the parse tree
 */
fn exit_activate(&mut self, _ctx: &ActivateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#enable}.
 * @param ctx the parse tree
 */
fn enter_enable(&mut self, _ctx: &EnableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#enable}.
 * @param ctx the parse tree
 */
fn exit_enable(&mut self, _ctx: &EnableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#disable}.
 * @param ctx the parse tree
 */
fn enter_disable(&mut self, _ctx: &DisableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#disable}.
 * @param ctx the parse tree
 */
fn exit_disable(&mut self, _ctx: &DisableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#unmanaged}.
 * @param ctx the parse tree
 */
fn enter_unmanaged(&mut self, _ctx: &UnmanagedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#unmanaged}.
 * @param ctx the parse tree
 */
fn exit_unmanaged(&mut self, _ctx: &UnmanagedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterResourcePlanStatement}.
 * @param ctx the parse tree
 */
fn enter_alterResourcePlanStatement(&mut self, _ctx: &AlterResourcePlanStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterResourcePlanStatement}.
 * @param ctx the parse tree
 */
fn exit_alterResourcePlanStatement(&mut self, _ctx: &AlterResourcePlanStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#globalWmStatement}.
 * @param ctx the parse tree
 */
fn enter_globalWmStatement(&mut self, _ctx: &GlobalWmStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#globalWmStatement}.
 * @param ctx the parse tree
 */
fn exit_globalWmStatement(&mut self, _ctx: &GlobalWmStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#replaceResourcePlanStatement}.
 * @param ctx the parse tree
 */
fn enter_replaceResourcePlanStatement(&mut self, _ctx: &ReplaceResourcePlanStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#replaceResourcePlanStatement}.
 * @param ctx the parse tree
 */
fn exit_replaceResourcePlanStatement(&mut self, _ctx: &ReplaceResourcePlanStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropResourcePlanStatement}.
 * @param ctx the parse tree
 */
fn enter_dropResourcePlanStatement(&mut self, _ctx: &DropResourcePlanStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropResourcePlanStatement}.
 * @param ctx the parse tree
 */
fn exit_dropResourcePlanStatement(&mut self, _ctx: &DropResourcePlanStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#poolPath}.
 * @param ctx the parse tree
 */
fn enter_poolPath(&mut self, _ctx: &PoolPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#poolPath}.
 * @param ctx the parse tree
 */
fn exit_poolPath(&mut self, _ctx: &PoolPathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#triggerExpression}.
 * @param ctx the parse tree
 */
fn enter_triggerExpression(&mut self, _ctx: &TriggerExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#triggerExpression}.
 * @param ctx the parse tree
 */
fn exit_triggerExpression(&mut self, _ctx: &TriggerExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#triggerExpressionStandalone}.
 * @param ctx the parse tree
 */
fn enter_triggerExpressionStandalone(&mut self, _ctx: &TriggerExpressionStandaloneContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#triggerExpressionStandalone}.
 * @param ctx the parse tree
 */
fn exit_triggerExpressionStandalone(&mut self, _ctx: &TriggerExpressionStandaloneContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#triggerOrExpression}.
 * @param ctx the parse tree
 */
fn enter_triggerOrExpression(&mut self, _ctx: &TriggerOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#triggerOrExpression}.
 * @param ctx the parse tree
 */
fn exit_triggerOrExpression(&mut self, _ctx: &TriggerOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#triggerAndExpression}.
 * @param ctx the parse tree
 */
fn enter_triggerAndExpression(&mut self, _ctx: &TriggerAndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#triggerAndExpression}.
 * @param ctx the parse tree
 */
fn exit_triggerAndExpression(&mut self, _ctx: &TriggerAndExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#triggerAtomExpression}.
 * @param ctx the parse tree
 */
fn enter_triggerAtomExpression(&mut self, _ctx: &TriggerAtomExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#triggerAtomExpression}.
 * @param ctx the parse tree
 */
fn exit_triggerAtomExpression(&mut self, _ctx: &TriggerAtomExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#triggerLiteral}.
 * @param ctx the parse tree
 */
fn enter_triggerLiteral(&mut self, _ctx: &TriggerLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#triggerLiteral}.
 * @param ctx the parse tree
 */
fn exit_triggerLiteral(&mut self, _ctx: &TriggerLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#comparisionOperator}.
 * @param ctx the parse tree
 */
fn enter_comparisionOperator(&mut self, _ctx: &ComparisionOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#comparisionOperator}.
 * @param ctx the parse tree
 */
fn exit_comparisionOperator(&mut self, _ctx: &ComparisionOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#triggerActionExpression}.
 * @param ctx the parse tree
 */
fn enter_triggerActionExpression(&mut self, _ctx: &TriggerActionExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#triggerActionExpression}.
 * @param ctx the parse tree
 */
fn exit_triggerActionExpression(&mut self, _ctx: &TriggerActionExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#triggerActionExpressionStandalone}.
 * @param ctx the parse tree
 */
fn enter_triggerActionExpressionStandalone(&mut self, _ctx: &TriggerActionExpressionStandaloneContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#triggerActionExpressionStandalone}.
 * @param ctx the parse tree
 */
fn exit_triggerActionExpressionStandalone(&mut self, _ctx: &TriggerActionExpressionStandaloneContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createTriggerStatement}.
 * @param ctx the parse tree
 */
fn enter_createTriggerStatement(&mut self, _ctx: &CreateTriggerStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createTriggerStatement}.
 * @param ctx the parse tree
 */
fn exit_createTriggerStatement(&mut self, _ctx: &CreateTriggerStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterTriggerStatement}.
 * @param ctx the parse tree
 */
fn enter_alterTriggerStatement(&mut self, _ctx: &AlterTriggerStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterTriggerStatement}.
 * @param ctx the parse tree
 */
fn exit_alterTriggerStatement(&mut self, _ctx: &AlterTriggerStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropTriggerStatement}.
 * @param ctx the parse tree
 */
fn enter_dropTriggerStatement(&mut self, _ctx: &DropTriggerStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropTriggerStatement}.
 * @param ctx the parse tree
 */
fn exit_dropTriggerStatement(&mut self, _ctx: &DropTriggerStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#poolAssign}.
 * @param ctx the parse tree
 */
fn enter_poolAssign(&mut self, _ctx: &PoolAssignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#poolAssign}.
 * @param ctx the parse tree
 */
fn exit_poolAssign(&mut self, _ctx: &PoolAssignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#poolAssignList}.
 * @param ctx the parse tree
 */
fn enter_poolAssignList(&mut self, _ctx: &PoolAssignListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#poolAssignList}.
 * @param ctx the parse tree
 */
fn exit_poolAssignList(&mut self, _ctx: &PoolAssignListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createPoolStatement}.
 * @param ctx the parse tree
 */
fn enter_createPoolStatement(&mut self, _ctx: &CreatePoolStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createPoolStatement}.
 * @param ctx the parse tree
 */
fn exit_createPoolStatement(&mut self, _ctx: &CreatePoolStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterPoolStatement}.
 * @param ctx the parse tree
 */
fn enter_alterPoolStatement(&mut self, _ctx: &AlterPoolStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterPoolStatement}.
 * @param ctx the parse tree
 */
fn exit_alterPoolStatement(&mut self, _ctx: &AlterPoolStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropPoolStatement}.
 * @param ctx the parse tree
 */
fn enter_dropPoolStatement(&mut self, _ctx: &DropPoolStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropPoolStatement}.
 * @param ctx the parse tree
 */
fn exit_dropPoolStatement(&mut self, _ctx: &DropPoolStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#createMappingStatement}.
 * @param ctx the parse tree
 */
fn enter_createMappingStatement(&mut self, _ctx: &CreateMappingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#createMappingStatement}.
 * @param ctx the parse tree
 */
fn exit_createMappingStatement(&mut self, _ctx: &CreateMappingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#alterMappingStatement}.
 * @param ctx the parse tree
 */
fn enter_alterMappingStatement(&mut self, _ctx: &AlterMappingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#alterMappingStatement}.
 * @param ctx the parse tree
 */
fn exit_alterMappingStatement(&mut self, _ctx: &AlterMappingStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link HiveSqlParser#dropMappingStatement}.
 * @param ctx the parse tree
 */
fn enter_dropMappingStatement(&mut self, _ctx: &DropMappingStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link HiveSqlParser#dropMappingStatement}.
 * @param ctx the parse tree
 */
fn exit_dropMappingStatement(&mut self, _ctx: &DropMappingStatementContext<'input>) { }

}

crate::coerce_from!{ 'input : HiveSqlParserListener<'input> }


