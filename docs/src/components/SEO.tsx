import Head from "next/head";
import globalConfig from "../../config/global.config";
import { SEOProps } from "../../interfaces/seo";
export default function SEO({ title, description, image }: SEOProps) {
  return (
    <Head>
      <title>{title}</title>
      <meta name="description" content={description} />
      <meta name="image" content={image} />
      <meta property="og:title" content={title} />
      <meta property="og:description" content={description} />
      <meta property="og:image" content={image} />
      <meta property="og:type" content="website" />
      <meta
        property="og:url"
        content={`https://${globalConfig.general.domain}/`}
      />
      <meta property="og:site_name" content={globalConfig.general.name} />
      <meta name="twitter:card" content="summary_large_image" />
      <meta name="twitter:title" content={title} />
      <meta name="twitter:description" content={description} />
      <meta name="twitter:image" content={image} />
      <meta name="twitter:image:alt" content={description} />
      <meta name="twitter:site" content={globalConfig.general.twitter} />
      <meta name="twitter:creator" content={globalConfig.general.twitter} />
      <link rel="icon" href="/favicon.ico" />
    </Head>
  );
}
