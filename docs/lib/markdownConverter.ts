import fs from "fs";
import path from "path";
import matter from "gray-matter";

const contentDirectory = path.join(process.cwd(), "md");

export function getFiles() {
  return fs
    .readdirSync(contentDirectory)
    .filter((file) => fs.statSync(path.join(contentDirectory, file)).isFile());
}

export async function getFileData(slug: string) {
  const fullPath = path.join(contentDirectory, `${slug}.md`);
  const fileContents = fs.readFileSync(fullPath, "utf8");
  const matterResult = matter(fileContents);

  return {
    frontmatter: matterResult.data,
    slug: slug,
    content: matterResult.content,
  };
}

export function getAllPostsData() {
  const files = getFiles();
  const posts = files.map((file) => {
    const fileNameWithoutExtension = file.replace(/\.md$/, "");
    return {
      slug: fileNameWithoutExtension,
      title: fileNameWithoutExtension.replace(/-/g, " "),
    };
  });
  return posts;
}