import asyncio
from py.nautilus.util.index import NautilusUtils
from solana.rpc.async_api import AsyncClient
from solders.pubkey import Pubkey
from solders.keypair import Keypair

class Nautilus:
    
    connection: AsyncClient
    programId: Pubkey
    payer: Keypair

    util: NautilusUtils

    def __init__(self, connection, programId, payer):
        self.connection = connection
        self.programId = programId
        self.payer = payer
        self.util = NautilusUtils()