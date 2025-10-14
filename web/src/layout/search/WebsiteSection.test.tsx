import { act, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { CheckSet } from 'clo-ui/components/CheckSetBadge';

import WebsiteSection from './WebsiteSection';

const user = userEvent.setup({ delay: null });

const defaultPropsOneRepo = {
  repositories: [
    {
      check_sets: [CheckSet.Community, CheckSet.Code],
      name: 'artifact-hub',
      url: 'https://github.com/artifacthub/hub',
      website_url: 'https://artifacthub.io',
    },
  ],
};

const defaultPropsRepos = {
  repositories: [
    {
      check_sets: [CheckSet.Code],
      name: 'sdk-go',
      url: 'https://github.com/cloudevents/sdk-go',
    },
    {
      check_sets: [CheckSet.Code],
      name: 'sdk-javascript',
      url: 'https://github.com/cloudevents/sdk-javascript',
    },
    {
      check_sets: [CheckSet.Code],
      name: 'sdk-csharp',
      url: 'https://github.com/cloudevents/sdk-csharp',
    },
    {
      check_sets: [CheckSet.Code],
      name: 'sdk-python',
      url: 'https://github.com/cloudevents/sdk-python',
    },
    {
      check_sets: [CheckSet.Community],
      name: 'spec',
      url: 'https://github.com/cloudevents/spec',
      website_url: 'https://cloudevents.io',
    },
    {
      check_sets: [CheckSet.Code],
      name: 'sdk-java',
      url: 'https://github.com/cloudevents/sdk-java',
      website_url: 'https://cloudevents.io/test',
    },
  ],
};

describe('WebsiteSection', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<WebsiteSection {...defaultPropsRepos} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders with more than one website url', async () => {
      jest.useFakeTimers();

      render(<WebsiteSection {...defaultPropsRepos} />);

      const content = screen.getByText('Websites');
      expect(content).toBeInTheDocument();
      const dropdown = screen.getByRole('complementary');

      expect(dropdown).not.toHaveClass('show');

      await user.hover(content);

      act(() => {
        jest.advanceTimersByTime(100);
      });

      expect(dropdown).toHaveClass('show');
      expect(screen.getAllByRole('link', { name: 'Website link' })).toHaveLength(2);
    });

    it('renders with one url', () => {
      render(<WebsiteSection {...defaultPropsOneRepo} />);

      expect(screen.getByText('Website')).toBeInTheDocument();
      expect(screen.getByRole('link', { name: 'Website link' })).toBeInTheDocument();
      expect(screen.getByRole('link', { name: 'Website link' })).toHaveAttribute('href', 'https://artifacthub.io');
    });
  });
});
