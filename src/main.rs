fn main() {
    println!("Hello, world!");//println! per stampare a schermo
    let mut n  = 43; /*Dichiariamo una variabile. Se non mettiamo mut, la variabile verrà automaticamente presa come immutable e 
    non potrà essere modificata in seguito. I due punti seguiti da i32 servono per specificare il tipo: nel nostro caso un intero a 32 bit*/
    n = 42; 

    let x: i32 = 4;
    let x: i32 = 5; //La possibilità di creare una variabile con lo stesso nome di un'altra precedente variabile è detta Shadowing: si dice 
    //che la seconda variabile ha messo in ombra (shadowed) la prima: quando andremo quindi a richiarmare la variabile x, avrà valore 5.
    //ATTENZIONE. Quello che abbiamo fatto non è stato cambiare il valore della variabile, ma sovrascrivere l'intera variabile con una nuova
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