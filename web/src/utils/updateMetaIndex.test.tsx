import { render } from '@testing-library/react';

import updateMetaIndex from './updateMetaIndex';

interface Test {
  title?: string;
  description?: string;
}

const tests: Test[] = [
  {
    title: 'Artifact Hub',
    description:
      'Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.',
  },
  {
    title: 'Kubernetes',
    description:
      'Kubernetes is an open-source system for automating deployment, scaling, and management of containerized applications.',
  },
  {
    title: 'Fluid',
    description:
      'Fluid is an orchestration platform for elastic data abstraction and acceleration in cloud native environment.',
  },
  { title: 'Rook', description: 'Storage Orchestration for Kubernetes.' },
];

const placeholder = {
  title: 'CLOMonitor',
  description:
    'CLOMonitor is a tool that periodically checks open source projects repositories to verify they meet a certain project health best practices.',
};

describe('updateMetaIndex', () => {
  it('renders default meta tags values', () => {
    const { container } = render(
      <>
        <title></title>
        <meta name="description" content="" />
        <meta property="og:type" content="website" />
        <meta property="og:title" content="" />
        <meta property="og:description" content="" />
        <meta property="og:image" content="https://clomonitor.io/static/media/clomonitor.png" />
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:title" content="" />
        <meta name="twitter:description" content="" />
        <meta name="twitter:image:src" content="https://clomonitor.io/static/media/clomonitor.png" />
      </>
    );

    updateMetaIndex();
    expect(container.querySelector(`meta[name='description']`)).toHaveAttribute('content', placeholder.description);
    expect(container.querySelector(`meta[property='og:description']`)).toHaveAttribute(
      'content',
      placeholder.description
    );
    expect(container.querySelector(`meta[name='twitter:description']`)).toHaveAttribute(
      'content',
      placeholder.description
    );
  });

  for (let i = 0; i < tests.length; i++) {
    it('returns proper object', () => {
      const { container } = render(
        <>
          <title></title>
          <meta name="description" content="" />
          <meta property="og:type" content="website" />
          <meta property="og:title" content="" />
          <meta property="og:description" content="" />
          <meta property="og:image" content="https://clomonitor.io/static/media/clomonitor.png" />
          <meta name="twitter:card" content="summary_large_image" />
          <meta name="twitter:title" content="" />
          <meta name="twitter:description" content="" />
          <meta name="twitter:image:src" content="https://clomonitor.io/static/media/clomonitor.png" />
        </>
      );

      updateMetaIndex(tests[i].title, tests[i].description);
      expect(document.title).toBe(tests[i].title);
      expect(container.querySelector(`meta[property='og:title']`)).toHaveAttribute('content', tests[i].title);
      expect(container.querySelector(`meta[name='twitter:title']`)).toHaveAttribute('content', tests[i].title);
      expect(container.querySelector(`meta[name='description']`)).toHaveAttribute('content', tests[i].description);
      expect(container.querySelector(`meta[property='og:description']`)).toHaveAttribute(
        'content',
        tests[i].description
      );
      expect(container.querySelector(`meta[name='twitter:description']`)).toHaveAttribute(
        'content',
        tests[i].description
      );
    });
  }
});
