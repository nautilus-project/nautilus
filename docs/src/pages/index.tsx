import Hero from "@/components/Hero";
import Navbar from "@/components/Navbar";
import SEO from "@/components/SEO";
import styles from "@/styles/Transitions.module.css";

export default function Home() {
  return (
    <>
      <SEO
        title="Home | Nautilus Project"
        description="An object-oriented, SQL-native Solana programming framework"
        image=""
      />
      <div className="w-full bg-[url(https://i.imgur.com/Fq6zXIb.jpg)] bg-no-repeat bg-cover lg:bg-center bg-fixed">
        <Navbar />
        <div className="flex flex-col min-h-screen container mx-auto items-center justify-center">
          <Hero />
        </div>
      </div>
    </>
  );
}
