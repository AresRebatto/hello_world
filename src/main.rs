fn main() {
    println!("Hello, world!");//println! per stampare a schermo
    let mut n  = 43; /*Dichiariamo una variabile. Se non mettiamo mut, la variabile verrà automaticamente presa come immutable e 
    non potrà essere modificata in seguito. I due punti seguiti da i32 servono per specificare il tipo: nel nostro caso un intero a 32 bit*/
    n = 42; 

    let x: i32 = 4;
    let x: i32 = 5; //La possibilità di creare una variabile con lo stesso nome di un'altra precedente variabile è detta Shadowing: si dice 
    //che la seconda variabile ha messo in ombra (shadowed) la prima: quando andremo quindi a richiarmare la variabile x, avrà valore 5.
    //ATTENZIONE. Quello che abbiamo fatto non è stato cambiare il valore della variabile, ma sovrascrivere l'intera variabile con una nuova
    
    let tup: (i32, f64) = (4, 8.09); //Tuple composto da un intero a 32 bit e un float a 64

    let primoElemento: i32 = tup.0; //Memorizza in una variabile l'elemento 0 del Tuple(4)
    println!("{primoElemento}");
    
    if n < 100
    {
        while n < 100 
        {
            n = n+1;
        }
        /*Per un ciclo infinito si utilizza la keyword loop al posto del while e non si mette nessuna condizione nel seguente modo: 
        loop
        {
            
        } */
        
    }else if n > 100
    {
        println!("{n} is greater than 100")
    }
    
    println!("{}", Somma(n, 4));//Stampa il risultato che ritorna la variabile Somma avendo come parametri n(Che ora è pari a 100) e 4: stamperà quindi 104
    Ciao();//Stampa "CIAO!"
    Array();
    Stringhe();

    let (somma, moltiplicazione) = Calculate(4, 6);//Dichiara due variabile e memorizza al loro interno
    //I due rispettivamente della somma e della moltiplicazione dati dalla funzione Calculate
    

}

fn Somma(a: i32, b: i32) -> i32 /* tra i parametri mettiamo il nome che gli diamo (a e b nel nostro caso) seguito dai due punti e dal tipo dei parametri
 E dopo la parentesi tonda mettiamo il tipo che il metodo ritorna: nel nostro caso un intero a 32 bit*/
{
    return a+b;
}

fn Ciao()//Per una funzione di tipo void basta non specificare il tipo di valore che ritorna(basta quindi no scrivere -> TipoDato)
{
    println!("CIAO!");
}

fn Calculate(n1: i32, n2: i32) -> (i32, i32) //Richiede due parametri e dà in output due valori
{
    return (n1+n2, n1*n2) //Ritorna sia la somma che la moltipliazione dei due numeri
}

//Da qui in poi vi saranno solo dei metodi. Per ogni argomento ci sarà un metodo: lo faccio per dare un senso di ordine al tutto

fn Array()
{
    let mut array: [i32; 5] = [10, 4, 5, 32, 2]; //In questa riga abbiamo dichiarato un'array di interi composta da 5 elementi e gli abbiamo inserito i nostri valori
    array[0] = 4; //Sostituzione valore in posizione 0 (10 -> 4)

    for element in array //Si usa il for come se fosse un foreach
    {
        println!("{element}");
    }

    for i in (1..4) //Qui invece lo usiamo come il tipico for che si ha in altri linguaggi. Comprende il primo
    //numero (1) e esclude l'ultimo(4). In caso si volesse far decrescere il valore (da 4 a 1), dopo l'intervallo andrebbe messo .rev() in questo modo
    // for i in (1..4).rev()
    {
        println!("{i}");//Stampa da 1 a 3
    }
}

//Ownership: regole per governare la memoria
//Heap: spazio di memoria dove i dati vengono allocati sennza un ordine preciso. Cerca semplicemente uno spazio libero e
//      vede se è sufficiente, quindi memorizza il dato e ritorna il puntatore, ovvero l'indirizzo di dove si trova il dato
//Stack: Tipo di memoria in qui i dati vengono inseriti in modalità LIFO(Last In First Out). E' più veloce l'inserimento di 
//       dati in questo tipo di memoria poiché non si fa una ricerca di uno spazio di memoria libero, ma si posiziona 
//       semplicemente il dato in cima allo stack

fn Stringhe()
{
    //Tipo che gestisce i dati allocati nell'heap
    /*Ci permette di memorizzare un quantitativo di testo a noi sconosciuto:
    Utile ad esempio quando chiediamo un valore in input da un utente
    */

    let stringa1: String = String::from("CIAO"); //Variabile stringa con dentro "CIAO"
    let stringa2: String = stringa1.clone() + &String::from(" Mondo"); // Copiamo la prima stringa nella seconda e gli aggiungiamo mondo
    println!("{stringa1}");
    println!("{stringa2}"); 
}