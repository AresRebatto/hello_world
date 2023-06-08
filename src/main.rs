fn main() {
    println!("Hello, world!");//println! per stampare a schermo
    let mut n  = 43; /*Dichiariamo una variabile. Se non mettiamo mut, la variabile verrà automaticamente presa come immutable e 
    non potrà essere modificata in seguito*/
    n = 42; 
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

}

fn Somma(a: i32, b: i32) -> i32 /* tra i parametri mettiamo il nome che gli diamo (a e b nel nostro caso) seguito dai due punti e dal tipo dei parametri
 E dopo la parentesi tonda mettiamo il tipo che il metodo ritorna: nel nostro caso un intero a 32 bit*/
{
    return a+b;
}

fn Ciao()//Per una funzione di tipo void basta non specificare il tipo di valore che ritorna(basta quindi no scrivere -> TipoDato)
{
    print!("CIAO!");
}
