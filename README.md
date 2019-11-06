# mash
Mahmoud Adas Shell, a bash-like shell for unix in rust

# Grammar
```
/* -------------------------------------------------------
   The grammar symbols
   ------------------------------------------------------- */
WORD            = ALPHA
                | WORD ALPHANUM
                | WORD '_'
                = ALPHA (ALPHA | DIGIT | '_')+ ;
ASSIGNMENT_WORD = WORD '=' WORD ;
NAME            = WORD ;
NEWLINE         = LF = '\n'
IO_NUMBER       = DIGIT


/* The following are the operators (see XBD Operator)
   containing more than one character. */


OR_IF = '||'
AND_IF         = '&&'           
DSEMI = ';;'
DLESSDASH = '<<-'
LESSGREAT   = '<>'  
GREATAND   = '>&'           
LESSAND   = '<&'     
DGREAT   = '>>'    
DLESS   = '<<'   
CLOBBER = '>|'


/* The following are the reserved words. */

DO    = 'do'  
DONE = 'done'
IF     = 'if'  
THEN     = 'then'  
ELSE     = 'else'  
ELIF     = 'elif'
FI     = 'fi' 
CASE = 'case'    
ESAC = 'esac'
WHILE = 'while'
UNTIL =  'until'   
FOR = 'for'


/* These are reserved words, not operator tokens, and are
   recognized when reserved words are recognized. */
RBRACE    = '}'   
LBRACE    = '{'       
BANG = '!'
IN = 'in'


/* -------------------------------------------------------
   The Grammar
   ------------------------------------------------------- */
%start program
%%
program          : linebreak complete_commands linebreak
                 | linebreak
                 ;
complete_commands: complete_commands newline_list complete_command
                 |                                complete_command
                 ;
complete_command : list separator_op
                 | list
                 ;
list             : list separator_op and_or
                 |                   and_or
                 ;
and_or           :                         pipeline
                 | and_or AND_IF linebreak pipeline
                 | and_or OR_IF  linebreak pipeline
                 ;
pipeline         :      pipe_sequence
                 | BANG pipe_sequence
                 ;
pipe_sequence    :                             command
                 | pipe_sequence '|' linebreak command
                 ;
command          : simple_command
                 | compound_command
                 | compound_command redirect_list
                 | function_definition
                 ;
compound_command : brace_group
                 | subshell
                 | for_clause
                 | case_clause
                 | if_clause
                 | while_clause
                 | until_clause
                 ;
subshell         : '(' compound_list ')'
                 ;
compound_list    : linebreak term
                 | linebreak term separator
                 ;
term             : term separator and_or
                 |                and_or
                 ;
for_clause       : FOR name                                      do_group
                 | FOR name                       sequential_sep do_group
                 | FOR name linebreak in          sequential_sep do_group
                 | FOR name linebreak in wordlist sequential_sep do_group
                 ;
name             : NAME                     /* Apply rule 5 */
                 ;
in               : IN                       /* Apply rule 6 */
                 ;
wordlist         : wordlist WORD
                 |          WORD
                 ;
case_clause      : CASE WORD linebreak in linebreak case_list    ESAC
                 | CASE WORD linebreak in linebreak case_list_ns ESAC
                 | CASE WORD linebreak in linebreak              ESAC
                 ;
case_list_ns     : case_list case_item_ns
                 |           case_item_ns
                 ;
case_list        : case_list case_item
                 |           case_item
                 ;
case_item_ns     :     pattern ')' linebreak
                 |     pattern ')' compound_list
                 | '(' pattern ')' linebreak
                 | '(' pattern ')' compound_list
                 ;
case_item        :     pattern ')' linebreak     DSEMI linebreak
                 |     pattern ')' compound_list DSEMI linebreak
                 | '(' pattern ')' linebreak     DSEMI linebreak
                 | '(' pattern ')' compound_list DSEMI linebreak
                 ;
pattern          :             WORD         /* Apply rule 4 */
                 | pattern '|' WORD         /* DO not apply rule 4 */
                 ;
if_clause        : IF compound_list THEN compound_list else_part FI
                 | IF compound_list THEN compound_list           FI
                 ;
else_part        : ELIF compound_list THEN compound_list
                 | ELIF compound_list THEN compound_list else_part
                 | ELSE compound_list
                 ;
while_clause     : WHILE compound_list do_group
                 ;
until_clause     : UNTIL compound_list do_group
                 ;
function_definition : fname '(' ')' linebreak function_body
                 ;
function_body    : compound_command                /* Apply rule 9 */
                 | compound_command redirect_list  /* Apply rule 9 */
                 ;
fname            : NAME                            /* Apply rule 8 */
                 ;
brace_group      : LBRACE compound_list RBRACE
                 ;
do_group         : DO compound_list DONE           /* Apply rule 6 */
                 ;
simple_command   : cmd_prefix cmd_word cmd_suffix
                 | cmd_prefix cmd_word
                 | cmd_prefix
                 | cmd_name cmd_suffix
                 | cmd_name
                 ;
cmd_name         : WORD                   /* Apply rule 7a */
                 ;
cmd_word         : WORD                   /* Apply rule 7b */
                 ;
cmd_prefix       :            io_redirect
                 | cmd_prefix io_redirect
                 |            ASSIGNMENT_WORD
                 | cmd_prefix ASSIGNMENT_WORD
                 ;
cmd_suffix       :            io_redirect
                 | cmd_suffix io_redirect
                 |            WORD
                 | cmd_suffix WORD
                 ;
redirect_list    :               io_redirect
                 | redirect_list io_redirect
                 ;
io_redirect      :           io_file
                 | IO_NUMBER io_file
                 |           io_here
                 | IO_NUMBER io_here
                 ;
io_file          : '<'       filename
                 | LESSAND   filename
                 | '>'       filename
                 | GREATAND  filename
                 | DGREAT    filename
                 | LESSGREAT filename
                 | CLOBBER   filename
                 ;
filename         : WORD                      /* Apply rule 2 */
                 ;
io_here          : DLESS     here_end
                 | DLESSDASH here_end
                 ;
here_end         : WORD                      /* Apply rule 3 */
                 ;
newline_list     :              NEWLINE
                 | newline_list NEWLINE
                 ;
linebreak        : newline_list
                 | /* empty */
                 ;
separator_op     : '&'
                 | ';'
                 ;
separator        : separator_op linebreak
                 | newline_list
                 ;
sequential_sep   : ';' linebreak
                 | newline_list
                 ;
```
# Rules
```
[Command Name]

When the TOKEN is exactly a reserved word, the token identifier for that reserved word shall result. Otherwise, the token WORD shall be returned. Also, if the parser is in any state where only a reserved word could be the next correct token, proceed as above.

Note:
Because at this point <quotation-mark> characters are retained in the token, quoted strings cannot be recognized as reserved words. This rule also implies that reserved words are not recognized except in certain positions in the input, such as after a <newline> or <semicolon>; the grammar presumes that if the reserved word is intended, it is properly delimited by the user, and does not attempt to reflect that requirement directly. Also note that line joining is done before tokenization, as described in Escape Character (Backslash), so escaped <newline> characters are already removed at this point.
Rule 1 is not directly referenced in the grammar, but is referred to by other rules, or applies globally.

[Redirection to or from filename]

The expansions specified in Redirection shall occur. As specified there, exactly one field can result (or the result is unspecified), and there are additional requirements on pathname expansion.

[Redirection from here-document]

Quote removal shall be applied to the word to determine the delimiter that is used to find the end of the here-document that begins after the next <newline>.

[Case statement termination]

When the TOKEN is exactly the reserved word esac, the token identifier for esac shall result. Otherwise, the token WORD shall be returned.

[ NAME in for]

When the TOKEN meets the requirements for a name (see XBD Name ), the token identifier NAME shall result. Otherwise, the token WORD shall be returned.

[Third word of for and case]

[ case only]

When the TOKEN is exactly the reserved word in, the token identifier for in shall result. Otherwise, the token WORD shall be returned.

[ for only]

When the TOKEN is exactly the reserved word in or do, the token identifier for in or do shall result, respectively. Otherwise, the token WORD shall be returned.

(For a. and b.: As indicated in the grammar, a linebreak precedes the tokens in and do. If <newline> characters are present at the indicated location, it is the token after them that is treated in this fashion.)

[Assignment preceding command name]

[When the first word]

If the TOKEN does not contain the character '=', rule 1 is applied. Otherwise, 7b shall be applied.

[Not the first word]

If the TOKEN contains an unquoted (as determined while applying rule 4 from Token Recognition) <equals-sign> character that is not part of an embedded parameter expansion, command substitution, or arithmetic expansion construct (as determined while applying rule 5 from Token Recognition):

If the TOKEN begins with '=', then rule 1 shall be applied.

If all the characters in the TOKEN preceding the first such <equals-sign> form a valid name (see XBD Name), the token ASSIGNMENT_WORD shall be returned.

Otherwise, it is unspecified whether rule 1 is applied or ASSIGNMENT_WORD is returned.

Otherwise, rule 1 shall be applied.

Assignment to the name within a returned ASSIGNMENT_WORD token shall occur as specified in Simple Commands.

[ NAME in function]

When the TOKEN is exactly a reserved word, the token identifier for that reserved word shall result. Otherwise, when the TOKEN meets the requirements for a name, the token identifier NAME shall result. Otherwise, rule 7 applies.

[Body of function]

Word expansion and assignment shall never occur, even when required by the rules above, when this rule is being parsed. Each TOKEN that might either be expanded or have assignment applied to it shall instead be returned as a single WORD consisting only of characters that are exactly the token described in Token Recognition.
```