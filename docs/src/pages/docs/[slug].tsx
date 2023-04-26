import DocumentationLayout from "@/components/DocsLayout";
import SEO from "@/components/SEO";
import matter from "gray-matter";
import { remark } from "remark";
import html from "remark-html";
import { getAllPostsData, getFileData } from "../../../lib/markdownConverter";

export default function Doc({ postData }: any) {
  return (
    <>
      <SEO
        title={`${postData?.frontmatter?.title} | Nautilus Project`}
        description={`${postData?.frontmatter?.description}`}
        image=""
      />
      <main className="bg-white">
        <DocumentationLayout
          title={postData?.frontmatter?.title}
          description={postData?.frontmatter?.description}
          previous={postData?.frontmatter?.previous}
          previousLink={postData?.frontmatter?.previousLink}
          next={postData?.frontmatter?.next}
          nextLink={postData?.frontmatter?.nextLink}
          content={postData?.contentHtml}
        />
      </main>
    </>
  );
}

export async function getStaticPaths() {
  const allPostsData = getAllPostsData();
  const paths = allPostsData.map(({ slug }) => ({
    params: { slug },
  }));

  return { paths, fallback: true };
}

export async function getStaticProps({ params }: any) {
  const postData = await getFileData(params.slug);
  const { data: frontmatter } = matter(postData);
  const processedContent = await remark().use(html).process(postData.content);
  const contentHtml = processedContent.toString();

  return {
    props: {
      postData: {
        ...postData,
        ...frontmatter,
        contentHtml,
      },
    },
  };
}
