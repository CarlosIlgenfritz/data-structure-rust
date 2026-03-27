use std::alloc::{Layout, alloc};
use std::{mem, ptr};

/*
    Dynamic Vector - Constraints
    Requisitos Básicos:

    Armazenamento dinâmico - cresce automaticamente conforme necessário
    Acesso por índice - O(1) para acessar elemento em qualquer posição
    Inserção no final - O(1) amortizado (push_back)
    Remoção do final - O(1) (pop_back)
    Redimensionamento - quando capacidade é atingida, deve alocar mais memória

    Operações Obrigatórias:

push_back(value) - adicionar elemento no final
pop_back() - remover elemento do final
get(index) - acessar elemento por índice
set(index, value) - modificar elemento
size() - retornar quantidade de elementos
capacity() - retornar espaço alocado
is_empty() - verificar se está vazio
insert(index, value) - inserir em posição específica
remove(index) - remover de posição específica
clear() - limpar o vetor

*/
pub struct Vector<T> {
    pointer: *mut T,
    len: usize,      // tamanho atual
    capacity: usize, //espaco alocado
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector {
            pointer: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }

        let layout = Layout::from_size_align(capacity * size_of::<T>(), align_of::<T>())
            .expect("Layout inválido");
        let pointer = unsafe { alloc(layout) as *mut T };
        if pointer.is_null() {
            panic!("Falha ao alocar memória para Vector");
        }

        Vector {
            pointer,
            len: 0,
            capacity,
        }
    }
}
