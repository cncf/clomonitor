/// <reference types="vite/client" />
/// <reference types="react" />

declare module '*.svg' {
  const src: string;
  export default src;
}

declare module '*.svg?url' {
  const src: string;
  export default src;
}
