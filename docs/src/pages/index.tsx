import Hero from '@/components/Hero'
import Head from 'next/head'

export default function Home() {
  return (
    <>
      <Head>
        <title>Nautilus</title>
        <meta name="description" content="Solana Program Framework" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <Hero/>
      </main>
    </>
  )
}
