#   Exemples

Ce dépôt contient de petits exemples de code source pour lesquels nous
allons produire du code compilé. Il contient aussi des exemples
simples de code assembleur produit par compilation.

Vous y trouverez un `Makefile` permettant de :

-   générer l’AST au format JSON pour un code source en appelant le
    parseur de Babel : `make 01-expressions.json`, etc.
-   afficher l’AST, toujours au format JSON, mais en filtrant beaucoup
    d’informations pas très utiles pour un humain : `make
    01-expressions.ast`, etc.
-   générer les AST pour tous les fichiers `.js` du répertoire :
    `make`.
