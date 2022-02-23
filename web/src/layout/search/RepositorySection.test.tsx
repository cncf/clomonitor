import { render, screen } from '@testing-library/react';

import { RepositoryKind } from '../../types';
import RepositorySection from './RepositorySection';

const defaultPropsOneRepo = {
  repositories: [
    {
      kind: RepositoryKind.Primary,
      name: 'artifact-hub',
      url: 'https://github.com/artifacthub/hub',
    },
  ],
};

const defaultPropsRepos = {
  repositories: [
    {
      kind: RepositoryKind.Secondary,
      name: 'sdk-go',
      url: 'https://github.com/cloudevents/sdk-go',
    },
    {
      kind: RepositoryKind.Secondary,
      name: 'sdk-javascript',
      url: 'https://github.com/cloudevents/sdk-javascript',
    },
    {
      kind: RepositoryKind.Secondary,
      name: 'sdk-csharp',
      url: 'https://github.com/cloudevents/sdk-csharp',
    },
    {
      kind: RepositoryKind.Secondary,
      name: 'sdk-python',
      url: 'https://github.com/cloudevents/sdk-python',
    },
    {
      kind: RepositoryKind.Primary,
      name: 'spec',
      url: 'https://github.com/cloudevents/spec',
    },
    {
      kind: RepositoryKind.Secondary,
      name: 'sdk-java',
      url: 'https://github.com/cloudevents/sdk-java',
    },
  ],
};

describe('RepositorySection', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<RepositorySection {...defaultPropsRepos} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders with more than one repo', () => {
      render(<RepositorySection {...defaultPropsRepos} />);

      expect(screen.getByText('Repositories')).toBeInTheDocument();
      expect(screen.getAllByRole('link', { name: 'Repository link' })).toHaveLength(6);
    });

    it('renders with one repo', () => {
      render(<RepositorySection {...defaultPropsOneRepo} />);

      expect(screen.getByText('Repository')).toBeInTheDocument();
      expect(screen.getByRole('link', { name: 'Repository link' })).toBeInTheDocument();
      expect(screen.getByRole('link', { name: 'Repository link' })).toHaveAttribute(
        'href',
        'https://github.com/artifacthub/hub'
      );
    });
  });
});
