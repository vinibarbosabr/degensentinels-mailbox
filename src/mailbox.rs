#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc::derive_imports::*; // Precisa adicionar para #[derive(...)]

// Peguei #[derive...] do tutorial CryptoZombie, para as structs na MultiversX
//------
// ManagedBuffer is a wrapper around Vec<u8> | vsCode comment
// Melhor solução para memória na blockchain da MvX, alocando na SpaceVM
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Debug)]
pub struct Mail<M: ManagedTypeApi> {
    title: ManagedBuffer<M>, 
    author: ManagedAddress<M>, 
    message: ManagedBuffer<M>, 
}

#[multiversx_sc::contract]
pub trait Mailbox {
    #[init]
    fn init(&self) {
        self.mail_count().set(0);
    }

    #[upgrade]
    fn upgrade(&self) {}

    // VecMapper permite armazenar uma lista de emails em ordem;
    // Começando no index 1 e não 0.
    #[storage_mapper("mails")]
    fn mails(&self) -> VecMapper<Mail<Self::Api>>; 

    #[storage_mapper("mailCount")]
    fn mail_count(&self) -> SingleValueMapper<u64>;

    #[endpoint(sendMail)]
    fn send_mail(&self, title: ManagedBuffer, message: ManagedBuffer) {
       let new_mail = Mail {
            title,
            author: self.blockchain().get_caller(),
            message: message,
         };
        
        // Push add o new email para a list, em ordem.
        let new_mail_id = self.mails().push(&new_mail); 
        self.mail_count().set(new_mail_id as u64);
    }

    // Essa função retorna o último email enviado
    // Eu estava usando #[endpoint], mas no docs eles sugerem #[view] para read-only
    #[view(getLastMail)]
    fn get_last_mail(&self) -> Mail<Self::Api> {
        let mail_count = self.mail_count().get();
        require!(mail_count > 0, "No mails sent yet");
        self.mails().get(mail_count as usize)
    }

}
