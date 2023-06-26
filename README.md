# Iniziare con Rust
Si fa presente che questa documentazione non è rivolta a chi non ha mai programmato prima: si dà per scontate quindi nozioni basilari quali quelle di variabili, costrutti, metodi e altri. In caso non aveste queste nozioni, consiglio di riferirsi alle risorse citate alla fine di questo documento. \
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
## Sources
Di seguito abbiamo tutti i rimandi ai corsi e alle risorse che ho utilizzato per lo studio del linguaggio in caso voleste approfondire o in caso non vi fosse chiaro un concetto. Inoltre consiglio ugualmente di spulciare almeno gli inizi dei corsi in quanto è presente tutta la parte storica e concettuale non essenziale ai fini dello sviluppo, ma comunque molto interessanti.\
\
[**Documentazione Ufficiale**](https://doc.rust-lang.org/book/title-page.html) (Forma testuale)\
[**Corso di Google**](https://google.github.io/comprehensive-rust/welcome.html) (Forma testuale)\
[**Corso di Zero To Mastery**](https://www.youtube.com/watch?v=lzKeecy4OmQ) (Video)
[**Corso di freecodecamp**](https://www.freecodecamp.org/italian/news/impara-a-programmare-con-rust-corso-interattivo-in-linguaggio-rust-su-replit/#stringhe-e-slice-in-rust) (forma testuale o video)
