#![no_main]
use libfuzzer_sys::fuzz_target;

use sana::Sana;

fuzz_target!(|input: String| {
    let lex = SqlToken::lexer(&input);
    for token in lex {
        if token.value == SqlToken::Error {
            break;
        }
    }
});


#[derive(Sana, Debug, Clone, Copy, PartialEq)]
pub enum SqlToken {
    #[regex(r"/\*", priority = 616)]
    CommentStart,
    #[token("'")]
    StringStart,
    #[regex(r"[bB]'")]
    BinaryStart,
    #[regex(r"[xX]'")]
    HexStart,
    #[regex(r"[nN]'")]
    NatStart,
    #[regex(r"[eE]'")]
    ExtStart,
    #[regex(r"[uU]&'")]
    UStringStart,
    #[regex(r"\$([A-Za-z\u{100}-\u{10ffff}_][A-Za-z\u{100}-\u{10ffff}_0-9]*)?\$")]
    DollarStart,
    #[regex("\"")]
    IdentStart,
    #[regex("[uU]&\"")]
    UIdentStart,

    #[regex("(?i)uescape", priority = 1)]
    Uescape,

    #[regex(r"[A-Za-z\u{100}-\u{10ffff}_][A-Za-z\u{100}-\u{10ffff}_0-9]*")]
    Ident,
    UIdent,

    #[regex(r"(\d*\.\d+)|(\d+\.\d*)")]
    Decimal,
    #[regex(r"(\d+|((\d*\.\d+)|(\d+\.\d*)))[Ee][-+]?\d+", priority = 1)]
    Real,
    String,
    Binary,
    Hex,
    NatString,
    ExtString,
    UString,
    #[regex(
        r"[~!@#^&|`?+\-/*%<>=]+"
        & !".*--.*"
        & !".*/\\*.*"
        & !r"[+\-/*<>=]+[+-]+"
    )]
    Op,

    #[regex(r"\d+", priority = 1)] Integer,
    #[regex(r"\$\d+")] Param,

    #[token(",")] Comma,
    #[token(";")] Semicolon,
    #[token(":")] Colon,
    #[token(".")] Dot,
    #[token("(")] LParren,
    #[token(")")] RParren,
    #[token("[")] LBracket,
    #[token("]")] RBracket,

    #[token("+", priority = 1)] Plus,
    #[token("-", priority = 1)] Minus,
    #[token("*", priority = 1)] Star,
    #[token("/", priority = 1)] Slash,
    #[token("%", priority = 1)] Percent,
    #[token("^", priority = 1)] Hat,
    #[token("<", priority = 1)] Less,
    #[token(">", priority = 1)] Greater,
    #[token("=", priority = 1)] Equals,

    #[token("::", priority = 1)] TypeCast,
    #[token("..", priority = 1)] DotDot,
    #[token(":=", priority = 1)] ColonEquals,
    #[token("=>", priority = 1)] EqualsGreater,
    #[token("<=", priority = 1)] LessEquals,
    #[token(">=", priority = 1)] GreaterEquals,
    #[regex("<>|!=", priority = 1)] NotEquals,

    // Special kludges we might need...
    NotLa, NullsLa, WithLa,

    #[regex(r"[ \t\f\r\n]+|--[^\r\n]*")]
    Skip,
    #[error]
    Error,

    // NOW THE KEYWORDS!
    #[regex(r"(?i)abort", priority = 1)] KwAbort,
    #[regex(r"(?i)absolute", priority = 1)] KwAbsolute,
    #[regex(r"(?i)access", priority = 1)] KwAccess,
    #[regex(r"(?i)action", priority = 1)] KwAction,
    #[regex(r"(?i)add", priority = 1)] KwAdd,
    #[regex(r"(?i)admin", priority = 1)] KwAdmin,
    #[regex(r"(?i)after", priority = 1)] KwAfter,
    #[regex(r"(?i)aggregate", priority = 1)] KwAggregate,
    #[regex(r"(?i)all", priority = 1)] KwAll,
    #[regex(r"(?i)also", priority = 1)] KwAlso,
    #[regex(r"(?i)alter", priority = 1)] KwAlter,
    #[regex(r"(?i)always", priority = 1)] KwAlways,
    #[regex(r"(?i)analyse", priority = 1)] KwAnalyse,
    #[regex(r"(?i)analyze", priority = 1)] KwAnalyze,
    #[regex(r"(?i)and", priority = 1)] KwAnd,
    #[regex(r"(?i)any", priority = 1)] KwAny,
    #[regex(r"(?i)array", priority = 1)] KwArray,
    #[regex(r"(?i)as", priority = 1)] KwAs,
    #[regex(r"(?i)asc", priority = 1)] KwAsc,
    #[regex(r"(?i)assertion", priority = 1)] KwAssertion,
    #[regex(r"(?i)assignment", priority = 1)] KwAssignment,
    #[regex(r"(?i)asymmetric", priority = 1)] KwAsymmetric,
    #[regex(r"(?i)at", priority = 1)] KwAt,
    #[regex(r"(?i)attach", priority = 1)] KwAttach,
    #[regex(r"(?i)attribute", priority = 1)] KwAttribute,
    #[regex(r"(?i)authorization", priority = 1)] KwAuthorization,
    #[regex(r"(?i)backward", priority = 1)] KwBackward,
    #[regex(r"(?i)before", priority = 1)] KwBefore,
    #[regex(r"(?i)begin", priority = 1)] KwBegin,
    #[regex(r"(?i)between", priority = 1)] KwBetween,
    #[regex(r"(?i)bigint", priority = 1)] KwBigint,
    #[regex(r"(?i)binary", priority = 1)] KwBinary,
    #[regex(r"(?i)bit", priority = 1)] KwBit,
    #[regex(r"(?i)boolean", priority = 1)] KwBoolean,
    #[regex(r"(?i)both", priority = 1)] KwBoth,
    #[regex(r"(?i)by", priority = 1)] KwBy,
    #[regex(r"(?i)cache", priority = 1)] KwCache,
    #[regex(r"(?i)call", priority = 1)] KwCall,
    #[regex(r"(?i)called", priority = 1)] KwCalled,
    #[regex(r"(?i)cascade", priority = 1)] KwCascade,
    #[regex(r"(?i)cascaded", priority = 1)] KwCascaded,
    #[regex(r"(?i)case", priority = 1)] KwCase,
    #[regex(r"(?i)cast", priority = 1)] KwCast,
    #[regex(r"(?i)catalog", priority = 1)] KwCatalog,
    #[regex(r"(?i)chain", priority = 1)] KwChain,
    #[regex(r"(?i)char", priority = 1)] KwChar,
    #[regex(r"(?i)character", priority = 1)] KwCharacter,
    #[regex(r"(?i)characteristics", priority = 1)] KwCharacteristics,
    #[regex(r"(?i)check", priority = 1)] KwCheck,
    #[regex(r"(?i)checkpoint", priority = 1)] KwCheckpoint,
    #[regex(r"(?i)class", priority = 1)] KwClass,
    #[regex(r"(?i)close", priority = 1)] KwClose,
    #[regex(r"(?i)cluster", priority = 1)] KwCluster,
    #[regex(r"(?i)coalesce", priority = 1)] KwCoalesce,
    #[regex(r"(?i)collate", priority = 1)] KwCollate,
    #[regex(r"(?i)collation", priority = 1)] KwCollation,
    #[regex(r"(?i)column", priority = 1)] KwColumn,
    #[regex(r"(?i)columns", priority = 1)] KwColumns,
    #[regex(r"(?i)comment", priority = 1)] KwComment,
    #[regex(r"(?i)comments", priority = 1)] KwComments,
    #[regex(r"(?i)commit", priority = 1)] KwCommit,
    #[regex(r"(?i)committed", priority = 1)] KwCommitted,
    #[regex(r"(?i)concurrently", priority = 1)] KwConcurrently,
    #[regex(r"(?i)configuration", priority = 1)] KwConfiguration,
    #[regex(r"(?i)conflict", priority = 1)] KwConflict,
    #[regex(r"(?i)connection", priority = 1)] KwConnection,
    #[regex(r"(?i)constraint", priority = 1)] KwConstraint,
    #[regex(r"(?i)constraints", priority = 1)] KwConstraints,
    #[regex(r"(?i)content", priority = 1)] KwContent,
    #[regex(r"(?i)continue", priority = 1)] KwContinue,
    #[regex(r"(?i)conversion", priority = 1)] KwConversion,
    #[regex(r"(?i)copy", priority = 1)] KwCopy,
    #[regex(r"(?i)cost", priority = 1)] KwCost,
    #[regex(r"(?i)create", priority = 1)] KwCreate,
    #[regex(r"(?i)cross", priority = 1)] KwCross,
    #[regex(r"(?i)csv", priority = 1)] KwCsv,
    #[regex(r"(?i)cube", priority = 1)] KwCube,
    #[regex(r"(?i)current", priority = 1)] KwCurrent,
    #[regex(r"(?i)current_catalog", priority = 1)] KwCurrentCatalog,
    #[regex(r"(?i)current_date", priority = 1)] KwCurrentDate,
    #[regex(r"(?i)current_role", priority = 1)] KwCurrentRole,
    #[regex(r"(?i)current_schema", priority = 1)] KwCurrentSchema,
    #[regex(r"(?i)current_time", priority = 1)] KwCurrentTime,
    #[regex(r"(?i)current_timestamp", priority = 1)] KwCurrentTimestamp,
    #[regex(r"(?i)current_user", priority = 1)] KwCurrentUser,
    #[regex(r"(?i)cursor", priority = 1)] KwCursor,
    #[regex(r"(?i)cycle", priority = 1)] KwCycle,
    #[regex(r"(?i)data", priority = 1)] KwData,
    #[regex(r"(?i)database", priority = 1)] KwDatabase,
    #[regex(r"(?i)day", priority = 1)] KwDay,
    #[regex(r"(?i)deallocate", priority = 1)] KwDeallocate,
    #[regex(r"(?i)dec", priority = 1)] KwDec,
    #[regex(r"(?i)decimal", priority = 1)] KwDecimal,
    #[regex(r"(?i)declare", priority = 1)] KwDeclare,
    #[regex(r"(?i)default", priority = 1)] KwDefault,
    #[regex(r"(?i)defaults", priority = 1)] KwDefaults,
    #[regex(r"(?i)deferrable", priority = 1)] KwDeferrable,
    #[regex(r"(?i)deferred", priority = 1)] KwDeferred,
    #[regex(r"(?i)definer", priority = 1)] KwDefiner,
    #[regex(r"(?i)delete", priority = 1)] KwDelete,
    #[regex(r"(?i)delimiter", priority = 1)] KwDelimiter,
    #[regex(r"(?i)delimiters", priority = 1)] KwDelimiters,
    #[regex(r"(?i)depends", priority = 1)] KwDepends,
    #[regex(r"(?i)desc", priority = 1)] KwDesc,
    #[regex(r"(?i)detach", priority = 1)] KwDetach,
    #[regex(r"(?i)dictionary", priority = 1)] KwDictionary,
    #[regex(r"(?i)disable", priority = 1)] KwDisable,
    #[regex(r"(?i)discard", priority = 1)] KwDiscard,
    #[regex(r"(?i)distinct", priority = 1)] KwDistinct,
    #[regex(r"(?i)do", priority = 1)] KwDo,
    #[regex(r"(?i)document", priority = 1)] KwDocument,
    #[regex(r"(?i)domain", priority = 1)] KwDomain,
    #[regex(r"(?i)double", priority = 1)] KwDouble,
    #[regex(r"(?i)drop", priority = 1)] KwDrop,
    #[regex(r"(?i)each", priority = 1)] KwEach,
    #[regex(r"(?i)else", priority = 1)] KwElse,
    #[regex(r"(?i)enable", priority = 1)] KwEnable,
    #[regex(r"(?i)encoding", priority = 1)] KwEncoding,
    #[regex(r"(?i)encrypted", priority = 1)] KwEncrypted,
    #[regex(r"(?i)end", priority = 1)] KwEnd,
    #[regex(r"(?i)enum", priority = 1)] KwEnum,
    #[regex(r"(?i)escape", priority = 1)] KwEscape,
    #[regex(r"(?i)event", priority = 1)] KwEvent,
    #[regex(r"(?i)except", priority = 1)] KwExcept,
    #[regex(r"(?i)exclude", priority = 1)] KwExclude,
    #[regex(r"(?i)excluding", priority = 1)] KwExcluding,
    #[regex(r"(?i)exclusive", priority = 1)] KwExclusive,
    #[regex(r"(?i)execute", priority = 1)] KwExecute,
    #[regex(r"(?i)exists", priority = 1)] KwExists,
    #[regex(r"(?i)explain", priority = 1)] KwExplain,
    #[regex(r"(?i)extension", priority = 1)] KwExtension,
    #[regex(r"(?i)external", priority = 1)] KwExternal,
    #[regex(r"(?i)extract", priority = 1)] KwExtract,
    #[regex(r"(?i)false", priority = 1)] KwFalse,
    #[regex(r"(?i)family", priority = 1)] KwFamily,
    #[regex(r"(?i)fetch", priority = 1)] KwFetch,
    #[regex(r"(?i)filter", priority = 1)] KwFilter,
    #[regex(r"(?i)first", priority = 1)] KwFirst,
    #[regex(r"(?i)float", priority = 1)] KwFloat,
    #[regex(r"(?i)following", priority = 1)] KwFollowing,
    #[regex(r"(?i)for", priority = 1)] KwFor,
    #[regex(r"(?i)force", priority = 1)] KwForce,
    #[regex(r"(?i)foreign", priority = 1)] KwForeign,
    #[regex(r"(?i)forward", priority = 1)] KwForward,
    #[regex(r"(?i)freeze", priority = 1)] KwFreeze,
    #[regex(r"(?i)from", priority = 1)] KwFrom,
    #[regex(r"(?i)full", priority = 1)] KwFull,
    #[regex(r"(?i)function", priority = 1)] KwFunction,
    #[regex(r"(?i)functions", priority = 1)] KwFunctions,
    #[regex(r"(?i)generated", priority = 1)] KwGenerated,
    #[regex(r"(?i)global", priority = 1)] KwGlobal,
    #[regex(r"(?i)grant", priority = 1)] KwGrant,
    #[regex(r"(?i)granted", priority = 1)] KwGranted,
    #[regex(r"(?i)greatest", priority = 1)] KwGreatest,
    #[regex(r"(?i)group", priority = 1)] KwGroup,
    #[regex(r"(?i)grouping", priority = 1)] KwGrouping,
    #[regex(r"(?i)groups", priority = 1)] KwGroups,
    #[regex(r"(?i)handler", priority = 1)] KwHandler,
    #[regex(r"(?i)having", priority = 1)] KwHaving,
    #[regex(r"(?i)header", priority = 1)] KwHeader,
    #[regex(r"(?i)hold", priority = 1)] KwHold,
    #[regex(r"(?i)hour", priority = 1)] KwHour,
    #[regex(r"(?i)identity", priority = 1)] KwIdentity,
    #[regex(r"(?i)if", priority = 1)] KwIf,
    #[regex(r"(?i)ilike", priority = 1)] KwIlike,
    #[regex(r"(?i)immediate", priority = 1)] KwImmediate,
    #[regex(r"(?i)immutable", priority = 1)] KwImmutable,
    #[regex(r"(?i)implicit", priority = 1)] KwImplicit,
    #[regex(r"(?i)import", priority = 1)] KwImport,
    #[regex(r"(?i)in", priority = 1)] KwIn,
    #[regex(r"(?i)include", priority = 1)] KwInclude,
    #[regex(r"(?i)including", priority = 1)] KwIncluding,
    #[regex(r"(?i)increment", priority = 1)] KwIncrement,
    #[regex(r"(?i)index", priority = 1)] KwIndex,
    #[regex(r"(?i)indexes", priority = 1)] KwIndexes,
    #[regex(r"(?i)inherit", priority = 1)] KwInherit,
    #[regex(r"(?i)inherits", priority = 1)] KwInherits,
    #[regex(r"(?i)initially", priority = 1)] KwInitially,
    #[regex(r"(?i)inline", priority = 1)] KwInline,
    #[regex(r"(?i)inner", priority = 1)] KwInner,
    #[regex(r"(?i)inout", priority = 1)] KwInout,
    #[regex(r"(?i)input", priority = 1)] KwInput,
    #[regex(r"(?i)insensitive", priority = 1)] KwInsensitive,
    #[regex(r"(?i)insert", priority = 1)] KwInsert,
    #[regex(r"(?i)instead", priority = 1)] KwInstead,
    #[regex(r"(?i)int", priority = 1)] KwInt,
    #[regex(r"(?i)integer", priority = 1)] KwInteger,
    #[regex(r"(?i)intersect", priority = 1)] KwIntersect,
    #[regex(r"(?i)interval", priority = 1)] KwInterval,
    #[regex(r"(?i)into", priority = 1)] KwInto,
    #[regex(r"(?i)invoker", priority = 1)] KwInvoker,
    #[regex(r"(?i)is", priority = 1)] KwIs,
    #[regex(r"(?i)isnull", priority = 1)] KwIsnull,
    #[regex(r"(?i)isolation", priority = 1)] KwIsolation,
    #[regex(r"(?i)join", priority = 1)] KwJoin,
    #[regex(r"(?i)key", priority = 1)] KwKey,
    #[regex(r"(?i)label", priority = 1)] KwLabel,
    #[regex(r"(?i)language", priority = 1)] KwLanguage,
    #[regex(r"(?i)large", priority = 1)] KwLarge,
    #[regex(r"(?i)last", priority = 1)] KwLast,
    #[regex(r"(?i)lateral", priority = 1)] KwLateral,
    #[regex(r"(?i)leading", priority = 1)] KwLeading,
    #[regex(r"(?i)leakproof", priority = 1)] KwLeakproof,
    #[regex(r"(?i)least", priority = 1)] KwLeast,
    #[regex(r"(?i)left", priority = 1)] KwLeft,
    #[regex(r"(?i)level", priority = 1)] KwLevel,
    #[regex(r"(?i)like", priority = 1)] KwLike,
    #[regex(r"(?i)limit", priority = 1)] KwLimit,
    #[regex(r"(?i)listen", priority = 1)] KwListen,
    #[regex(r"(?i)load", priority = 1)] KwLoad,
    #[regex(r"(?i)local", priority = 1)] KwLocal,
    #[regex(r"(?i)localtime", priority = 1)] KwLocaltime,
    #[regex(r"(?i)localtimestamp", priority = 1)] KwLocaltimestamp,
    #[regex(r"(?i)location", priority = 1)] KwLocation,
    #[regex(r"(?i)lock", priority = 1)] KwLock,
    #[regex(r"(?i)locked", priority = 1)] KwLocked,
    #[regex(r"(?i)logged", priority = 1)] KwLogged,
    #[regex(r"(?i)mapping", priority = 1)] KwMapping,
    #[regex(r"(?i)match", priority = 1)] KwMatch,
    #[regex(r"(?i)materialized", priority = 1)] KwMaterialized,
    #[regex(r"(?i)maxvalue", priority = 1)] KwMaxvalue,
    #[regex(r"(?i)method", priority = 1)] KwMethod,
    #[regex(r"(?i)minute", priority = 1)] KwMinute,
    #[regex(r"(?i)minvalue", priority = 1)] KwMinvalue,
    #[regex(r"(?i)mode", priority = 1)] KwMode,
    #[regex(r"(?i)month", priority = 1)] KwMonth,
    #[regex(r"(?i)move", priority = 1)] KwMove,
    #[regex(r"(?i)name", priority = 1)] KwName,
    #[regex(r"(?i)names", priority = 1)] KwNames,
    #[regex(r"(?i)national", priority = 1)] KwNational,
    #[regex(r"(?i)natural", priority = 1)] KwNatural,
    #[regex(r"(?i)nchar", priority = 1)] KwNchar,
    #[regex(r"(?i)new", priority = 1)] KwNew,
    #[regex(r"(?i)next", priority = 1)] KwNext,
    #[regex(r"(?i)no", priority = 1)] KwNo,
    #[regex(r"(?i)none", priority = 1)] KwNone,
    #[regex(r"(?i)not", priority = 1)] KwNot,
    #[regex(r"(?i)nothing", priority = 1)] KwNothing,
    #[regex(r"(?i)notify", priority = 1)] KwNotify,
    #[regex(r"(?i)notnull", priority = 1)] KwNotnull,
    #[regex(r"(?i)nowait", priority = 1)] KwNowait,
    #[regex(r"(?i)null", priority = 1)] KwNull,
    #[regex(r"(?i)nullif", priority = 1)] KwNullif,
    #[regex(r"(?i)nulls", priority = 1)] KwNulls,
    #[regex(r"(?i)numeric", priority = 1)] KwNumeric,
    #[regex(r"(?i)object", priority = 1)] KwObject,
    #[regex(r"(?i)of", priority = 1)] KwOf,
    #[regex(r"(?i)off", priority = 1)] KwOff,
    #[regex(r"(?i)offset", priority = 1)] KwOffset,
    #[regex(r"(?i)oids", priority = 1)] KwOids,
    #[regex(r"(?i)old", priority = 1)] KwOld,
    #[regex(r"(?i)on", priority = 1)] KwOn,
    #[regex(r"(?i)only", priority = 1)] KwOnly,
    #[regex(r"(?i)operator", priority = 1)] KwOperator,
    #[regex(r"(?i)option", priority = 1)] KwOption,
    #[regex(r"(?i)options", priority = 1)] KwOptions,
    #[regex(r"(?i)or", priority = 1)] KwOr,
    #[regex(r"(?i)order", priority = 1)] KwOrder,
    #[regex(r"(?i)ordinality", priority = 1)] KwOrdinality,
    #[regex(r"(?i)others", priority = 1)] KwOthers,
    #[regex(r"(?i)out", priority = 1)] KwOut,
    #[regex(r"(?i)outer", priority = 1)] KwOuter,
    #[regex(r"(?i)over", priority = 1)] KwOver,
    #[regex(r"(?i)overlaps", priority = 1)] KwOverlaps,
    #[regex(r"(?i)overlay", priority = 1)] KwOverlay,
    #[regex(r"(?i)overriding", priority = 1)] KwOverriding,
    #[regex(r"(?i)owned", priority = 1)] KwOwned,
    #[regex(r"(?i)owner", priority = 1)] KwOwner,
    #[regex(r"(?i)parallel", priority = 1)] KwParallel,
    #[regex(r"(?i)parser", priority = 1)] KwParser,
    #[regex(r"(?i)partial", priority = 1)] KwPartial,
    #[regex(r"(?i)partition", priority = 1)] KwPartition,
    #[regex(r"(?i)passing", priority = 1)] KwPassing,
    #[regex(r"(?i)password", priority = 1)] KwPassword,
    #[regex(r"(?i)placing", priority = 1)] KwPlacing,
    #[regex(r"(?i)plans", priority = 1)] KwPlans,
    #[regex(r"(?i)policy", priority = 1)] KwPolicy,
    #[regex(r"(?i)position", priority = 1)] KwPosition,
    #[regex(r"(?i)preceding", priority = 1)] KwPreceding,
    #[regex(r"(?i)precision", priority = 1)] KwPrecision,
    #[regex(r"(?i)prepare", priority = 1)] KwPrepare,
    #[regex(r"(?i)prepared", priority = 1)] KwPrepared,
    #[regex(r"(?i)preserve", priority = 1)] KwPreserve,
    #[regex(r"(?i)primary", priority = 1)] KwPrimary,
    #[regex(r"(?i)prior", priority = 1)] KwPrior,
    #[regex(r"(?i)privileges", priority = 1)] KwPrivileges,
    #[regex(r"(?i)procedural", priority = 1)] KwProcedural,
    #[regex(r"(?i)procedure", priority = 1)] KwProcedure,
    #[regex(r"(?i)procedures", priority = 1)] KwProcedures,
    #[regex(r"(?i)program", priority = 1)] KwProgram,
    #[regex(r"(?i)publication", priority = 1)] KwPublication,
    #[regex(r"(?i)quote", priority = 1)] KwQuote,
    #[regex(r"(?i)range", priority = 1)] KwRange,
    #[regex(r"(?i)read", priority = 1)] KwRead,
    #[regex(r"(?i)real", priority = 1)] KwReal,
    #[regex(r"(?i)reassign", priority = 1)] KwReassign,
    #[regex(r"(?i)recheck", priority = 1)] KwRecheck,
    #[regex(r"(?i)recursive", priority = 1)] KwRecursive,
    #[regex(r"(?i)ref", priority = 1)] KwRef,
    #[regex(r"(?i)references", priority = 1)] KwReferences,
    #[regex(r"(?i)referencing", priority = 1)] KwReferencing,
    #[regex(r"(?i)refresh", priority = 1)] KwRefresh,
    #[regex(r"(?i)reindex", priority = 1)] KwReindex,
    #[regex(r"(?i)relative", priority = 1)] KwRelative,
    #[regex(r"(?i)release", priority = 1)] KwRelease,
    #[regex(r"(?i)rename", priority = 1)] KwRename,
    #[regex(r"(?i)repeatable", priority = 1)] KwRepeatable,
    #[regex(r"(?i)replace", priority = 1)] KwReplace,
    #[regex(r"(?i)replica", priority = 1)] KwReplica,
    #[regex(r"(?i)reset", priority = 1)] KwReset,
    #[regex(r"(?i)restart", priority = 1)] KwRestart,
    #[regex(r"(?i)restrict", priority = 1)] KwRestrict,
    #[regex(r"(?i)returning", priority = 1)] KwReturning,
    #[regex(r"(?i)returns", priority = 1)] KwReturns,
    #[regex(r"(?i)revoke", priority = 1)] KwRevoke,
    #[regex(r"(?i)right", priority = 1)] KwRight,
    #[regex(r"(?i)role", priority = 1)] KwRole,
    #[regex(r"(?i)rollback", priority = 1)] KwRollback,
    #[regex(r"(?i)rollup", priority = 1)] KwRollup,
    #[regex(r"(?i)routine", priority = 1)] KwRoutine,
    #[regex(r"(?i)routines", priority = 1)] KwRoutines,
    #[regex(r"(?i)row", priority = 1)] KwRow,
    #[regex(r"(?i)rows", priority = 1)] KwRows,
    #[regex(r"(?i)rule", priority = 1)] KwRule,
    #[regex(r"(?i)savepoint", priority = 1)] KwSavepoint,
    #[regex(r"(?i)schema", priority = 1)] KwSchema,
    #[regex(r"(?i)schemas", priority = 1)] KwSchemas,
    #[regex(r"(?i)scroll", priority = 1)] KwScroll,
    #[regex(r"(?i)search", priority = 1)] KwSearch,
    #[regex(r"(?i)second", priority = 1)] KwSecond,
    #[regex(r"(?i)security", priority = 1)] KwSecurity,
    #[regex(r"(?i)select", priority = 1)] KwSelect,
    #[regex(r"(?i)sequence", priority = 1)] KwSequence,
    #[regex(r"(?i)sequences", priority = 1)] KwSequences,
    #[regex(r"(?i)serializable", priority = 1)] KwSerializable,
    #[regex(r"(?i)server", priority = 1)] KwServer,
    #[regex(r"(?i)session", priority = 1)] KwSession,
    #[regex(r"(?i)session_user", priority = 1)] KwSessionUser,
    #[regex(r"(?i)set", priority = 1)] KwSet,
    #[regex(r"(?i)setof", priority = 1)] KwSetof,
    #[regex(r"(?i)sets", priority = 1)] KwSets,
    #[regex(r"(?i)share", priority = 1)] KwShare,
    #[regex(r"(?i)show", priority = 1)] KwShow,
    #[regex(r"(?i)similar", priority = 1)] KwSimilar,
    #[regex(r"(?i)simple", priority = 1)] KwSimple,
    #[regex(r"(?i)skip", priority = 1)] KwSkip,
    #[regex(r"(?i)smallint", priority = 1)] KwSmallint,
    #[regex(r"(?i)snapshot", priority = 1)] KwSnapshot,
    #[regex(r"(?i)some", priority = 1)] KwSome,
    #[regex(r"(?i)sql", priority = 1)] KwSql,
    #[regex(r"(?i)stable", priority = 1)] KwStable,
    #[regex(r"(?i)standalone", priority = 1)] KwStandalone,
    #[regex(r"(?i)start", priority = 1)] KwStart,
    #[regex(r"(?i)statement", priority = 1)] KwStatement,
    #[regex(r"(?i)statistics", priority = 1)] KwStatistics,
    #[regex(r"(?i)stdin", priority = 1)] KwStdin,
    #[regex(r"(?i)stdout", priority = 1)] KwStdout,
    #[regex(r"(?i)storage", priority = 1)] KwStorage,
    #[regex(r"(?i)stored", priority = 1)] KwStored,
    #[regex(r"(?i)strict", priority = 1)] KwStrict,
    #[regex(r"(?i)strip", priority = 1)] KwStrip,
    #[regex(r"(?i)subscription", priority = 1)] KwSubscription,
    #[regex(r"(?i)substring", priority = 1)] KwSubstring,
    #[regex(r"(?i)support", priority = 1)] KwSupport,
    #[regex(r"(?i)symmetric", priority = 1)] KwSymmetric,
    #[regex(r"(?i)sysid", priority = 1)] KwSysid,
    #[regex(r"(?i)system", priority = 1)] KwSystem,
    #[regex(r"(?i)table", priority = 1)] KwTable,
    #[regex(r"(?i)tables", priority = 1)] KwTables,
    #[regex(r"(?i)tablesample", priority = 1)] KwTablesample,
    #[regex(r"(?i)tablespace", priority = 1)] KwTablespace,
    #[regex(r"(?i)temp", priority = 1)] KwTemp,
    #[regex(r"(?i)template", priority = 1)] KwTemplate,
    #[regex(r"(?i)temporary", priority = 1)] KwTemporary,
    #[regex(r"(?i)text", priority = 1)] KwText,
    #[regex(r"(?i)then", priority = 1)] KwThen,
    #[regex(r"(?i)ties", priority = 1)] KwTies,
    #[regex(r"(?i)time", priority = 1)] KwTime,
    #[regex(r"(?i)timestamp", priority = 1)] KwTimestamp,
    #[regex(r"(?i)to", priority = 1)] KwTo,
    #[regex(r"(?i)trailing", priority = 1)] KwTrailing,
    #[regex(r"(?i)transaction", priority = 1)] KwTransaction,
    #[regex(r"(?i)transform", priority = 1)] KwTransform,
    #[regex(r"(?i)treat", priority = 1)] KwTreat,
    #[regex(r"(?i)trigger", priority = 1)] KwTrigger,
    #[regex(r"(?i)trim", priority = 1)] KwTrim,
    #[regex(r"(?i)true", priority = 1)] KwTrue,
    #[regex(r"(?i)truncate", priority = 1)] KwTruncate,
    #[regex(r"(?i)trusted", priority = 1)] KwTrusted,
    #[regex(r"(?i)type", priority = 1)] KwType,
    #[regex(r"(?i)types", priority = 1)] KwTypes,
    #[regex(r"(?i)unbounded", priority = 1)] KwUnbounded,
    #[regex(r"(?i)uncommitted", priority = 1)] KwUncommitted,
    #[regex(r"(?i)unencrypted", priority = 1)] KwUnencrypted,
    #[regex(r"(?i)union", priority = 1)] KwUnion,
    #[regex(r"(?i)unique", priority = 1)] KwUnique,
    #[regex(r"(?i)unknown", priority = 1)] KwUnknown,
    #[regex(r"(?i)unlisten", priority = 1)] KwUnlisten,
    #[regex(r"(?i)unlogged", priority = 1)] KwUnlogged,
    #[regex(r"(?i)until", priority = 1)] KwUntil,
    #[regex(r"(?i)update", priority = 1)] KwUpdate,
    #[regex(r"(?i)user", priority = 1)] KwUser,
    #[regex(r"(?i)using", priority = 1)] KwUsing,
    #[regex(r"(?i)vacuum", priority = 1)] KwVacuum,
    #[regex(r"(?i)valid", priority = 1)] KwValid,
    #[regex(r"(?i)validate", priority = 1)] KwValidate,
    #[regex(r"(?i)validator", priority = 1)] KwValidator,
    #[regex(r"(?i)value", priority = 1)] KwValue,
    #[regex(r"(?i)values", priority = 1)] KwValues,
    #[regex(r"(?i)varchar", priority = 1)] KwVarchar,
    #[regex(r"(?i)variadic", priority = 1)] KwVariadic,
    #[regex(r"(?i)varying", priority = 1)] KwVarying,
    #[regex(r"(?i)verbose", priority = 1)] KwVerbose,
    #[regex(r"(?i)version", priority = 1)] KwVersion,
    #[regex(r"(?i)view", priority = 1)] KwView,
    #[regex(r"(?i)views", priority = 1)] KwViews,
    #[regex(r"(?i)volatile", priority = 1)] KwVolatile,
    #[regex(r"(?i)when", priority = 1)] KwWhen,
    #[regex(r"(?i)where", priority = 1)] KwWhere,
    #[regex(r"(?i)whitespace", priority = 1)] KwWhitespace,
    #[regex(r"(?i)window", priority = 1)] KwWindow,
    #[regex(r"(?i)with", priority = 1)] KwWith,
    #[regex(r"(?i)within", priority = 1)] KwWithin,
    #[regex(r"(?i)without", priority = 1)] KwWithout,
    #[regex(r"(?i)work", priority = 1)] KwWork,
    #[regex(r"(?i)wrapper", priority = 1)] KwWrapper,
    #[regex(r"(?i)write", priority = 1)] KwWrite,
    #[regex(r"(?i)xml", priority = 1)] KwXml,
    #[regex(r"(?i)xmlattributes", priority = 1)] KwXmlattributes,
    #[regex(r"(?i)xmlconcat", priority = 1)] KwXmlconcat,
    #[regex(r"(?i)xmlelement", priority = 1)] KwXmlelement,
    #[regex(r"(?i)xmlexists", priority = 1)] KwXmlexists,
    #[regex(r"(?i)xmlforest", priority = 1)] KwXmlforest,
    #[regex(r"(?i)xmlnamespaces", priority = 1)] KwXmlnamespaces,
    #[regex(r"(?i)xmlparse", priority = 1)] KwXmlparse,
    #[regex(r"(?i)xmlpi", priority = 1)] KwXmlpi,
    #[regex(r"(?i)xmlroot", priority = 1)] KwXmlroot,
    #[regex(r"(?i)xmlserialize", priority = 1)] KwXmlserialize,
    #[regex(r"(?i)xmltable", priority = 1)] KwXmltable,
    #[regex(r"(?i)year", priority = 1)] KwYear,
    #[regex(r"(?i)yes", priority = 1)] KwYes,
    #[regex(r"(?i)zone", priority = 1)] KwZone,
}
