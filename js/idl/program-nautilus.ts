// import BN from "bn.js"

import { PublicKey } from "@solana/web3.js"

export type ProgramNautilusProgram = {
  tables: {
    person: string,
    home: string,
    car: string
  }
}









type Person = {
   id: number;
   name: string;
   authority: PublicKey;
};
type Home = {
   id: number;
   house_number: number;
   street: string;
};
type Car = {
   id: number;
   make: string;
   model: string;
   purchase_authority: PublicKey;
   operating_authority: PublicKey;
};