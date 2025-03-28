/// <reference types="astro/client" />

interface ImportMetaEnv {
  /** treat as localhost:3000 if not set */
  readonly PUBLIC_HOST: string | undefined;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
