grammar Terrible;

// set :> type x = expression |
variable_assignment: 'set' ':>' type NAME '->' expression '|';
type: 'int' | 'float' | 'string';
expression: NUMBER;

NAME: [a-zA-Z][a-zA-Z0-9_]*;
NUMBER: '-'? [0-9]+;