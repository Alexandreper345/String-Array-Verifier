use std::collections::HashMap;

fn is_vetor_par(value: [i8; 5]) -> HashMap<i8,u8>{
    


    //aqui eu crio um contador para que ele venha contar cada elemento da minha array permitindo
    //uma assinatura de  tipo explicita como o "i8 e u8" para que eles alocar numeros positivos e
    // negativos e criando uma instacia para chaves e valores     
    let mut contador: HashMap<i8,u8> = HashMap::new();
    



    //faço um for para interar com o valor da minha lista e dá a entrada do meu contador mutavel
    //dentro do valor da minha lista para que
    //ele venha passando de um em um de cada elemento do meu valor
    for num in value.iter(){
        *contador.entry(*num).or_insert(0)+=1;

    }
    //aqui eu criou uma tupla pra que assim o valor no  "_" que eu passar como exemplo o numero
    //alocado na memoria da lista e quantas vezes ele se repete, iterando tudo isso dentro do
    //contador mutavel.
    for (_,repeticoes) in contador.iter_mut(){
        if *repeticoes > 0{
           *repeticoes /= 2
        }
    }
    return contador
}


#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_is_par() {
            let list:[i8;5] = [1,2,2,3,4];
            let repete = is_vetor_par(list);
            assert_eq!(repete[&2],1);
    }
         #[test]
    fn test_is_par2(){
        let list:[i8;5] = [1,2,3,4,5];
        let repete = is_vetor_par(list);
        assert_eq!(repete[&1],0)
    }
}
