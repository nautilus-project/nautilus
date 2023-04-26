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
      <div className="bg-[url(https://i.imgur.com/Fq6zXIb.jpg)] backdrop-blur-md bg-no-repeat bg-cover bg-center bg-fixed">
        <div className="bg-cover bg-fixed backdrop-blur-md pt-2">
          <div className={styles.fadeInUp}>
            <Navbar />
          </div>
          <main>
            <Hero />
          </main>
        </div>
      </div>
    </>
  );
}
