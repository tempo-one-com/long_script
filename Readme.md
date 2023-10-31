# Note
Ce programme execute un processus et au bout de n secondes (passé en argument de commande) écrit sur le console

# Installation de Rust
## Linux / Mac
```sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sudo sh```
ou
## Windows
```https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe```

# Build (debug => plus rapide)
```cargo build```

# Build (optimisé pour prod => plus lent)
```cargo build --release```

# Copier le fichier binaire à la racine du projet
## Linux / Mac
```ln -s target/debug/long_script .```  
ou  
## Windows
```copy target\debug\long_script.exe .```

# Exécution
Lancer le script avec un temps d'exécution en secondes  
(Pour provoquer une erreur passer un délai négatif)

Exemple :  
## Linux / Mac
```./long_script 3```  
ou  
## Windows
```.\long_script.exe 3```

