# mash
Mahmoud Adas Shell, a bash-like shell for unix in rust

# Grammar
```
/* -------------------------------------------------------
   The grammar symbols
   ------------------------------------------------------- */
WORD            = ALPHA
                | WORD ALPHA
                | WORD '_'
ASSIGNMENT_WORD = WORD '=' WORD
NAME            = WORD
NEWLINE         = LF
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