fn main() {
	let variavel = 128;
	println!("variável = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

	let decimal:f32 = 2.5;
	println!("decimal = {}", decimal); 

	let mut booleana:bool = false;
	booleana = true;
	println!("booleada = {}, tamanho {} bytes", booleana, std::mem::size_of_val(&booleana));

	let letra:char = 'C';
	println!("letra = {}, tamanho = {} bytes", letra, std::mem::size_of_val(&letra));
}