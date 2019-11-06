# mash
Mahmoud Adas Shell, a bash-like shell for unix in rust

# Grammar
```
WORD            = ALPHA
                / WORD ALPHA
                / WORD "_"
ASSIGNMENTWORD = WORD "=" WORD
NAME           = WORD
NEWLINE        = LF
IONUMBER       = DIGIT

; The following are the operators mentioned above.

ANDIF    = "&&"
ORIF     = "||"
DSEMI    = ";;"

DLESS     = "<<"
DGREAT    = ">>"
LESSAND   = "<&"
GREATAND  = ">&"
LESSGREAT = "<>"
DLESSDASH = "<<-"

CLOBBER   = ">|"

; The following are the reserved words.

If   = %s" if "
Then = %s" then "
Else = %s" else "
Elif = %s" elif "
Fi   = %s" fi"
Do   = %s" do "
Done = %s" done "

Case  = %s" case "
Esac  = %s" esac "
While = %s" while "
Until = %s" until "
For   = %s" for "

; These are reserved words, not operator tokens, and are
;  recognized when reserved words are recognized.


Lbrace = "{"
Rbrace = "}"
Bang   = "!"

In = %s" in "

program          = linebreak completecommands linebreak
                 / linebreak

completecommands = completecommands newlinelist completecommand
                 / completecommand

completecommand  = list separatorop
                 / list

list             = list separatorop andor
                 /                  andor

andor            =                       pipeline
                 / andor ANDIF linebreak pipeline
                 / andor ORIF  linebreak pipeline

pipeline         =      pipesequence
                 / Bang pipesequence

pipesequence     =                            command
                 / pipesequence "|" linebreak command

command          = simplecommand
                 / compoundcommand
                 / compoundcommand redirectlist
                 / functiondefinition

compoundcommand  = bracegroup
                 / subshell
                 / forclause
                 / caseclause
                 / ifclause
                 / whileclause
                 / untilclause

subshell         = "(" compoundlist ")"

compoundlist     = linebreak term
                 / linebreak term separator

term             = term separator andor
                 /                andor

forclause        = For name                                     dogroup
                 / For name                       sequentialsep dogroup
                 / For name linebreak in          sequentialsep dogroup
                 / For name linebreak in wordlist sequentialsep dogroup

;name             = NAME                     ;/* Apply rule 5 */

;in               = In                       ;/* Apply rule 6 */

wordlist         = wordlist WORD
                 /          WORD

caseclause       = Case WORD linebreak in linebreak caselist   Esac
                 / Case WORD linebreak in linebreak caselistns Esac
                 / Case WORD linebreak in linebreak            Esac
                 ;
caselistns       = caselist caseitemns
                 /          caseitemns

caselist         = caselist caseitem
                 /          caseitem

caseitemns       =     pattern ")"              linebreak
                 /     pattern ")" compoundlist
                 / "(" pattern ")"              linebreak
                 / "(" pattern ")" compoundlist

caseitem         =     pattern ")" linebreak    DSEMI linebreak
                 /     pattern ")" compoundlist DSEMI linebreak
                 / "(" pattern ")" linebreak    DSEMI linebreak
                 / "(" pattern ")" compoundlist DSEMI linebreak

pattern          =             WORD         ;/* Apply rule 4 */
                 / pattern "|" WORD         ;/* Do not apply rule 4 */

ifclause         = If compoundlist Then compoundlist elsepart Fi
                 / If compoundlist Then compoundlist          Fi

elsepart         = Elif compoundlist Then compoundlist
                 / Elif compoundlist Then compoundlist elsepart
                 / Else compoundlist

whileclause      = While compoundlist dogroup

untilclause      = Until compoundlist dogroup

functiondefinition = fname "(" ")" linebreak functionbody

functionbody     = compoundcommand               ;/* Apply rule 9 */
                 / compoundcommand redirectlist  ;/* Apply rule 9 */

fname            = NAME                            ;/* Apply rule 8 */

bracegroup       = Lbrace compoundlist Rbrace

dogroup          = Do compoundlist Done           ;/* Apply rule 6 */

simplecommand    = cmdprefix cmdword cmdsuffix
                 / cmdprefix cmdword
                 / cmdprefix
                 / cmdname cmdsuffix
                 / cmdname

cmdname          = WORD                   ;/* Apply rule 7a */

cmdword          = WORD                   ;/* Apply rule 7b */

cmdprefix        =           ioredirect
                 / cmdprefix ioredirect
                 /           ASSIGNMENTWORD
                 / cmdprefix ASSIGNMENTWORD

cmdsuffix        =           ioredirect
                 / cmdsuffix ioredirect
                 /           WORD
                 / cmdsuffix WORD

redirectlist     =              ioredirect
                 / redirectlist ioredirect

ioredirect       =          iofile
                 / IONUMBER iofile
                 /          iohere
                 / IONUMBER iohere

iofile           = "<"       filename
                 / LESSAND   filename
                 / ">"       filename
                 / GREATAND  filename
                 / DGREAT    filename
                 / LESSGREAT filename
                 / CLOBBER   filename

filename         = WORD                      ;/* Apply rule 2 */

;iohere           = DLESS     hereend
;                 / DLESSDASH hereend

iohere           = DLESS     hereend *(*ALPHA / NEWLINE) NEWLINE hereend NEWLINE
                 / DLESSDASH hereend *(*ALPHA / NEWLINE) NEWLINE hereend NEWLINE

;hereend          = WORD                      ;/* Apply rule 3 */
hereend          = %s"EOF"

newlinelist      =             NEWLINE
                 / newlinelist NEWLINE

linebreak        = newlinelist
                 / ""            ; empty

separatorop      = "&"
                 / ";"

separator        = separatorop linebreak
                 / newlinelist

sequentialsep    = ";" linebreak
                 / newlinelist
```