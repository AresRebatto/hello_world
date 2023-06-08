# Iniziare con Rust
Come prima cosa si necessita di scaricare ed installare Rust e Cargo, ovvero il gestore di pacchetti per Rust. Per farlo seguite questo link: https://rustup.rs/ .\
Premete su **rustup-init.exe**, scaricate e quindi installate l'esegubile.\
\
Una voltra installato il tutto potete aprire Visual Studio Code e aprire una cartella dove volete creare il nuovo progetto in Rust: aprite quindi una console di VS Code e digitate il seguente comando: \
\
**cargo new Hello_World**\
\
Facendo questo vedrete che si creerà una cartella col nome del vostro progetto(Nel nostro caso abbiamo scelto Hello_World, ma potete chiaramente scegliere anche altri nomi). Al momento però vi trovate all'esterno della
cartella, cosa che nel momento dell'esecuzione causerebbe dei problemi nel trovare l'esegubile. Per ovviare eseguire il seguente comando sempre sul terminale: \
\
**cd Hello_World** \
\
Facendo questo vi troverete nella cartella del progetto. Se al posto di Hello_World avete scelto un altro nome per il vostro progetto, dopo il cd mettete quello.\
A questo punto in alto a sinistra dovreste avere la cartella del progetto aperta. Andate in src e aprite il file denominato come "main.rs". Qui sarà il nostro punto di partenza da dove inizieremo a scrivere.\
\
Una volta scritto il vostro codice, se volete eseguirlo basterà che sempre da terminale eseguiate il seguente comando:\
\
**cargo run**\
\
Ora potete iniziare a studiare e programmare in Rust. Buona fortuna.
